use std::fmt::Display;

fn main() {
    let a = "hello";
    let b = "この國は";
    let ann = "ann...";
    let longest_word = longest_with_announcement(&a, &b, &ann);
    println!("longest word is: {}", &longest_word);
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
