struct Custom {
    name: String,
    age: usize,
}

enum Item {
    Number(usize),
    Custom(Custom),
    String(String),
}

fn main() {
    let foo = Item::Number(5);

    match &foo {
        Item::Number(num) => println!("i am a number: {}", num),
        Item::String(str) => println!("i am a string: {}", str),
        Item::Custom(custom) =>
            println!("name: {}, age: {}", custom.name, custom.age),
    }

    match &foo {
        Item::Custom(custom) =>
            println!("name: {}, age: {}", custom.name, custom.age),
        _ => {}
    }

    match &foo {
        Item::Custom(Custom {
            age,
            ..
        }) => println!("age: {}", age),
        _ => {}
    }

    match &foo {
        Item::Custom(custom) if custom.name == "Ricky" =>
            println!("Hi, Ricky"),
        Item::Custom(custom) if custom.age > 33 =>
            println!("N64 was the best console"),
        Item::Custom(custom) if custom.age < 30  =>
            println!("Xbox was the best console"),
        _ => {}
    }
}