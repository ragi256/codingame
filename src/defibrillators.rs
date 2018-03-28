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
    let lon = parse_input!(input_line.replace(",", "."), f32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = parse_input!(input_line.replace(",", "."), f32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut dist = std::f32::MAX;
    let mut result = String::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fields = input_line.trim_right().split(";").collect::<Vec<_>>();
        
        let name = format!("{}", fields[1]);
        let defib_lon = parse_input!(fields[4].replace(",", "."), f32);
        let defib_lat = parse_input!(fields[5].replace(",", "."), f32);
        let x = (lon - defib_lon) * ((lat + defib_lat) / 2.).cos();
        let y = lat - defib_lat;
        let d = (x.powi(2) + y.powi(2)).sqrt() * 6371.;

        if d < dist {
            dist = d;
            result = name;
        }
    }

    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("{}", result);
}
