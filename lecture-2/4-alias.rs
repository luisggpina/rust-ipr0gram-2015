struct Object {
    id: usize,
}

struct Pointer<'a,T> {
    to: &'a T,
}

fn compare<'c>(a: &'c Object, b: &'c Object) -> &'c Object {
    if a.id < b.id {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let the_object = Object { id: 1 };

    let mut res;
    {
        let other_object = Object { id: 2 };

        let ptr1 = Pointer<Object> { to: &the_object };

        res = compare(&the_object, &other_object);
    }


    let ptr2 = Pointer { to: &the_object };
}
