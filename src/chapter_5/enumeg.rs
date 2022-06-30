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