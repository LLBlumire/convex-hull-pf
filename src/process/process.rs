use process::error::Error;
use io::input::input::Input;
use io::output::output::Output;
use process::processor::Processor;

pub fn process(input: &Input) -> Result<Output, Error> {
    let processor = Processor::new(input);
    unimplemented!()
}
