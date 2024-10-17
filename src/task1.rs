#[test]

// src/lib.rs

// Оголошення модуля front_of_house
pub mod front_of_house {
    // Оголошення підмодуля hosting
    pub mod hosting {
        // Функція, що додає гостя до списку очікування
        pub fn add_to_waitlist() {
            // Реалізація функції
            println!("Adding to the waitlist.");
        }

        // Функція, що розсаджує за столом
        pub fn seat_at_table() {
            // Реалізація функції
            println!("Seating at the table.");
        }
    }

    // Оголошення підмодуля serving
    pub mod serving {
        // Функція, що приймає замовлення
        pub fn take_order() {
            // Реалізація функції
            println!("Taking the order.");
        }

        // Функція, що подає замовлення
        pub fn serve_order() {
            // Реалізація функції
            println!("Serving the order.");
        }

        // Функція, що приймає оплату
        pub fn take_payment() {
            // Реалізація функції
            println!("Taking the payment.");
        }

        // Функція, що обробляє скаргу
        pub fn complain() {
            // Реалізація функції
            println!("Handling a complaint.");
        }
    }
}

/*
Модуль front_of_house: Цей модуль є кореневим модулем для структури. Він оголошується за допомогою pub mod front_of_house { ... }, що робить його доступним ззовні.

Підмодуль hosting: Цей підмодуль містить дві функції: add_to_waitlist та seat_at_table, які реалізують логіку для управління списком очікування та розсаджуванням гостей.

Підмодуль serving: Цей підмодуль включає функції take_order, serve_order, take_payment та complain, що відповідають за обробку замовлень, їх подачу, приймання оплати та обробку скарг.
*/