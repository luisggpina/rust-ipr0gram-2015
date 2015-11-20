struct Object {
    id: usize,
}

struct Pointer {
    to: Object,
}

fn main() {
    let the_object = Object { id: 1 };

    let ptr1 = Pointer { to: the_object };

    let ptr2 = Pointer { to: &the_object };
}
