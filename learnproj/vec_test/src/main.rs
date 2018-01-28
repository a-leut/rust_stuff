fn main() {
    // Type annotation
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    // Inferred type w/ vec! macro
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    // Updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
    // Big zero vector
    let mut zero_vec = vec![0; 1000];
    println!("zero_vec.len(): {:?}", zero_vec.len());
    // Updating the zero vector
    for i in 1..999 {
        if i % 2 == 0 {
            zero_vec[i] = 1;
        }
    }
}
