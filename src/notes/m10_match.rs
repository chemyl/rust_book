use rand::Rng;

#[derive(Debug)]
enum MyEnum {
    First,
    Second,
    Third,
    Fourth,
}

fn match_simple_enum(x: MyEnum) {
    match x {
        MyEnum::First => println!("first {:?}", x),
        MyEnum::Second => println!("second {:?}", x),
        _ => println!("default matching arm{:?}", x), //default match
    }
}

#[derive(Debug)]
enum Fruits {
    Apple,
    Banana,
    Orange,
}

#[derive(Debug)]
enum Animals {
    Baboon,
    Lion,
    Crocodile,
}

/*
    Матчинг нескольких объектов одновременно
*/

fn match_complex_enum(a: &Fruits, b: &Animals) {
    match (a, b) {
        (Fruits::Apple, Animals::Baboon) => println!("Baboon and Apple! {:?}, {:?}", a, b),
        (Fruits::Orange, Animals::Lion) => println!("Orange and Lion! {:?}, {:?}", a, b),
        _ => println!("default matching arm {:?},{:?}",a,b),
    }
}

/*
    Матчинг чисел в диапазоне 
*/

fn match_simple_numbers(num: i32) {
    match num {
        1 => println!("{num}"),
        2 | 3 | 4 | 5 => println!("find in list {num}"),
        5..=19 => println!("find in range {num}"),
        _ => println!("default matching arm {num}"),
    }
}

/*
    Матчинг и внутренняя обработка
*/

fn match_pairs(pair:(i32,i32)){
    match pair{
        (x,y) if x==y => println!("numbers are equals {:?}", pair),
        (x,y) if x+y == 0 => println!("numbers sum equal zero{:?}", pair),
        (x, _) if x.pow(3) == 512  => println!("x in 3 power is 512! {:?}", pair),      // ignore 'y' value
        _ => println!("default matching arm {:?}", pair),
    }
}

/*
    Матчинг части tuple c игнорированием 
*/

fn match_tuple_partition(tuple: (i32,i32,i32)){
    match tuple{
        (0,y,z)=> println!("first item is 0  second= {y}, third= {z}"),
        (1,..)=>println!("fisrt item is 1 others ={:?}",tuple),     //ignore other but first
        _ => println!("default matching arm {:?}", tuple),
    }
}

/*
    Матчинг c вычислением выражения
*/

fn match_expression() {
    let boo = rand::thread_rng().gen_range(0..=100);
    let x = match boo {
        2 | 3 | 4 | 5 => true,
        10..=50 => false,
        _ => false,
    };
    println!("boo is {boo} and x is {x}");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_test() {
        match_complex_enum(&Fruits::Apple, &Animals::Baboon);
    }
    #[test]
    fn match_test2(){
        match_simple_numbers(2);
        match_simple_numbers(200);
        match_simple_numbers(8);
        match_simple_numbers(5);
        match_simple_numbers(19);
    }

    #[test]
    fn match_test3(){
        match_pairs((3,8));
        match_pairs((-2,2));
        match_pairs((8,0));
        match_pairs((0,0));
    }

    #[test]
    fn match_test4(){
        match_tuple_partition((5,6,7));
        match_tuple_partition((0,3,55));
        match_tuple_partition((8,4,7));
        match_tuple_partition((1,9,7));
    }

    #[test]
    fn match_test5(){
        match_expression();
        match_expression();
        match_expression();
    }

}
