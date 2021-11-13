use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Stuff {
    pub name: String,
    pub age: u8,
    pub god: bool,
    pub bananas: i32,
}

fn main() {
    let s = Stuff {
        name: "Mario".to_string(),
        age: 30,
        god: false,
        bananas: -123,
    };
    let a = bincode::serialize(&s).unwrap();

    let b: Stuff = bincode::deserialize(&a).unwrap();

    println!("{:?}", a);
    println!("{:?}", b == s);
    {
        let mut file = File::create("test.bin").unwrap();
        file.write_all(&a).unwrap();
    }

    {
        let mut file = File::open("test.bin").unwrap();
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        println!("{:?}", buffer);
        let b: Stuff = bincode::deserialize(&buffer).unwrap();
        println!("{:?}", b);
    }
}
