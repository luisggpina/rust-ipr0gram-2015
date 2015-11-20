enum Color {
    Black,
    White,
    Grey(u8),
    Red,
    Blue,
    Green,

    RGB(u8,u8,u8),
}

fn print_color(c: &Color) {
    match *c { // The *c is syntactic sugar for matching against &Color:: on each case
        Color::Black => println!("Black"),
        Color::White => println!("White"),
        Color::Grey(pc) if pc < 100 => println!("Grey {}%", pc),
        Color::Grey(_) => unreachable!(),

        Color::Red | Color::Green | Color:: Blue => println!("Red or green or blue"),

        Color::RGB(_,_,_) => println!("RGB"),
        // _ => println!("Other color"),
    }
}

enum OptionalColor {
    Some(Color),
    None,
}

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

fn nth_color(slice: &[Color], n: usize) -> Option<&Color> {
    if n < slice.len() {
        return Some(&slice[n]);
    } else {
        return None;
    }
}

fn main() {
    print_color(&Color::Black);
    print_color(&Color::White);
    print_color(&Color::Grey(42u8)); // Declaring the 42 literal as an unsigned byte
    print_color(&Color::Red);
    print_color(&Color::Blue);
    print_color(&Color::Green);

    let array : [Color;3] = [ Color::Black, Color::Grey(44), Color::Black ];
    let slice = &array[..];

    // Get the first argument passed on the command line
    let nth_argument : String = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("No argument");
            return;
        },
    };

    // Parse it as an usize, might fail, thus returns a Result<usize,_>
    let n : usize = match nth_argument.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argument is not a number");
            return;
        },
    };

    // Print the nth_color
    match nth_color(slice, n) {
        Some(c) => print_color(c),
        None => {
            println!("Out of bounds");
        },
    }
}
