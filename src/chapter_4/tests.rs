use crate::chapter_4::ownership;

#[test]
fn test01() {
    let song = ownership::ownership_eg(String::from("hello rus"));
    println!("{}", song);
    let len=song;
    println!("{}", len);
    println!("{}", song);
}

#[test]
fn test02(){
    let s;
    {
        let slice1 = ownership::slice("hello");
        s=slice1;
        println!("{}", slice1);
    }
    println!("{}", s);
}
