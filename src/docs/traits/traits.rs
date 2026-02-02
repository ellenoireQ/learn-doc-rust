use std::io;

#[allow(non_camel_case_types)]
pub enum Keyboard {
    Q,
    I,
    S,
    D,
}
pub trait Settings {
    fn new() -> Self;
    fn increment(&mut self);
    fn decrement(&mut self);

    fn show(&self);

    fn keyboard(&self) -> Keyboard;
}

struct Counter {
    counter: i32,
}

impl Settings for Counter {
    fn new() -> Self {
        Self { counter: 0 }
    }
    fn increment(&mut self) {
        self.counter += 1;

        println!("Increment +{}", self.counter);
    }
    fn decrement(&mut self) {
        self.counter -= 1;

        println!("Decrement -{}", self.counter);
    }

    fn show(&self) {
        println!("Counted +{}", self.counter)
    }

    fn keyboard(&self) -> Keyboard {
        let mut input_text = String::new();

        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        match input_text.as_str().trim() {
            "Q" | "q" => Keyboard::Q,
            "I" | "i" => Keyboard::I,
            "S" | "s" => Keyboard::S,
            "D" | "d" => Keyboard::D,
            _ => unimplemented!(),
        }
    }
}

pub fn test_traits() {
    let mut counter = Counter::new();
    loop {
        match counter.keyboard() {
            Keyboard::Q => break,
            Keyboard::I => counter.increment(),
            Keyboard::D => counter.decrement(),
            Keyboard::S => counter.show(),
        }
    }
}
