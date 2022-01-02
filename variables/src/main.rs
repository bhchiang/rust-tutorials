fn main() {
    // variables and mutability
    let mut x = 5;
    println!("value of x = {}", x);

    x = 6;
    println!("value of x = {}", x);

    //constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}
