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
    let message = input_line.trim_right().to_string();

    let bin = message
        .as_bytes()
        .into_iter()
        .map(|m| format!("{:07b}", m))
        .collect::<Vec<_>>()
        .concat()
        .chars()
        .collect::<Vec<char>>();
    print_err!("{:?}", bin);
    let mut count = 1;
    let len = bin.len();
    let mut result = String::new();
    for i in 1..len as usize {
        if bin[i] == bin[i - 1] {
            //if bin.nth(i).unwrap() == bin.nth(i - 1).unwrap() {
            count += 1;
        } else {
            let second_block = "0".repeat(count);
            //print_err!("{:?}", second_block);
            match bin[i - 1] {
                '0' => result.push_str(&format!("00 {} ", second_block)),
                _ => result.push_str(&format!("0 {} ", second_block)),
            }
            count = 1;
        }
    }
    let second_block = "0".repeat(count);
    match bin[len - 1] {
        '0' => result.push_str(&format!("00 {} ", second_block)),
        _ => result.push_str(&format!("0 {} ", second_block)),
    }

    println!("{}", result.trim_right());
}
// % 00100101
// 00 0 0 0 00 00 0 0 00 0 0 0
//      0 0 00 00 0 0 00 0 0 0
// Chuck Norris' keyboard 0 0 00 0000 0 0000 00 0 0 0 0
