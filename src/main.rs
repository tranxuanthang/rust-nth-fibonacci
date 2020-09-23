use std::io;

fn main() {
  let mut input: String = "".to_string();

  println!("Type a number:");

  io::stdin()
    .read_line(&mut input)
    .expect("Hmm, something is wrong :(");

  let input: i32 = input
    .trim()
    .parse()
    .expect("Something is wrong with your input!");

  println!("The {}th fibonacci number is: {}", input, fib(input));
}

fn fib(n: i32) -> i32 {
  if n < 2 {
    return n;
  }

  return fib(n - 1) + fib(n - 2);
}
