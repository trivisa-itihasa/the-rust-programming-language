use std::io;

fn main() {
  println!("Please input a non-negative integer.");

  let n: u32 = loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    match input.trim().parse() {
      Ok(num) => break num,
      Err(_) => {
        println!("Please input a non-negative integer.");
        continue;
      }
    };
  };

  let suffix = match (n % 100, n % 10) {
    (11..=13, _) => "th",
    (_, 1) => "st",
    (_, 2) => "nd",
    (_, 3) => "rd",
    _ => "th",
  };

  println!(
    "The {n}{suffix} number of Fibonacci sequence is {}.",
    fibonacci(n)
  );
}

fn fibonacci(n: u32) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}
