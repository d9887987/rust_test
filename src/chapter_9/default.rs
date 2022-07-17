
#[derive(Debug)]
pub struct Package{
   pub weight:i32
}

impl Package{
    pub fn new(weight: i32)->Self{
        Self{
            weight
        }
    }
}


// Default 是一个默认的trait
// 有sdk内置，当数据类型有默认实现时，实现这个接口
impl Default for Package{
    fn default() -> Self {
        Self{
            weight:30
        }
    }
}