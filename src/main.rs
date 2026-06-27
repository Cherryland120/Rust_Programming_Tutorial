mod gigasecond;
mod allintegers;
mod reverse_string;

fn main() {
    let ts = std::io::stdin();
    let mut inputted = "".to_string();
    let it = ts.read_line(&mut inputted).unwrap().to_string();
    println!("{}", reverse(&inputted));
}

fn reverse(input: &str) -> String{
    return input.chars().rev().collect::<String>();
}