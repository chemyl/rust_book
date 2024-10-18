/*
    Структура с дженериками
*/

#[derive(Debug)]
struct Example<T> {
    open_count: usize,
    data: Vec<T>,
}

/*
    Имплементация конструктора структуры с дженерикми
*/
impl<T> Example<T> {
    fn new() -> Self {
        Self {
            open_count: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, x: T) {
        self.open_count += 1;
        self.data.push(x);
    }
}

/*
    Реализация экземпляров структуры с дженериками
*/
fn generics_struct_instance() {
    let mut x = Example::<i32>::new(); //::<> - turbofish
    let z: Example<i32> = Example {
        open_count: 0,
        data: Vec::new(),
    };

    x.push(80);
    println!("instance x = {:?}", x);
    println!("instance z = {:?}", z);
}
