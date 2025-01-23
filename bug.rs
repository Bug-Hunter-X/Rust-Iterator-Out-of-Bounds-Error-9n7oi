fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap()); //Ok
    println!("Second element: {}", iter.next().unwrap()); //Ok
    println!("Third element: {}", iter.next().unwrap()); //Ok
    println!("Fourth element: {}", iter.next().unwrap()); //Error: 'None' value
}