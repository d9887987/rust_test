use std::collections::HashMap;

pub fn demo_hash(){
    let mut map = HashMap::new();
    map.insert("java",1);
    map.insert("golang",2);
    map.insert("rust",3);
    for (key, value) in map {
        println!("key:{:?},value:{:?}", key,value);
    }
}