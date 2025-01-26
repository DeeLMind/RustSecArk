///
/// https://leetcode.cn/problems/two-sum/description/
/// 

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            
            // 检查 complement 是否已存在于哈希表中
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            
            // 将当前数字及其索引存入哈希表
            map.insert(num, i);
        }
        
        vec![] // 如果没有找到答案，返回空数组（题目中保证一定有答案）
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // 输出：[0, 1]
}

