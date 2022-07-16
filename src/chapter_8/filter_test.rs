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