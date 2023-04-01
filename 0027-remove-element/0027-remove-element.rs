impl Solution {
    pub fn remove_element(v1: &mut Vec<i32>, val: i32) -> i32 {
        if v1.len() == 0 {
            return 0;
        }
        
        let mut index1 = v1.len() - 1;
        let mut index2 = v1.len() - 1; 

        while index2 >= 0 {
            if v1[index2] == val {
                v1[index2] = v1[index1];
                index1 -= 1;
            }
            if index2 == 0 {
                return (index1 + 1) as i32;
            }
            index2 -= 1;
        }

        return (index1 + 1) as i32;
    }
}