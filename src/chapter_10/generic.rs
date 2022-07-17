//泛型方法的定义
// pub fn generic<T,U>(param1:T,param2:U)
// where
// T,U,都是对应的特型的类型
//     T: Trait1,
//     U:Trait1+Trait2,
// {
//
// }


use std::fmt::Debug;

pub trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}
#[derive(Debug)]
pub struct Cargo {
    pub name: String,
}

#[derive(Debug)]
pub struct Rustc{
    pub name: String,
}

impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("hello:{}", self.name);
    }

    fn process(&self) {
        println!("我是测试的数据:{:?}", self.name);
    }
}

impl Default for Cargo{
    fn default() -> Self {
        Self{
            name:"cargo".to_string()
        }
    }
}

impl CheckIn for Rustc{
    fn check_in(&self) {
        println!("hello:{}", self.name);
    }

    fn process(&self) {
        println!("我是测试的数据:{:?}", self.name);
    }
}


impl Default for Rustc{
    fn default() -> Self {
        Self{
            name:"rust".to_string()
        }
    }
}

pub fn  process_item<T>(item:T)
where
    //泛型的类型只能是对应的trait类型的集合
    //架构体是不行的
    T:CheckIn+Debug,
{
    item.process();
    item.check_in()
}

