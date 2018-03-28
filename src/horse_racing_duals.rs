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
    let mut horses = Vec::new();
    let mut res = std::i32::MAX;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        horses.push(pi);
    }
    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");
    horses.sort();
    for i in 0..horses.len()-1 {
        res = (horses[i+1] - horses[i]).min(res);
    }
    //let res = horses.iter().map(|&x, &y| (x-y)).collect().min();
    
    println!("{}", res);
}