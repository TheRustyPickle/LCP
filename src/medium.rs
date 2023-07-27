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
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

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
            if min_heap.len() > k as usize {
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

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        // Each 3 rows contains 3 blocks
        let mut block_data: Vec<HashSet<&char>> = Vec::with_capacity(3);

        block_data.push(HashSet::new());
        block_data.push(HashSet::new());
        block_data.push(HashSet::new());

        let mut row_data: HashSet<&char> = HashSet::new();

        // We are looping by rows, so to check column
        // We need to save data on every column on every row
        let mut column_data: Vec<HashSet<&char>> = Vec::with_capacity(9);
        for _ in 0..9 {
            column_data.push(HashSet::new())
        }

        for row_num in 0..9 {
            for row_val in 0..9 {
                if board[row_num][row_val] == '.' {
                    continue;
                }
                // check if block is valid
                {
                    let block_index = row_val / 3;

                    if !block_data[block_index].insert(&board[row_num][row_val]) {
                        return false;
                    }
                }

                // check if the row is valid
                {
                    if !row_data.insert(&board[row_num][row_val]) {
                        return false;
                    }
                }

                // check if the column is valid
                {
                    if !column_data[row_val].insert(&board[row_num][row_val]) {
                        return false;
                    }
                }
            }

            // Clear block_data every 3 row
            if (row_num + 1) % 3 == 0 {
                for set in &mut block_data {
                    set.clear();
                }
            }

            // Looping through row so after each row, get it cleared
            row_data.clear();
        }

        true
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut max_length = 0;
        // hashset has better .contain() complexity than vector
        let data = nums.into_iter().collect::<HashSet<i32>>();

        for &num in &data {
            if !data.contains(&(num - 1)) {
                // no num -1 = start of the chain
                let mut current_length = 1;
                let mut current_num = num + 1;

                while data.contains(&current_num) {
                    current_num += 1;
                    current_length += 1;
                }

                if current_length > max_length {
                    max_length = current_length;
                }
            }
        }

        max_length
    }
}
