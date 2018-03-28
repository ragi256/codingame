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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    if n == 0 {
        println!("{}", 0);
        return
    }
    let mut result: i32 = 5526;
    for i in inputs.split(" ") {
        let t = parse_input!(i, i32);
        result = if result.abs() > t.abs() {
                t 
            } else if result.abs() == t.abs() { 
                if result < t { t } else { result }
            } else {
                result
            };
    }

    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("{}", result);
}
