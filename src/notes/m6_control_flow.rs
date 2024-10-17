pub fn simply_loop(count: u32) {
    let mut inner_count = count;
    loop {
        println!("in loop num = {}", inner_count);
        if inner_count != 0 {
            inner_count = inner_count - 1
        } else {
            break;
        }
    }
}

pub fn complex_loop() {
    let mut count = 0;
    'count_up: loop {
        println!("this is count_up Loop. Count = {}", count);
        let mut remainig = 10;
        loop {
            println!("this is inner Loop Remaining = {}", remainig);
            if remainig == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remainig -= 1;
        }
        count += 1;
    }
    println!("this is the end. Count {}", count);
}

pub fn for_loop() {
    let arr: [u32; 5] = [2, 5, 7, 9, 66];
    for i in arr {
        println!("arr num {}", i);
    }
    for i in 1..5 {
        println!("arr num in range {}", i);
    }
}
