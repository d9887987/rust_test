//泛型结构体
//定义
//字段类型为Trait(特型)

// struct Name<T, U>
//     where
//         T: Trait1,
//         U: Trait2,
// {
//     name1: T,
//     name2: U,
// }


//空的特型，用于泛型约束
pub trait Body {}

pub trait Color {}

//创建泛型结构体
pub struct Vehicle<B, C>
    where
        B: Body,
        C: Color,
{
    body: B,
    color: C,
}

//泛型结构体的初始化方法
//泛型表示是特型实现
impl<B, C> Vehicle<B, C>
    where
        B: Body, C: Color
{
    pub fn new(b: B, c: C) -> Self {
        Self {
            body: b,
            color: c,
        }
    }
}

//结构体
pub struct Car;

//表示改结构体可用于泛型实现
impl Body for Car {}

pub struct Truck;

impl Body for Truck {}

pub struct Red;

impl Color for Red {}

pub struct Blue;

impl Color for Blue {}


pub fn structs() {
    let red_truck = Vehicle::new(Truck, Red);
    let car_truck = Vehicle::new(Car, Blue);
}








