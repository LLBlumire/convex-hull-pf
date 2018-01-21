#[macro_use]
extern crate clap;

extern crate convex_hull_pf;
extern crate image;
extern crate serde_json;
extern crate toml;

use clap::{App, Arg};
use std::io::Read;
use std::fs::File;
use convex_hull_pf::io::input::Input;
use convex_hull_pf::io::output::Output;
use convex_hull_pf::process::process;
use std::io::Write;
use std::io::Cursor;
use image::ImageBuffer;
use image::GenericImage;
use image::png::PNGEncoder;
use image::RGB;
use image::Rgb;
use image::Pixel;

macro_rules! hard_crash {
    ($code:expr, $($arg:tt)*) => {{
        println!($($arg)*);
        $crate::std::process::exit($code);
    }}
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Finds a path along a route using a convex hull algorithm.")
        .arg(
            Arg::with_name("INPUT")
                .help("The input to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("The file to output to")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("output")
                .help("Specify the output mode, \"toml\" or \"json\" or \"png\"")
                .short("o")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output-scale")
                .help("Specify the output scale, only valid in \"png\" mode")
                .short("s")
                .takes_value(true),
        )
        .get_matches();

    // Unwrap is safe as CLAP handles requirement of value.
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let mode = matches.value_of("output").unwrap_or("toml");
    let scale: u32 = matches
        .value_of("output-scale")
        .unwrap_or("1")
        .parse()
        .unwrap_or(1);

    match File::open(input_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    let input = text_to_input(&buf, input_file);
                    let output = input_to_output(&input);
                    match File::create(output_file) {
                        Ok(mut file) => {
                            if let Err(e) = match mode {
                                "toml" => file.write(&output_to_toml(&output)),
                                "json" => file.write(&output_to_json(&output)),
                                "png" => file.write(&output_to_png(&output, scale)),
                                mode => hard_crash!(1, "Invalid output mode `{}`", mode),
                            } {
                                hard_crash!(1, "Error Writing to `{}` :: `{}`", output_file, e);
                            }
                            if let Err(e) = file.flush() {
                                hard_crash!(1, "Error Flushing `{}` :: `{}`", output_file, e);
                            }
                        }
                        Err(e) => {
                            hard_crash!(1, "Error Opening `{}` :: `{}`", output_file, e);
                        }
                    }
                }
                Err(e) => hard_crash!(1, "Error reading `{}` :: `{}`", input_file, e),
            }
        }
        Err(e) => {
            hard_crash!(1, "Error opening `{}` :: `{}`", input_file, e);
        }
    }
}

/// Processes the input text file, turning it into an input serial object.
fn text_to_input(input: &str, input_file: &str) -> Input {
    match toml::from_str(input) {
        Ok(input) => input,
        Err(e) => hard_crash!(1, "Error parsing `{}` :: `{}`", input_file, e),
    }
}

/// Processes the input, converting it to the output.
fn input_to_output(input: &Input) -> Output {
    process(input)
}

/// Converts the output to a toml binary encoded text format.
fn output_to_toml(output: &Output) -> Vec<u8> {
    format!("{}", toml::Value::try_from(output).unwrap()).into_bytes()
}

/// Converts the output to a json binary encoded text format.
fn output_to_json(output: &Output) -> Vec<u8> {
    serde_json::to_string(output).unwrap().into_bytes()
}

/// Converts the output to a png binary format.
fn output_to_png(output: &Output, scale: u32) -> Vec<u8> {
    let (x_size, y_size) = Some(output.input.start)
        .into_iter()
        .chain(Some(output.input.end).into_iter())
        .chain(output.input.route.clone().into_iter())
        .chain(
            output
                .input
                .polygons
                .clone()
                .into_iter()
                .flat_map(|polygon| polygon.points.into_iter()),
        )
        .fold((0, 0), |(mut max_x, mut max_y), coord| {
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
            (max_x, max_y)
        });

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(
        (x_size + 20) as u32,
        (y_size + 20) as u32,
        Rgb {
            data: [255, 255, 255],
        },
    );

    // Draw Polygons
    for segment in output
        .input
        .polygons
        .iter()
        .flat_map(|polygon| polygon.segments())
    {
        bresenham_line(
            segment.a.x,
            segment.a.y,
            segment.b.x,
            segment.b.y,
            &mut image,
            10,
            Rgb { data: [0, 0, 0] },
        );
    }

    // Draw Hulls
    for segment in output.hulls.iter().flat_map(|hull| hull.segment_set.iter()) {
        bresenham_line(
            segment.a.x,
            segment.a.y,
            segment.b.x,
            segment.b.y,
            &mut image,
            10,
            Rgb {
                data: [255, 0, 255],
            },
        );
    }

    // Draw Polypoints
    for point in output
        .input
        .polygons
        .iter()
        .flat_map(|polygon| polygon.points.iter())
    {
        image.put_pixel(
            point.x as u32 + 10,
            point.y as u32 + 10,
            Rgb { data: [0, 0, 255] },
        );
    }

    // Draw Route
    for point in &output.input.route {
        image.put_pixel(
            point.x as u32 + 10,
            point.y as u32 + 10,
            Rgb { data: [128, 0, 0] },
        )
    }

    // Draw Start
    image.put_pixel(
        output.input.start.x as u32 + 10,
        output.input.start.y as u32 + 10,
        Rgb { data: [0, 255, 0] },
    );

    // Draw End
    image.put_pixel(
        output.input.end.x as u32 + 10,
        output.input.end.y as u32 + 10,
        Rgb { data: [255, 0, 0] },
    );

    // Scale image up (nearest neighbour)
    let scaled_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(
        (x_size + 20) as u32 * scale,
        (y_size + 20) as u32 * scale,
        |x, y| *image.get_pixel(x / scale, y / scale),
    );

    // Return png data
    let mut buf = Vec::new();
    PNGEncoder::new(Cursor::new(&mut buf))
        .encode(
            &scaled_image.into_vec(),
            (x_size + 20) as u32 * scale,
            (y_size + 20) as u32 * scale,
            RGB(8),
        )
        .unwrap();
    buf
}

/// Standard Bresenham Line Algorithm
fn bresenham_line<G, P>(mut x0: i64, mut y0: i64, x1: i64, y1: i64, g: &mut G, pad: u32, color: P)
where
    G: GenericImage<Pixel = P>,
    P: Pixel,
{
    let dx = x1 - x0;
    let sx = dx.signum();
    let dx = dx.abs();

    let dy = y1 - y0;
    let sy = dy.signum();
    let dy = dy.abs();

    let mut err = if dx > dy { dx } else { -dy } / 2;

    let mut e2;

    loop {
        g.put_pixel(x0 as u32 + pad, y0 as u32 + pad, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}
