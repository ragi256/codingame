use std::io;

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
    let l = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_right().to_string().to_uppercase();
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right_matches('\n').to_string();
        for tt in t.as_bytes() {
            let a = 'A' as u8;
            let index = if tt >= &a { tt - &a } else { 255 };
            if index <= 25 {
                let (start, end) = (index as usize * l, (index + 1) as usize * l);
                print!("{}", &row[start..end]);
            } else {
                let (start, end) = (row.len() - l, row.len());
                print!("{}", &row[start..end]);
            }
        }
        println!();
    }
}
