#[cfg(test)]
mod test {
    #[test]
    fn go_long() {
        type I = i32;
        fn add1(x: i32) {
            println!("{}", x);
        }
        fn add2(x: I) {
            println!("{}", x);
        }
        let x = 1;
        add2(x);
        add1(x);
    }

    #[test]
    fn test_001() {
        fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
            move || a + b
        }

        let add_later = lazy_adder(1024, 2048);
        println!("{:?}", add_later());
    }


    #[test]
    fn console() {
        let mut array: [[i32; 5]; 6] = [[0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4]];

        for x in &mut array {
            for x in x {
                *x += 5
            }
        }
        println!("{:?}", array);
    }

    #[test]
    fn test_002() {
        let string1 = String::from("hello");
        let string2 = String::from("world");
        let boor1 = boor(string1.as_str(), string2.as_str());
        println!("{}", boor1);
    }

    fn boor<'a, 't>(x: &'a str, y: &'a str) -> &'t str
        where 'a: 't
    {
        return x;
    }
}