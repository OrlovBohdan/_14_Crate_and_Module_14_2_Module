#[test]

/*
// In lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // FILL in the blank in three ways
        //1. using keyword `super`
        //2. using absolute path
        __.serve_order();
    }

    fn cook_order() {}
}
*/

// src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to the waitlist.");
        }

        pub fn seat_at_table() {
            println!("Seating at the table.");
        }
    }

    pub mod serving {
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
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 1. Використання ключового слова `super`
        super::front_of_house::serving::serve_order();

        // 2. Використання абсолютного шляху
        crate::front_of_house::serving::serve_order();

        // 3. Відносний шлях
        front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}


/*
Використання super: super::front_of_house::serving::serve_order() - тут super посилається на батьківський модуль, тобто lib.rs, і далі ми вказуємо шлях до serve_order.

Використання абсолютного шляху: crate::front_of_house::serving::serve_order() - це повний шлях до функції, починаючи з кореня пакету.

Відносний шлях: front_of_house::serving::serve_order() - це прямий виклик, оскільки back_of_house і front_of_house знаходяться на одному рівні у структурі модулів.
*/