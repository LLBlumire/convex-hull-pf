#[macro_use]
extern crate clap;

extern crate convex_hull_pf;
extern crate toml;

use clap::{Arg, App};
use std::io::Read;
use std::fs::File;
use convex_hull_pf::io::input::input::Input;
use convex_hull_pf::io::output::output::Output;
use convex_hull_pf::process::process::process;

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
        .get_matches();

    // Unwrap is safe as CLAP handles requirement of value.
    let input_file = matches.value_of("INPUT").unwrap();

    match File::open(input_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => {
                    let input = text_to_input(&buf, input_file);
                    let output = input_to_output(&input, input_file);
                }
                Err(e) => hard_crash!(1, "Error reading `{}` :: `{}`", input_file, e),
            }
        }
        Err(e) => {
            hard_crash!(1, "Error opening `{}` :: `{}`", input_file, e);
        }
    }
}

fn text_to_input(input: &str, input_file: &str) -> Input {
    match toml::from_str(input) {
        Ok(input) => input,
        Err(e) => hard_crash!(1, "Error parsing `{}` :: `{}`", input_file, e),
    }
}

fn input_to_output(input: &Input, input_file: &str) -> Output {
    match process(input) {
        Ok(output) => output,
        Err(e) => hard_crash!(1, "Error processing `{}` :: `{}`", input_file, e),
    }
}

fn output_to_text(output: &Output, input_file: &str) -> String {
    unimplemented!()
}
