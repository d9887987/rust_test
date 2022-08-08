use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    height: f64,
    weight: f64,
}

impl Person {
    pub fn new(s: String, a: i32, h: f64, w: f64) -> Self {
        Self {
            name: s,
            age: a,
            height: h,
            weight: w,
        }
    }
}

type CellPerson=Cell<Person>;
pub fn modify_internal_properties(){
    let p1 = Person::new(String::from("deng"), 21, 175.0, 75.0);
    let person = CellPerson::new(p1);
    let person1 = Person::new(String::from("xiong"), 21, 175.0, 75.0);
    person.set(person1);
}

type ReCellPerson=RefCell<Person>;


