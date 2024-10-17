/*
Щоб викликати функції з бібліотеки з бінарного коду в файлі src/main.rs, потрібно спочатку оголосити відповідні функції в бібліотеці та потім правильно їх викликати.

потрібно визначити, які саме функції з бібліотеки слід викликати, щоб вони повертали значення, з якими ви будете порівнювати. Припустимо, ви хочете викликати функції seat_at_table і take_order з модуля front_of_house::hosting і front_of_house::serving.

Зміни у файлі src/lib.rs
Спочатку додати до файлу src/front_of_house/hosting.rs та src/front_of_house/serving.rs відповідні функції, які повертають значення.

src/front_of_house/hosting.rs:

pub fn add_to_waitlist() -> &'static str {
    println!("Adding to the waitlist.");
    "sit down please"
}

pub fn seat_at_table() {
    println!("Seating at the table.");
}


src/front_of_house/serving.rs:
pub fn take_order() -> &'static str {
    println!("Taking the order.");
    "yummy yummy!"
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

Тепер, у файлі src/main.rs, викличте ці функції, щоб заповнити пропуски:
src/main.rs:
fn main() {
    assert_eq!(hello_package::front_of_house::hosting::add_to_waitlist(), "sit down please");
    assert_eq!(hello_package::front_of_house::serving::take_order(), "yummy yummy!");

    println!("Success!");
}


hello_package::front_of_house::hosting::add_to_waitlist() — виклик функції add_to_waitlist, яка повертає рядок "sit down please".
hello_package::front_of_house::serving::take_order() — виклик функції take_order, яка повертає рядок "yummy yummy!".
*/