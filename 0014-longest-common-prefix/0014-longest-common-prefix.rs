impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        if strs.len() == 0 {
            return prefix;
        }
        let mut i = 0;
        loop {
            let mut c = ' ';
            for s in strs.iter() {
                if i >= s.len() {
                    return prefix;
                }
                if c == ' ' {
                    c = s.chars().nth(i).unwrap();
                } else if c != s.chars().nth(i).unwrap() {
                    return prefix;
                }
            }
            prefix.push(c);
            i += 1;
        }
    }
}