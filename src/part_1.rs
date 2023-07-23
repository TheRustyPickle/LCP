use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut set = HashSet::with_capacity(nums.len());

        for i in &nums {
            let status = set.insert(i);
            if status == false {
                return true;
            }
        }
        false
    }

    pub fn two_sum(my_vals: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i_index = 0;
        let mut u_index = 0;
        let mut final_vec = Vec::new();
        'outer: for i in my_vals.iter() {
            for u in my_vals.iter() {
                if i + u == target && i_index != u_index {
                    final_vec.push(i_index);
                    final_vec.push(u_index);
                    break 'outer;
                }
                u_index += 1;
            }
            u_index = 0;
            i_index += 1;
        }
        final_vec
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = vec![0; 26];

        for i in 0..s.len() {
            char_count[(s.as_bytes()[i] - b'a') as usize] += 1;
            char_count[(t.as_bytes()[i] - b'a') as usize] -= 1;
        }

        !char_count.iter().any(|i| *i != 0)
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut collected_data: Vec<Vec<String>> = Vec::new();
        let mut seen_before: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for str in strs.iter() {
            let mut char_count = [0; 26];
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
}
