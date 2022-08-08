#[cfg(test)]
mod tests {
    #[test]
    fn owner_ship() {
        let string1 = String::from("hello world");
        let word;
        {
            let string2 = String::from("java golang and rust");
            word = long_word(string1.as_str(), string2.as_str());
        }
        println!("{}", word);
    }

    fn long_word<'a>(p0: &'a str, p1: &str) -> &'a str {
        p0
    }
    /*#[test]
    fn return_str(){
        let str = get_str();
    }*/

    #[test]
    fn return_string() {
        let string2 = get_string();
    }

    fn get_string() -> String {
        String::from("hello rust")
    }

    /*fn get_str() -> &str {
        let string1 = String::from("hello rust");
        &string1
    }*/
}
