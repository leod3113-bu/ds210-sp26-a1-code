// This will create a big vector and then return a pointer to it.
fn big_vector(value: i32) -> Vec<i32> {
    // We create a local vector v...
    let mut v = Vec::new();

    // Fill v with a million elements,
    for _ in 0..1000000 {
        v.push(value);
    }

    // Now we want to return a pointer to it to avoid copying all the elements, what could go wrong?
    return v;
}

fn main() {
    let my_vec = big_vector(10);
    let element_at_0 = my_vec[0];
    println!("{element_at_0}");
}
