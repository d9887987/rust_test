use std::sync::{Arc, Mutex};
use std::thread;

pub fn thread_demo(){
    let nums = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut child = vec![];
    for n in 0..nums.len()  {
        let num = Arc::clone(&nums);
        let i = thread::spawn(move || {
            println!("{}", num[n]);
        });
        child.push(i);
    }
    for x in child {
        x.join().unwrap();
    }
}

pub fn mutex_demo(){
    let vec = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0..5 {
        let num = vec.clone();
        let t = thread::spawn(move || {
            let mut num = num.lock().unwrap();
            num.push(i);
        });
        childs.push(t);
    }
    for x in childs {
        x.join().unwrap();
    }
    println!("{:?}", vec);
}

pub fn update_demo(){

    let mut  nums = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    let mut vec=vec![];
    for i in 0..5 {
        let num = nums.clone();
        let handle = thread::spawn(move || {
            let mut guard = num.lock().unwrap();
            guard[i]=guard[i]+1;
        });
        vec.push(handle);
    }
    for x in vec {
        x.join().unwrap();
    }
    println!("{:?}", nums);
}