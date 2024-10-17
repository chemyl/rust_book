/*
    То, что может быть в момент времени одним единственным вариантом из нескольких.
*/

#[derive(Debug)]
enum CarColor {
    Blue,
    Red,
    Green,
    Silver,
}

#[derive(Debug)]
enum Message {
    Quit,                       // пустой элемент без ассоциированных данных
    Move { x: i32, y: i32 },    // Имеет именованные поля как и структура
    Write(String),              // элемент с единственной строкой типа String
    ChangeColor(i32, i32, i32), // кортеж из 3х значений типа i32
}

/*
    Структура с полем типа Enum
*/
struct Letter {
    kind: Message,
    author: String,
}

//RESULT (Ok / Err)
#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

//OPTION (Some / None)
#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

/*
    Создание варианта перечисления
*/
fn handle_car_color_silver() -> CarColor {
    CarColor::Silver
}

/*
    Функция принимает любой из вариантов Enum
*/
fn handle_enum_message(msg: Message) {
    println!("This-is message enum = {:?}", msg)
}
fn test_handle() {
    handle_enum_message(Message::Quit);
    handle_enum_message(Message::Move { x: 5, y: 34 });
    handle_enum_message(Message::Write(String::from("Hello")));
    handle_enum_message(Message::ChangeColor(6, 4, 3));
}

/*
    Result by ENUM Version
*/
fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not good".to_string())
    }
}

/*
    Result by std
*/
fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not good".to_string())
    }
}

/*
    в этом примере обобщенный типом может быть f32
*/
fn remainder_zero(num: f32) -> GivenOption<f32> {
    let remainder: f32 = num % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

/*
    Option from std
*/
fn remainder_zero_built_in(num: f32) -> Option<f32> {
    let remainder: f32 = num % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit!"),
        Message::Move { x, y } => println!("Moving to coordinates: {}, {}", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to: {}, {}, {}", r, g, b),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn enums_test() {
        let car_color: CarColor = handle_car_color_silver();
        dbg!(car_color);

        let res_under_five: GivenResult<u8, String> = check_under_five(4);
        dbg!(res_under_five);

        let res_under_five: Result<u8, String> = check_under_five_built_in(4);
        dbg!(res_under_five);

        let res_above_five: GivenResult<u8, String> = check_under_five(7);
        dbg!(res_above_five);

        let res_above_five: Result<u8, String> = check_under_five_built_in(7);
        dbg!(res_above_five);

        let remainder: GivenOption<f32> = remainder_zero(12.2);
        dbg!(remainder);

        let remainder: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(remainder);

        let msg: Message = Message::Write("write something".to_string());
        process_message(msg);

        let change_color: Message = Message::ChangeColor(4, 2, 5);
        process_message(change_color);

        test_handle();
    }
}
