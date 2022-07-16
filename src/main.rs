use crate::chapter_8::filter_test;

// mod user_info;
// mod book_info;
// mod chapter_2;
// mod chapter_3;
// mod chapter_4;
// mod chapter_5;
// mod chapter_6;
// mod chapter_7;
mod chapter_8;

// use crate::book_info::info_mation;
// use crate::book_info::service_info;
// use crate::user_info::user;
// use crate::user_info::service_go;
// use crate::chapter_2::guess_games;
// use crate::chapter_3::basic_grammar;
// use crate::chapter_4::ownership;
// use crate::chapter_5::enumeg;
//use crate::chapter_6::{my_hash};
// use crate::chapter_7::closure;


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
    filter_test::filter_up();
    filter_test::iter_demo();
}