/*
    если успешно, то это вернется с этим типом
   если это отсутствует, что ничего не будет возвращаено
*/
enum OPTION<T> {
    Some(T),
    None,
}

/*
   для обработки более сереьзных ошибок, с выводом текста
*/
enum Rusult<T, E> {
    Ok(T),
    Err(E),
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by Zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error_handling_option_test() {
        assert_eq!(Some(2.5), divide_option(5.0, 2.0));
        assert_eq!(None, divide_option(5.0, 0.0));

        let res = divide_option(6.0, 2.0);
        match res {
            Some(x) => println!("Result is {}", x),
            None => println!("Cannot divide by Zero"),
        }
    }
    #[test]
    fn error_handling_result_test() {
        match divide_result(8.0, 2.0) {
            Ok(result) => println!("Divide result is {}", result),
            Err(err) => println!("result is {}", err),
        }
        match divide_result(8.0, 0.0) {
            Ok(result) => println!("Divide result is {}", result),
            Err(err) => println!("{}", err),
        }
    }
}
