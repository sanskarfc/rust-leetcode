impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if nums[i] == target {
                return i as i32;
            }
        } 

        let mut i = 0;
        while i < nums.len() {
            if nums[i] > target {
                return i as i32;
            }
            i += 1;
        }

        return nums.len() as i32;
    }
}