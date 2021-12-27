struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // find the shortest string
        let mut shortest = strs[0].clone();
        let mut shortest_len = shortest.len();
        for str in strs.iter().skip(1) {
            let len = str.len();

            if len < shortest_len {
                shortest = str.clone();
                shortest_len = len;
            }
        }

        let mut cur_prefix = shortest;
        while cur_prefix.len() > 0 {
            let mut stop = true;
            for str in strs.iter() {
                if !str.starts_with(&cur_prefix) {
                    cur_prefix = cur_prefix[..cur_prefix.len() - 1].to_string();
                    stop = false;
                    break;
                }
            }

            if stop {
                break;
            }
        }

        return cur_prefix.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::lc_14_longest_common_prefix::Solution;

    #[test]
    fn example1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(input), String::from("fl"));
    }

    #[test]
    fn example2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(input), String::from(""));
    }
}
