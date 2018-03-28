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
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
 
fn compare(a: i32, b: i32) -> usize {
    if a == b {
        1
    } else if a < b {
        0
    } else {
        2
    }
}
 
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position
    let mut tx = initial_tx;
    let mut ty = initial_ty;
    const direction: [[&str; 3]; 3]  = [["NW","W","SW"], ["N","","S"], ["NE", "E", "SE"]];
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.
        
        let dx = compare(light_x, tx);
        let dy = compare(light_y, ty);
        let next = direction[dx][dy];
        tx = tx + dx as i32 - 1;
        ty = ty + dy as i32 - 1;
        
        print_err!("{} {} {}", next, tx, ty);
        
        println!("{}", next);
    }
}