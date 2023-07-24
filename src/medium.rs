use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut collected_data: Vec<Vec<String>> = Vec::new();
        let mut seen_before: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for str in strs.iter() {
            let mut char_count = [0; 26];

            // bytes - 'a' byte = char index from the alphabet list
            // based on index add 1 for each char.
            // if is anagram, another string will have the exact
            // same char count.
            // {char_count: [the string list that has same char count]}
            for &byte in str.as_bytes() {
                char_count[(byte - b'a') as usize] += 1;
            }

            seen_before
                .entry(char_count)
                .or_insert(Vec::new())
                .push(str.to_string());
        }

        for (_, val) in seen_before {
            collected_data.push(val);
        }

        collected_data
    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{HashMap, BinaryHeap};
        use std::cmp::Reverse;

        let mut data = HashMap::new();
        
        let mut collection = Vec::new();

        for i in nums {
            let count = data.entry(i).or_insert(0);
            *count += 1;
        }
        let mut min_heap = BinaryHeap::with_capacity(k as usize);
        
        for (index, amount) in data {
            min_heap.push(Reverse((amount, index)));

            // pop out the lowest element
            if min_heap.len() > k as usize{
                min_heap.pop();
            }
        }

        while let Some(Reverse((_, num))) = min_heap.pop() {
            collection.push(num);
        }
        
        collection
    }

    /// magic math algo solution
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1; n];
        
        let mut left_product = 1;
        for i in 0..n {
            output[i] *= left_product;
            left_product *= nums[i];
        }
        
        let mut right_product = 1;
        for i in (0..n).rev() {
            output[i] *= right_product;
            right_product *= nums[i];
        }
        
        output
    }
}
