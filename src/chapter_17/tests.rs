use std::ptr::addr_of_mut;

#[cfg(test)]
mod tests {
    use crate::chapter_17::tests::{combination_sum, combination_sum2, length_of_last_word, trap};

    #[test]
    fn test_001() {
        assert_eq!(4, 2 + 2)
    }

    #[test]
    fn length_of_last_word_test() {
        let string1 = String::from("hello world golang rust and java");
        let i = length_of_last_word(string1);
        println!("{}", i);
    }

    #[test]
    fn combination_sum_test() {
        let mut rec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 4;
        let sum = combination_sum(rec, target);
        println!("{:?}", sum);
    }

    #[test]
    fn trap_test() {
        let mut num = vec![5, 4, 5, 6, 3, 5, 7, 4, 9, 6];
        let trap1 = trap(num);
        println!("{}", trap1);
    }

    #[test]
    fn combination_sum2_test() {
        let nums = vec![5, 7, 3, 8, 9, 4, 2, 4];
        let target = 8;
        let num = combination_sum2(nums, target);
        println!("{:?}", num);
    }
}

//最后一个单词长度
fn length_of_last_word(s: String) -> i32 {
    if s.trim().len() == 0 {
        return 0;
    }
    s.split_whitespace().last().unwrap().len() as i32
}

//组合，不可重复
fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    fn recursion(candidates: &Vec<i32>, target: i32, index: usize) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![vec![]];
        }
        let mut ans = vec![];
        let mut push = 0;
        for can in candidates.iter().skip(index) {
            if *can > target {
                break;
            }
            let target = target - *can;
            let mut rec = recursion(candidates, target, index + push);
            for i in rec.iter_mut() {
                i.push(*can);
            }
            ans.append(&mut rec);
            push += 1;
        }
        ans
    }
    recursion(&candidates, target, 0)
}

//接雨水
fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let max = height.iter().enumerate().fold(0, |acc, (i, &x)| if height[acc] < x { i } else { acc });
    let mut left = 0;
    height[..max].iter().for_each(|&x| {
        if x < left {
            ans += left - x;
        } else {
            left = x;
        }
    });
    let mut right = 0;
    height[max..].iter().rev().for_each(|&x| {
        if x < right {
            ans += right - x;
        } else {
            right = x;
        }
    });
    ans
}

//组合可重复
fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();//方便剪枝
    let mut ans: Vec<Vec<i32>> = vec![];
    fn dfs(candidates: &Vec<i32>, target: i32, start_at: usize, acc: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ans.push(acc.to_owned());
        } else if 0 < target {
            for i in start_at..candidates.len() {
                let candidate = candidates[i];
                if i == start_at || candidate != candidates[i - 1] {
                    acc.push(candidate);
                    dfs(candidates, target - candidate, 1 + i, acc, ans);
                    //回溯
                    acc.pop();
                }
            }
        }
    }
    dfs(&candidates, target, 0, &mut vec![], &mut ans);
    ans
}


