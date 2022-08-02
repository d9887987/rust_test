#[cfg(test)]
mod tests {
    use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

    #[test]
    fn deque() {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);
        deque.push_front(4);
        deque.push_back(100);
        deque.push_back(200);
        deque.push_back(300);
        deque.push_back(400);


        for x in deque.iter() {
            println!("{}", x);
        }

        for x in deque.iter_mut() {
            *x += 15;
        }


        if let Some(x) = deque.front_mut() {
            *x += 50;
        }
        if let Some(x) = deque.back_mut() {
            *x += 50;
        }

        println!("{:?}", deque);

        if let Some(x) = deque.pop_front() {
            println!("{}", x);
        }

        if let Some(x) = deque.pop_back() {
            println!("{}", x);
        }

        if let Some(x) = deque.back() {
            println!("{}", x);
        }

        if let Some(x) = deque.front() {
            println!("{}", x);
        }
    }



    #[test]
    fn heap() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);
        heap.push(5);

        if let Some(x) = heap.peek() {
            println!("{}", x);
        }

        if let Some(x) = heap.pop() {
            println!("{}", x);
        }
    }

    #[test]
    fn hash_map(){
        let mut map = HashMap::new();
        map.insert(String::from("hello"),1);//插入，多次插入为更新
        map.entry(String::from("golang")).or_insert(2);//不存在则插入

        for (key, value) in &map {
            println!("{}-{}", key,value);
        }
    }

    #[test]
    fn num_hash(){
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let x = map.entry(word).or_insert(0);//返回对应value的一个可变引用
            *x+=1;
        }
        println!("{:?}", map);
    }

    #[test]
    fn btree_hash(){
        let text = "hello world wonderful world";
        let mut map = BTreeMap::new();
        for word in text.split_whitespace() {
            let x = map.entry(word).or_insert(0);//返回对应value的一个可变引用
            *x+=1;
        }
        println!("{:?}", map);
    }

    #[test]
    fn hash_set(){
        let text = "hello world wonderful world";
        let mut set = HashSet::new();
        for word in text.split_whitespace() {
            set.insert(word);
        }
        println!("{:?}", set);
    }

    #[test]
    fn BTree_set(){
        let text = "hello world wonderful world";
        let mut set = BTreeSet::new();
        for word in text.split_whitespace() {
            set.insert(word);
        }
        println!("{:?}", set);
    }

    #[test]
    fn owner_var(){
        let y;
        {
            let x=5;
            y=x;
        }
        println!("{}", y);
    }

  /*  #[test]
    fn owner_re(){
        let y;
        {
            let x=5;
            y=&x;
        }
        println!("{}", y);
    }*/

    #[test]
    fn longest_test(){
        let string1 = String::from("long string is long");//s1在整个方法中有效
        let result;
        {
            let string2 =String::from("xyz");//s2在当前作用域中有效
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("{}",result)
    }


    fn longest<'a>(x:&'a str,y:&str)->&'a str{
       x
    }

}
