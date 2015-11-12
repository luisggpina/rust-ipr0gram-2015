// Factorial

fn fact(mut n: u64) -> u64 {

    let mut res : u64 = 1;

    while n!=0 {
        res = res * n;
        n = n - 1;
    }

    return res;
}

fn main() -> () {
    for i in 0..10 {
        println!("fact({}) = {}", i, fact(i));
    }
}
