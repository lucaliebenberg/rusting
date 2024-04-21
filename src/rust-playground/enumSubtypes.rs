struct Custom {
    age: usize,
    name: String
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom)
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("hello, Fem!".to_string())); // to_string()
}

fn main() {
    let mut items = vec![];
    append(&mut items);
}