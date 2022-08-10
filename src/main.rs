// use crate::chapter_9::traits;
// use crate::traits::{Animals, Classification, inter};
// use crate::book_info::info_mation;
// use crate::book_info::service_info;
// use crate::user_info::user;
// use crate::user_info::service_go;
// use crate::chapter_2::guess_games;
// use crate::chapter_3::basic_grammar;
// use crate::chapter_4::ownership;
// use crate::chapter_5::enumeg;
// use crate::chapter_6::{my_hash};
// use crate::chapter_7::closure;
// use crate::chapter_8::{filter_test, while_let,iter};
// use crate::chapter_9::traits;
// use crate::traits::Animals;
// use crate::traits::Classification::Dog;

// use crate::chapter_9::default;
// use crate::default::Package;

// use crate::chapter_10::generic::{Cargo, process_item, Rustc};
// use crate::chapter_10::structs;

use crate::chapter_11::trait_object::test_trait;
use crate::chapter_15::variability;
use crate::chapter_16::rc;

// mod user_info;
// mod book_info;
// mod chapter_2;
// mod chapter_3;
// mod chapter_4;
// mod chapter_5;
// mod chapter_6;
// mod chapter_7;
// mod chapter_8;
// mod chapter_9;
// mod chapter_10;
mod chapter_11;
mod chapter_12;
mod chapter_13;
mod chapter_14;
mod chapter_15;
mod chapter_16;
mod chapter_17;
mod chapter_18;
mod chapter_19;

fn main() {
    //guess_games::guess_games_num();
    // basic_grammar::variable();
    // basic_grammar::loop_control_flow();
    // basic_grammar::function(1,3);
    // basic_grammar::for_control_flow();
    // basic_grammar::if_control_flow(25);
    // basic_grammar::while_control_flow(5);
    // let i = ownership::ownership_eg(String::from("hello world"));
    // println!("{}", i);
    // let string = String::from("hello world");
    // let slice1 = ownership::slice(&string[..]);
    // println!("{}", slice1);
    // let nums = vec![1, 2, 3, 4, 5];
    // let res = enumeg::two_sum(&nums, 4);
    // println!("{:?}", res);
    // println!("{:?}", nums);
    // my_hash::demo_hash();
    // let num = closure::sum(7, 8);
    // println!("{:?}", num);
    // filter_test::filter_up();
    // iter::iter_demo();
    // while_let::while_let();
    // let dog = traits::Car {
    //     class: Dog,
    //     name: "ba dou".to_string(),
    // };
    // inter(&traits::Dog { class: Classification::Dog, name: "test xx".to_string() });
    // dog.eat();
    // dog.call();
    // let package = Package::default();
    // println!("{:#?}", package);
    //
    // let x1 = Package::new(15);
    // println!("{:#?}", x1);
    // let cargo = Cargo::default();
    // process_item(cargo);
    // process_item(Rustc::default());
    // structs::structs()
    //test_trait()
    //variability::modify_internal_properties()
    //rc::rc_count()
}
