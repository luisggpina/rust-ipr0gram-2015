// Arrays

fn get(array: [u32; 3], i: usize) -> u32 {
    array[i]
}

fn main() {
    let array : [u32; 3] = [ 1 , 2 , 3 ] ;

    println!("{}", get(array, 2));
}
