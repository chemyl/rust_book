mod m0_types;
mod m1_enums;
mod m2_stuct;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetime;
mod m6_control_flow;
mod m7_error_handling;
mod m8_collections;

use m0_types::*;
use m6_control_flow::*;

fn main() {
    println!("{}", TIME);

    fn minimum(arr: &[i32]) -> i32 {
        *arr.iter().min().unwrap()
    }

    fn maximum(arr: &[i32]) -> i32 {
        *arr.iter().max().unwrap()
    }

    simply_loop(6);
    complex_loop();
    for_loop();
    my_variables();
}
