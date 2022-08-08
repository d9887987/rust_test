use std::rc::Rc;
use std::sync::Arc;
use std::thread;

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

type RcPerson=Rc<Person>;

type ArcPerson=Arc<Person>;

pub fn rc_count(){
    //单线程共享引用，不转移所有权
    let p1 = Person::new(String::from("deng"), 21, 175.0, 75.0);
    let rc = RcPerson::new(p1);
    let rc1 = rc.clone();
    let rc2 = rc.clone();
    let rc3 = rc.clone();
    println!("{:?}",rc3);
}

pub fn arc_count(){
    let p1 = Person::new(String::from("deng"), 21, 175.0, 75.0);

}