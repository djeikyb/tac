use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut lines: Vec<String> = vec![];
    let mut reader = BufReader::new(io::stdin());

    loop {
        let mut ln = String::new();
        let len = reader.read_line(&mut ln);
        match len {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                lines.push(ln);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    for ln in lines.iter().rev() {
        print!("{}", ln)
    }

    Ok(())
}
