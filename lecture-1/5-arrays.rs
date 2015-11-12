// Arrays

fn get(array: &[u32], i: usize) -> u32 {
    array[i]
}

fn main() {
    let array : [u32; 3] = [ 1 , 2 , 3 ] ;
    let slice : &[u32] = &array[0..2];

    println!("{}", get(slice, 2));
}
