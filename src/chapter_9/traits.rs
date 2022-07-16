#[derive(Debug)]
enum Classification {
    Dog,
    Person,
    Car,
}


#[derive(Debug)]
pub struct Dog {
    pub class: Classification,
    pub name: String,
}

#[derive(Debug)]
pub struct Person {
    pub class: Classification,
    pub name: String,
}

#[derive(Debug)]
pub struct Car {
    pub class: Classification,
    pub name: String,
}


pub trait Animals {
    fn eat(&self);
    fn call(&self);
}

impl Animals for Dog {
    fn eat(&self) {
        println!("{:?}", self.class)
    }

    fn call(&self) {
        println!("{:?}", self.name)
    }
}

impl Animals for Person {
    fn eat(&self) {
        println!("{:?}", self.class)
    }

    fn call(&self) {
        println!("{:?}", self.name)
    }
}

impl Animals for Car {
    fn eat(&self) {
        println!("{:?}", self.class)
    }

    fn call(&self) {
        println!("{:?}", self.name)
    }
}