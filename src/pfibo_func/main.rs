use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    println!("Enter number of Phibonacci. I will give you back it's fractal number");
    
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: i32 = match user_input.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };

    println!("{}-n Phibonacci number: {}", user_input, 
    fibonacci(user_input.try_into().unwrap()));
}