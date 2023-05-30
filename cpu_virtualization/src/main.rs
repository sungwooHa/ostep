use std::time::Duration;
use std::{env, process::exit};
use std::io::{self, Write};
use std::{thread, time};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        // io::stderr().write_all(b"hello world")?;
        println!("usage : cpu <string>");
        exit(1);
    }

    let str = &args[1];
    while(true) {
        thread::sleep(Duration::from_secs(1));
        println!("{:?}",str);
    }

    Ok(())
}
