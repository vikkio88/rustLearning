fn main() {
    for n in 0..10 {
        println!("stuff: {}", n + 1);
    }

    let mut v = Vec::new();

    v.push(3);
    v.push(3);
    v.push(5);
    v.push(1000);

    println!("***********************\n\n");
    for (i, e) in v.iter().enumerate() {
        println!("elem: {} - {}", i, e);
    }
}
