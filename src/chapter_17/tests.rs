use std::cmp;
use std::ptr::addr_of_mut;

#[cfg(test)]
mod tests {
    use crate::chapter_17::tests::{combination_sum, combination_sum2, jump2, length_of_last_word, permute, trap};

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

    #[test]
    fn permute_test() {
        let nums = vec![5, 7, 3, 8];
        let num = permute(nums);
        println!("{:?}", num);
        println!("{}", num.len())
    }

    #[test]
    fn jump_test() {
        let nums = vec![2, 3, 4, 5, 1, 1, 1, 1, 1, 1, 2];
        let i = jump2(nums);
        println!("{}", i);
    }
}

//最后一个单词长度
fn length_of_last_word(s: String) -> i32 {
    if s.trim().len() == 0 {
        return 0;
    }
    s.split_whitespace().last().unwrap().len() as i32
}

//组合，可重复
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

//组合不可重复
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

//排列组合
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(first: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            ans.push(nums.to_vec());
            return;
        }
        for i in first..nums.len() {
            nums.swap(first, i);
            dfs(first + 1, nums, ans);
            nums.swap(first, i);
        }
    }
    let mut ans = Vec::new();
    let mut nums = nums;
    dfs(0, &mut nums, &mut ans);
    ans
}

//跳跃游戏2
fn jump2(nums: Vec<i32>) -> i32 {
    let mut steps = 0 as i32;
    let mut end = 0 as i32;
    let mut max = 0 as i32;

    for x in 0..nums.len() - 1 {
        let max = cmp::max(max, nums[x] + (x as i32));
        if (x as i32) == end {
            end = max;
            steps += 1;
        }
    }
    steps
}


pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() <= 1 {
        return vec![nums];
    }
    nums.sort_unstable();
    let mut stack = vec![];
    let mut ans = vec![];
    let mut used = vec![false; nums.len()];
    fn dfs(nums: &Vec<i32>, depth: usize, stack: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
        if stack.len() == nums.len() || depth == nums.len() {
            ans.push(stack.to_owned());
            return;
        }
        for n in 0..nums.len() {
            if (n > 0 && nums[n] == nums[n - 1] && !used[n - 1]) || used[n] {
                continue;
            }
            stack.push(nums[n]);
            used[n] = true;
            dfs(nums, depth + 1, stack, ans, used);
            stack.pop();
            used[n] = false;
        }
    }
    dfs(&nums, 0, &mut stack, &mut ans, &mut used);
    ans
}



