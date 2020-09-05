// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

pub mod delicious_snacks {
    pub use self::fruits::APPLE as fruit;
    pub use self::veggies::CARROT as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
