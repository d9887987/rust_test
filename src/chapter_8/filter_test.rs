pub fn filter_up() {
    let mut num = Vec::new();
    num.push(1);
    num.push(2);
    num.push(3);
    num.push(4);
    num.push(5);
    println!("{:?}", num);

    let x: Vec<i32> = num.iter().map(|num| num + 1).collect();
    println!("{:?}", x);

    let x1: Vec<&i32> = x.iter().filter(|num| num == &&2).collect();
    println!("{:?}", x1);

    let numbers: Vec<_> = num.iter().filter(|num| num >= &&2).collect();
    println!("{:?}", numbers);

    let find = num.iter().find(|num| num == &&3);

    match find {
        None => {
            println!("不存在这样的值")
        }
        Some(num) => {
            println!("需要查找的值是：{:?}", num);
        }
    }
    println!("{:?}", find);
}

pub fn iter_demo() {
    let mut number = Vec::new();
    number.push(1);
    number.push(3);
    number.push(5);
    number.push(7);
    number.push(8);

    /*
    使用闭包时注意参数的借用关系
    */

    let data: Vec<_> = number.iter()
        .map(|num| num * &3)
        .filter(|num| num > &10)
        .collect();
    println!("结果值:{:?}", data);

    for num in data {
        println!("{}", num);
    }
}

