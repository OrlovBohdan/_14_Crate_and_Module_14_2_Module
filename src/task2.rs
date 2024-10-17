#[test]

/*
// In lib.rs

// FILL in the blanks and FIX the errors
// You need to make something public with `pub` to provide accessibility for outside code `fn eat_at_restaurant()`
mod front_of_house {
    /* ...snip... */
}

pub fn eat_at_restaurant() {
    // Call add_to_waitlist with **absolute path**:
    __.add_to_waitlist();

    // Call with **relative path**
     __.add_to_waitlist();
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

// Функція, що викликає add_to_waitlist
pub fn eat_at_restaurant() {
    // Виклик add_to_waitlist з **абсолютним шляхом**:
    crate::front_of_house::hosting::add_to_waitlist();

    // Виклик з **відносним шляхом**
    front_of_house::hosting::add_to_waitlist();
}

/*
Абсолютний шлях: crate::front_of_house::hosting::add_to_waitlist() - Це повний шлях до функції, починаючи з кореня вашого пакету (crate).

Відносний шлях: front_of_house::hosting::add_to_waitlist() - Оскільки eat_at_restaurant та front_of_house знаходяться на одному рівні в структурі, ви можете просто вказати відносний шлях.
*/