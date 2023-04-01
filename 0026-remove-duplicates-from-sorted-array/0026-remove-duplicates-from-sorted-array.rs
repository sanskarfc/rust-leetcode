use std::collections::HashMap; 

impl Solution {
    pub fn remove_duplicates(v1: &mut Vec<i32>) -> i32 {
        let mut index1 = 1;
        let mut index2 = 1; 
        let mut map: HashMap<i32, bool> = HashMap::new(); 

        map.insert(v1[0], true);

        while index2 < v1.len() {
            if !map.contains_key(&v1[index2]) {
                map.insert(v1[index2], true);
                let temp = v1[index1];
                v1[index1] = v1[index2];
                v1[index2] = temp;
                index1 += 1;
                index2 += 1;
            }
            else {
                index2 += 1;
            }
        }

        return index1 as i32; 
    }
}