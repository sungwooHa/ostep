use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        panic!("file name please");
    }

    let word = args[1].clone();
    args.remove(0);
    args.remove(0);

    for file_name in args {

        println!("file name : {}", file_name.as_str());
        let file = File::open(file_name.as_str())?;

        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            match line {
                Ok(line) => {
                    if line.contains(&word) {
                        println!("{}", line);
                    }
                }
                Err(_) => {
                    println!("not matched")
                }
            }
        }
    }

    Ok(())
}
