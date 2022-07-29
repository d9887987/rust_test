

pub fn variable(){
    let x=5;
    println!("x:{}", x);
    let mut y=6.75;
    println!("y:{}", y);
    y=3.14;
    println!("y:{}", y);
    let x=x+5;
    println!("x+5:{}", x);
    println!("Rust 变量默认不可变，使用mut关键字可以改变，最后变量隐藏，类型推断时Rust的特色");

    let a :[i32;5]=[1,2,3,4,5];

    let nums=[1,2,3,4,5];
    for x in nums.iter() {
        println!("数组:{}", x);
    }
    for x in nums {
        println!("数组:{}", x);
    }
    let tup=(1,2.1,5);
    println!("元组tup.0:{}", tup.0);

}

pub fn function(a :i32,b: i32)->i32{
    return a+b
}

pub fn data_type(){

}

pub fn method(){

}

pub fn loop_control_flow(){
    let mut x=1;
    loop {
        if x==10{
            println!("{}", x);
            break;
        }
        x+=1;
    }
}

pub fn if_control_flow(i: i32){
    if i>=5{
        println!("hello");
    }else {
        println!("no hello");
    }
}

pub fn for_control_flow(){
    let nums=[1,2,3,4,5];
    for x in nums.iter() {
        println!("数组:{}", x);
    }
}

pub fn while_control_flow(mut i:i32){
    while i < 10 {
        println!("{}", i);
        i=i+1;
    }
}

