use std::collections::HashMap;

/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    for (i, num1) in nums[..n - 1].iter().enumerate() {
        for (j, num2) in nums[i + 1..n].iter().enumerate() {
            if num1 + num2 > target {
                continue;
            }
            let start = i as i32;
            let end = 1 + start + j as i32;
            if num1 + num2 == target {
                return vec![start, end];
            }
        }
    }
    vec![]
}

/// The brute force solution is O(n^2).
/// To get O(n), use a first pass to build a hashmap and records {complement: index}.
/// Then for each element, check if it exists in the complement map.
fn two_sum_smart(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements = HashMap::<i32, i32>::new();
    for (i, num) in nums.iter().enumerate() {
        let other = target - *num;
        match complements.get(&other) {
            Some(j) => {
                return vec![i as i32, *j as i32];
            }
            None => {
                complements.insert(*num, i as i32);
            }
        }
    }

    vec![]
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //two_sum_brute(nums, target)
    two_sum_smart(nums, target)
}

/// Order does not matter
fn check_answer(ans: Vec<i32>, expected: Vec<i32>) -> bool {
    if ans.len() != expected.len() {
        return false;
    }
    let mut ans = ans;
    let mut expected = expected;
    ans.sort();
    expected.sort();
    for i in 0..ans.len() {
        if ans[i] != expected[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = [0, 1];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }
    #[test]
    fn test_ex_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = [1, 2];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }
    #[test]
    fn test_ex_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = [0, 1];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }

    #[test]
    fn test_ex_4() {
        let nums = vec![3, 2, 3];
        let target = 6;
        let expected = [0, 2];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }

    #[test]
    fn test_ex_5() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let expected = [0, 3];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }

    #[test]
    fn test_ex_6() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let expected = [2, 4];
        assert!(check_answer(two_sum(nums, target), expected.to_vec()));
    }
}
