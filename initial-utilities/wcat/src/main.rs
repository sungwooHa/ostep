use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("file name please");
    }
    
    args.remove(0);

    for file_name in args {
        let mut file = File::open(file_name.as_str())?;
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        let contents = str::from_utf8(&data).unwrap(); 
        println!("{}", contents);
    }

    Ok(())
}
