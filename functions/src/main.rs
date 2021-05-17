use rand::Rng;

fn main() {
    println!("Starting up another_function(x: i32)");
    let rand_num = rand::thread_rng().gen_range(1..101);
    another_function(rand_num);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
