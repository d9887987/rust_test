enum Classification {
    Dog,
    Person,
    Car,
}


pub struct Animal {
    class: Classification,
    name: String,
}


pub trait Animals {
    fn eat(&self);
    fn call(&self);
}


impl Animals for Animal {
    fn eat(&self) {
        todo!()
    }

    fn call(&self) {
        todo!()
    }
}