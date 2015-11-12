// Doesn't compile

// fn main() {
//     let s = "Hello".to_string();
// 
//     let a1 = s;
//     let a2 = s;
// 
// }

// Compiles but looks bad
// fn print(arg: String) -> String {
// 
//     println!("{}", arg);
//     return arg;
// }
//
// fn main() {
//     let mut s = "Hello".to_string();
// 
//     s = print(s);
//     print(s);
// 
// }

// With borrowing

fn print(arg: &String) {
    println!("{}", arg);
}

fn main() {
    let s = "Hello".to_string();

    print(&s);
    print(&s);

}
