pub const TIME: i32 = 2;

#[allow(dead_code, unused_variables)]
pub fn my_variables() {
    let a: u32 = 55;
    let b: i32 = -90;
    let c: f64 = -5555.12;
    let d: char = '⚠';
    let dd: bool = true;
    let dd: bool = false; //shadowing

    /*
        Strings
        &str
    */
    let e: &str = "String"; // строковый литерал нельзя изменять
    let mut f: String = String::from("Hello!"); // создать String из строкового литерала "Hello!" ::from()
    f.push('🦀');
    f.push_str(" Rust");
    println!("Concated String = {}", f);

    let g: String = String::new();
    println!("empty string object = {}", g);

    {
        let b = "james"; // visible only in the scope
    }
    // b is no longer visible

    /*
        arrays
    */
    let arr: [u32; 10] = [10, 234, 55, 234, 6, 7, 88, 3, 2, 1];
    let fruits: [&str; 3] = ["apple", "banana", "pineaple"];
    println!("array by index = {}", arr[5]);
    println!("arr max index by iter = {}", *arr.iter().max().unwrap());

    /*
        tulps
    */
    let tul: (&str, u32, f64) = ("hello", 24, 12.8);
    let (x, y, z) = tul;
    println!("destructurized tulp = {},{},{}", x, y, z);
    println!("tul by index = {}", tul.2);
    let tul_2 = ("james", 55, 1.1, ["alex", "melman"], "phoenix");
}
