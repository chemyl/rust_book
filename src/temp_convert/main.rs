use std::io;

fn main() {
    println!("== welcome to the term converter == ");

    loop {
        println!("Please enter Celsius temperature");

        let mut user_temp = String::new();
        io::stdin()
            .read_line(&mut user_temp)
            .expect("Failed to read line");

        let user_temp: f32 = match user_temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let far_temp: f32 = user_temp * 1.8 + 32.0;

        println!("You °C tem {:?} means {:?} in °F units", user_temp, far_temp);
        println!("Cool awesome stuff!");
        break;
    }
}
