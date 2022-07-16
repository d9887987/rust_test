pub fn sum(a: i32, b: i32) -> i32 {
    /*
    闭包函数的作用在于捕捉上下文变量
    */

    //与golang中的闭包函数类似

    /*var add =func(a,b int) int{
        return a+b
    }*/


    //Rust闭包函数
    /*

    这儿的入参类型也可忽略
    忽略标准在于编译器是否可以推断出变量类型

    let add = |a i32, b i32| -> i32{
        a + b
    };
    */


    let add = |a, b| -> i32{ a + b };
    add(a, b)
}