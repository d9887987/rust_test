use std::collections::HashMap;
use crate::ownership::slice;

pub enum Relation{
    Book(String),
    Info(String),
    Common(String)
}

pub fn test_add(){
    let x=1;
    let y=2;
    {
        let i = add(&x, &y);
        println!("{}", i);
    }
    println!("{}", x);
    println!("{}", y);
}

pub fn add(a:&i32,b :&i32) ->i32{
    (*a)+(*b)
}

pub fn add01(a:i32,b:i32)->i32{
    a+b
}

pub fn str_add(s:String,s1:String)->String{
    s1
}

pub fn string01(){
    let mut x="gave".to_string();
    let y=&x;
    let j=&x;
    println!("{}", y);
    println!("{}", x);
    println!("{}", j);
}

pub fn hash(){
    let mut hash = HashMap::new();
    hash.insert("java".to_string(),1);
    hash.insert("golang".to_string(),2);
    hash.insert("python".to_string(),1);
    hash.insert("rust".to_string(),3);

    for x in hash.iter() {
        println!("{:?}", x);
    }
    println!("{:?}", hash);
}
