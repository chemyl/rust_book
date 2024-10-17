#[allow(dead_code, unused_variables)] // выключить подчеркивание неиспользованного кода
fn example_01() {
    // allocate memory
    let highest_age: u32;

    // init vars
    let bob_age: u32 = 10;
    let alice_age: u32 = 21;

    // call function
    highest_age = largest(&bob_age, &alice_age);

    //print resukt
    print!("Hihest age is {}", highest_age);

    /*
        expected `u32`, found `&u32`
        - чтобы исправить - добавить разыменование *, тогда будет не ссылка в значение.
    */
    fn largest(compare_1: &u32, compare_2: &u32) -> u32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }

    /*
        если возвращать ссылку
        Тип возвращаемый функцией содержит заимстсованный тип (ссылку),
        в сигнатуре метода необходимо указать откуда оно заимствовано (compare_1 или compare_2)

        'a все всходящие ссылки одного срока жизни
        fn largest_v2<'a>(compare_1: &'a u32, compare_2: &'a u32) -> &'a u32
        срок жизни вывода будет таким же как и самый короткий срок из переданных аргументов

        <'a, 'b>
        <'a, 'b:'a> - два срока жизни, но апостроф 'b живет столько же сколько и апостроф 'a

    */
    fn largest_v2<'a, 'b: 'a>(compare_1: &'a u32, compare_2: &'b u32) -> &'a u32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

// Lifetime with Generics

#[allow(dead_code, unused_variables)]
fn example_02() {
    // allocate memory
    let highest_age: &u32;

    // init vars
    let bob_age: u32 = 10;
    let alice_age: u32 = 21;

    // ::<u32> - явно указать, какой тип будет передаваться в функцию с дженериком
    highest_age = largest_v2::<u32>(&bob_age, &alice_age);

    //print resukt
    print!("Hihest age is {}", highest_age);

    /*
        добваить дженерик в сигнурату функции.
            - Чтобы к типу дженерика могли применяться операции сравнения, ему надо добавить признак PartialOrd
        изменить параметры на дженерики
        изменить тип возвращаемого значения на дженерик

    */
    fn largest_v2<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
