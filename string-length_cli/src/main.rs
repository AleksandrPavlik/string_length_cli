#[warn(unused_parens)]
use std::io;
fn main() {
    let mut text = String::new();
    println!("Введите текст: ");

    io::stdin().read_line(&mut text).expect("Не удалось считать строку");

    let res = calculate_length(&text);

    println!("Длина {} равна {} (- 1)", text, res);
}

fn calculate_length(text: &String) -> usize {

    text.len()
}