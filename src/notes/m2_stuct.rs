#[derive(Debug)] // добавить, чтобы включить отладку для экземпляров структуры. Чтобы работал типаж Debug
struct User {
    name: String,
    age: u8,
    email: String,
    active: bool,
}

/*
использование форматирования по типажу debug :?. Для вывода сложных структур.
типаж Display для примитивных типов. - они по умолчанию реализуют этот типаж.
*/
fn struct_print(user: &User) {
    println!("{:?}", user);
}

/*
    кортежные структуры
*/
struct Color(u32, f64, u32);
fn tulp_ctruct() {
    let black = Color(1, 2.0, 3);
    println!("this is black{}", black.1);
}

/*
        Методы похожи на функции.
        Они определяются в констексте структуры и их первым параметром всегда является self,
        представляющий собой экземпляр структуры с которой вызывается этот метод.
        Self - псевдоним типа, для которого реализован блок impl
*/

impl User {
    fn age_increment(&mut self) {
        // &mut self - если метод должен изменить экземпляр структуры
        self.age += 1
    }

    fn get_user_activ(&self) -> bool {
        // &self - если значение экземпляра только для чтения
        self.active
    }

    fn change_user_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    fn compare_two_users(&self, other_user: &User) -> bool {
        self.age == other_user.age
    }
}

/*
    У структуры может быть несколько блоков impl
*/
impl User {
    fn email(&self) -> String {
        //метод с названием как поле структуры и простым возвратом значения - геттер
        self.email.to_string()
    }

    /*
        Ассоциированная функция - им не нужен экземпляр self для работы.
        Ассоциированные функции не являются методами, часто используются для констуркторов.
        Возвращают Self - псевдоним типа, для которого реализован блок impl
    */
    fn generate(active: bool) -> Self {
        Self {
            name: (String::from("Jack")),
            age: (11),
            email: (String::from("email.com")),
            active, // сокращенный вариант, если имея аргумента = имени поля. Вместо active:active
        }
    }
}

fn change_user_name(user: &mut User, new_user_name: &str) {
    user.name = String::from(new_user_name);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_test() {
        dbg!("Unit test in Struct");

        /*
            Изменяемый экземпляр пользовательской структуры User
        */
        let mut user_1 = User {
            name: String::from("Tolik"),
            email: String::from("1@mail.com"),
            age: 50,
            active: true,
        };

        user_1.active = false;
        assert_eq!(user_1.get_user_activ(), false);
        change_user_name(&mut user_1, "Mike");

        let mut user_2: User = User {
            age: user_1.age,
            active: user_1.active,
            email: String::from("2@mail.com"),
            name: String::from("Kate"),
        };

        /*
            синтаксис обновления структуры. Если оставшиеся поля структуры-донора не реализуют Copy trait, то донор больше недействителен.
        */
        let user_3: User = User {
            email: String::from("2@mail.com"),
            name: String::from("Kate"),
            ..user_2
        };

        /*
            значения полей типа стринг были перенесены в новую структуру из user_3, теперь больше нельзя использовать user_3
        */
        let user_4: User = User {
            age: user_1.age,
            ..user_3
        };

        user_2.change_user_email("newEmail.com");
        user_1.age_increment();

        struct_print(&user_1);
        struct_print(&user_2);
        struct_print(&user_4);
        println!("this is email getter method = {}", user_1.email());
        println!("comparison = {}", user_4.compare_two_users(&user_2));
        println!("stuct constructor {:?}", User::generate(false)); // вызов конструктора через ассоциированную функцию

        dbg!(user_1);
        dbg!(user_2);
    }
}
