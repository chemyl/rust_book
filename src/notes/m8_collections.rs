use std::collections::{hash_map, HashMap};

/*
    Vectors Vec<T>
        однородного типа, может быть расширяемым
*/
/*
    1. новый пустой вектор с ячейками типа i32
    2. проинициализированный вектор
*/
fn new_vec() -> Vec<u16> {
    let v: Vec<i32> = Vec::new();
    let v2: Vec<i8> = vec![1, 2, 3];

    let mut v3: Vec<u16> = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_test() {
        println!("this is empty vector - {:?}", new_vec());
    }

    /*
        получить элемент из вектора по индексу - &u16 = &vec[2]
    */
    #[test]
    fn vector_test2() {
        let some_vec_item: &u16 = &new_vec()[new_vec().len() - 2];
        println!("this is index 2 vetor item = {}", some_vec_item);
    }

    /*
        получить элемент из вектора через get(index)
    */
    #[test]
    fn vector_test3() {
        let full_vec = new_vec();
        let some_vec_item = full_vec.get(4);
        match some_vec_item {
            Some(x) => println!("this is Some value of the .get(3) index {x}"),
            None => println!("There is no value in index"),
        }
    }

    #[test]
    fn string_test() {
        string_builder();
    }

    #[test]
    fn map_test() {
        map_builder();
        map_iterator();
    }
}

/*
UTF8
    Строка - это цепочка символов.
    Строки в Rust обеспечивают автоматическую безопасную кодировку utf8
*/
fn string_builder() {
    let s = "Hello!";
    let s2 = String::from("Здравствуйте!");
    let s3: String = "Hello ".to_string();
    let mut s4 = String::from("World ");
    s4.push('R');
    s4.push_str("ust");

    let s5 = s3 + &s4; // s3 был перенесен и больше не может использоваться

    println!("{s4}");
    println!("{s5}");
}

/*
    Hash Map <K, V>
    использование требует стандартной библиотеки - use std::collections::HashMap;
*/

fn map_builder() {
    let mut scores = HashMap::new();
    scores.insert("jack", 15);
    scores.insert("tomas", 17);
    scores.insert("antony", 11);
    scores.insert("smith", 10);

    println!("{:?}", scores.get("tomas").unwrap());
    let antony_score = scores.get("antony").unwrap(); // unwrap to - i32 else - Option

    println!("Antony score is = {antony_score}");
}

fn map_iterator() {
    let mut items = HashMap::new();
    items.insert(String::from("Yiesk"), 55);
    items.insert(String::from("Taganrog"), 90);
    items.insert(String::from("Moscow"), 11);
    items.insert(String::from("Kazan"), 23);
    items.insert(String::from("St.Pitersburg"), 4);
    items.insert(String::from("Volgograd"), 3456);
    items.insert(String::from("Murmansk"), 22);

    for (key, value) in &items {
        println!("item key - {key}, and item value - {value}");
    }
    if items.contains_key("Moscow") {
        println!("There is Mocsow in the keys list!");
    }
}
