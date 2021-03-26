use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    for item in 1..=n {
        println!("{}", fizz_buzz(item));
    }
}

fn fizz_buzz(n: i32) -> String {
    let result = match n {
        n if n % 15 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => n.to_string()
    };
    result
}