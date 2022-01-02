fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner scope x = {}", x);
    }
    println!("outer scope x = {}", x);

    let spaces = "    ";
    // spaces = spaces.len();
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
}
