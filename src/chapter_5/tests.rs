use crate::chapter_5::enumeg;

#[test]
fn test01(){
    let nums =vec![0,1,2,3,4,5,6,7,8];
    let sum = enumeg::two_sum(&nums, 9);
    println!("{:?}", sum);
}