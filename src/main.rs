pub mod easy;
pub mod medium;

fn main() {
    let status = Solution::product_except_self(vec![1,2,3,4]);
    println!("{:?}", status);
}

pub struct Solution {}

impl Solution {
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
