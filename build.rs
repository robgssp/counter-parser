extern crate lalrpop;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from the build script!");
    //lalrpop::process_root()?;
    lalrpop::Configuration::new()
        .process_current_dir()?;
    return Ok(());
}
