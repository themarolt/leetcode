struct Solution {}

impl Solution {
    fn get_closing(open: char) -> char {
        return match open {
            '{' => '}',
            '(' => ')',
            '[' => ']',
            _ => panic!("incorect open parentheses"),
        };
    }

    fn is_open(c: char) -> bool {
        return match c {
            '{' => true,
            '[' => true,
            '(' => true,
            _ => false,
        };
    }

    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut stack: Vec<char> = vec![];

        let first = s.chars().nth(0).unwrap();
        if !Self::is_open(first) {
            return false;
        }

        stack.push(first);

        for ch in s.chars().skip(1) {
            if Self::is_open(ch) {
                stack.push(ch);
            } else {
                if let Some(last) = stack.last() {
                    if Self::get_closing(*last) == ch {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        return stack.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::lc_20_valid_parentheses::Solution;

    #[test]
    fn example1() {
        assert!(Solution::is_valid(String::from("()")));
    }

    #[test]
    fn example2() {
        assert!(Solution::is_valid(String::from("()[]{}")));
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_valid(String::from("(]")));
    }
}
