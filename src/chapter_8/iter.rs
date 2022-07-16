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