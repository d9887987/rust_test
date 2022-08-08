struct Container<'a> {
    length: &'a str,
    width: f64,
    height: f64,
}

// impl<'t> Container<'t> {
//     fn test<'a>(&self, s: &'a str)->&'a str {
//         return String::from("hello").as_str();
//     }
// }