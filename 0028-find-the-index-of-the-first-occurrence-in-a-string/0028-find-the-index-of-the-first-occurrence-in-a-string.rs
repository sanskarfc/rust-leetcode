impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        for i in 0..=(haystack.len() - needle.len()) {
            if &haystack[i..(i+needle.len())] == &needle {
                return i as i32;
            }
        } 
        return -1;
    }
}