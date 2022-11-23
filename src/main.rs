// Alexandre CAILLIEZ
// Version 1.0.0
use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

//    let width = user_input_to_int("Rectangle's width: ");
//    let height = user_input_to_int("Rectangle's height: ");

    let rect1 = Rectangle {
        width: user_input_to_int("Rectangle's width: "),
        height: user_input_to_int("Rectangle's height: "),
    };
    println!(
        "The rectangle has an area of {} square units.",
        rectangle_area(rect1)
    );
}

fn rectangle_area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn user_input_to_int(text_to_display: &str) -> u32 {
    println!("{}", text_to_display);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input.");

    let parsed_input: u32 = user_input.trim().parse().expect("Failed to convert to int");
    parsed_input
}
