use rand::Rng; // not sure how traits work
use std::cmp::Ordering;
use std::io;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // let mut guess = 1;
    // guess = 2;
    // println!("guess = {}", guess)

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret # = {}", secret_number);

    println!("guess the #");
    println!("input your guess");

    let std = io::stdin();

    loop {
        let mut guess = String::new();
        println!("before `{}`", guess);
        print_type_of(&guess);

        std.read_line(&mut guess).expect("failed to read line");

        println!("after `{}`", guess);
        print_type_of(&guess);

        // variable shadowing, infer int type for .parse()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("you guessed {}", guess);
        print_type_of(&guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win :D");
                break;
            }
        }
    }
}
