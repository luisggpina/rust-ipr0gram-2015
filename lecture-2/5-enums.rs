// This does not compile, can you fix it?
// File 6-enums.rs compiles (and expands this example)

enum Color {
    Black,
    White,
    Grey(u8),
    Red,
    Blue,
    Green,

    RGB(u8,u8,u8),
}

fn print_color(c: Color) {
    match c {
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

fn nth_color(slice: &[Color], n: usize) -> Option<Color> {
    if n < slice.len() {
        return Some(slice[n]);
    } else {
        return None;
    }
}

fn main() {
    print_color(Color::Black);
    print_color(Color::White);
    print_color(Color::Grey(42));
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Green);

    let array : [Color;1] = [ Color::Black ];
    let slice = &array[..];

    match nth_color(slice, 2) {
        Some(c) => print_color(c),
        None => (),
    }
}
