use std::env;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    
    let args: Vec<String> = env::args().collect();
    
    
    let file_name = if args.len() > 1 {
        &args[1]
    } else {
        "default.txt"
    };

    
    File::create(file_name)?;
    
    println!("File Created: {}", file_name);
    
    Ok(())
}
