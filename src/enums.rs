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