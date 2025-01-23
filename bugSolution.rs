fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    if let Some(first) = iter.next() {
        println!("First element: {}", first);
    }
    if let Some(second) = iter.next() {
        println!("Second element: {}", second);
    }
    if let Some(third) = iter.next() {
        println!("Third element: {}", third);
    }
    if let None = iter.next(){
        println!("Fourth element: Does not exist");
    }
} 