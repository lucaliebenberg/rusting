enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) ->  bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Red => println!("red")
    }
}

fn main() {
    // let foo = Color::Green;
}














/* BASIC READ FILE IMPLEMENTATION */
// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();
//
//     file
//         .lines()
//         .enumerate()// enumerate creates a tuple
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_, line)| println!("{}", line));
// }


/* BASIC MAPPING IMPLEMENTATION */
// fn main() {
//     let data = vec![1,2,3];
//     let mut iter = data
//         .iter()
//         .map(|x| x + 1);
//
//     let mut new_vector = vec![];
//
//     while let Some(x) = iter.next() {
//         new_vector.push(x);
//     }
//
//     println!("{:?}", new_vector);
// }