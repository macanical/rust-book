fn main() {
    const MAX_POINTS: u32 = 100_000; // underscores can be used to make numbers more readable
    println!("The value of MAX_POINTS: {}", MAX_POINTS);
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1; // This is called shadowing using the let keyword again
    println!("The value of x is: {}", x);
    let x = x * 2; // This is called shadowing using the let keyword again
    println!("The value of x is: {}", x);

}
