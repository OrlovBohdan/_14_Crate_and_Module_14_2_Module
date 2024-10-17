/*
1. Cargo.toml
Створіть файл Cargo.toml з таким вмістом:

toml
Копировать код
[package]
name = "hello-package"
version = "0.1.0"
edition = "2021"

[lib]
name = "hello_package"
path = "src/lib.rs"
2. src/lib.rs
Створіть файл lib.rs з таким вмістом:

mod back_of_house;
mod front_of_house;

pub fn eat_at_restaurant() {
    // Виклик add_to_waitlist з абсолютним шляхом
    front_of_house::hosting::add_to_waitlist();

    // Виклик add_to_waitlist з відносним шляхом
    front_of_house::hosting::add_to_waitlist();
}

3. src/back_of_house.rs
Створіть файл back_of_house.rs з таким вмістом:

use crate::front_of_house::serving;

fn fix_incorrect_order() {
    cook_order();
    // 1. Використання ключового слова `super`
    serving::serve_order();

    // 2. Використання абсолютного шляху
    crate::front_of_house::serving::serve_order();

    // 3. Відносний шлях
    front_of_house::serving::serve_order();
}

fn cook_order() {}
4. src/front_of_house/mod.rs
Створіть файл mod.rs в каталозі front_of_house з таким вмістом:

pub mod hosting;
pub mod serving;
5. src/front_of_house/hosting.rs
Створіть файл hosting.rs з таким вмістом:

pub fn add_to_waitlist() {
    println!("Adding to the waitlist.");
}

pub fn seat_at_table() {
    println!("Seating at the table.");
}
6. src/front_of_house/serving.rs
Створіть файл serving.rs з таким вмістом:

pub fn take_order() {
    println!("Taking the order.");
}

pub fn serve_order() {
    println!("Serving the order.");
}

pub fn take_payment() {
    println!("Taking the payment.");
}

pub fn complain() {
    println!("Handling a complaint.");
}
7. src/main.rs
Створіть файл main.rs з таким вмістом:

fn main() {
    hello_package::eat_at_restaurant();
    println!("Success!");
}


Cargo.toml: Це файл конфігурації проекту.
src/lib.rs: Корінь бібліотеки, де оголошуються модулі та функції, доступні зовні.
src/back_of_house.rs: Модуль, який реалізує логіку, пов'язану з кухнею та замовленнями.
src/front_of_house/mod.rs: Головний модуль для фронт-офісу, що імплементує hosting та serving.
src/front_of_house/hosting.rs: Модуль, що містить функції для управління чергою.
src/front_of_house/serving.rs: Модуль, що містить функції для обслуговування замовлень.
src/main.rs: Основна точка входу програми, яка викликає функцію eat_at_restaurant.



*/