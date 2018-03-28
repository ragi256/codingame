use std::io;
use std::collections::HashMap;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize); // Number of elements which make up the association table.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q: usize = parse_input!(input_line, usize); // Number Q of file names to be analyzed.

    let mut mime = HashMap::new();

    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let ext = inputs[0].trim().to_string().to_lowercase();
        let mt = inputs[1].trim().to_string();
        mime.insert(ext, mt);
    }
    for _ in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        if input_line.contains(".") {
            let inputs = input_line.split(".").collect::<Vec<_>>();
            let ext = inputs.last().unwrap().trim().to_lowercase();
            match mime.get(&ext) {
                Some(mt) => println!("{}", mt),
                _ => println!("UNKNOWN"),
            }
        } else {
            println!("UNKNOWN");
        }
    }
}