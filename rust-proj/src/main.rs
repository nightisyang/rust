fn only_evens(x: usize) -> bool {
    if x % 2 == 1 {
        unreachable!("this should never happen")
    }

    todo!("hey, me, finish this later")
}

fn main() {
    let foo = None;

    let bar: i32 = foo.unwrap();
}
