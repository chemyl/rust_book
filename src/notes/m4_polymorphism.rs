trait DoSomethig {
    fn wake_up(&self);
}

struct Human {
    sex: String,
    name: String,
}

struct Animal {
    sex: String,
    animl_type: String,
}

impl DoSomethig for Human {
    fn wake_up(&self) {
        println!("Human waked up")
    }
}

impl DoSomethig for Animal {
    fn wake_up(&self) {
        println!("Animal waked Up")
    }
}

fn this_is_morning_for<T: DoSomethig>(doer: T) {
    doer.wake_up();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn poly_test() {
        let work_man: Human = Human {
            name: String::from("Peter"),
            sex: String::from("male"),
        };
        let tropical_animal: Animal = Animal {
            animl_type: String::from("mammal"),
            sex: String::from("male"),
        };

        this_is_morning_for(work_man);
        this_is_morning_for(tropical_animal);
    }
}

// Сделать признак. Определить функцию
// Сделать имплементацию признака для нескольких типов. Для каждого отдельно

// Сделать функцию, которая принимает обобщенный тип Т, который реализует признак.
// Дженерик должен использовать определенную черту.

// fn get_different_inputs<T: TraitName>(value: T) -> Something {
//     value.implemented_trait_function()
// }

// Теперь в функцию мжно передавать несколько различных типов, для которых есть имплементация trait функции
