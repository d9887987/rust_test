pub fn while_let(){
    let mut num = Vec::new();
    num.push(1);
    num.push(2);
    num.push(3);
    num.push(4);
    num.push(5);

    let mut num_iter = num.iter();

    while let Some(num) = num_iter.next() {
        println!("{:?}", num);
    }
    println!("done");
}