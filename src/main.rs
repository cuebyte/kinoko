pub mod capillary;

fn main() {
    let x = Solution::find_duplicates(vec![10,2,5,10,9,1,1,4,3,7]);
    println!("{:?}", x);
}
struct Solution();
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        
        for i in 0..nums.len() {
            let x = nums[i].abs() as usize - 1;
            if nums[x] < 0 {
                result.push(x as i32 + 1);
            } else {
                nums[x] = -nums[x];
            }
        }
        result
    }
}