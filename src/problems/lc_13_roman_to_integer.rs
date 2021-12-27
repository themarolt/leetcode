struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let len = s.len();

        let chars: Vec<char> = s.chars().collect();
        let mut skip = false;
        for i in 0..len {
            if skip {
                skip = false;
                continue;
            }

            let cur = chars[i];

            if i != len - 1 && (cur == 'I' || cur == 'X' || cur == 'C') {
                let next_char = chars[i + 1];

                if cur == 'I' && next_char == 'V' {
                    res += 4;
                    skip = true;
                    continue;
                } else if cur == 'I' && next_char == 'X' {
                    res += 9;
                    skip = true;
                    continue;
                } else if cur == 'X' && next_char == 'L' {
                    res += 40;
                    skip = true;
                    continue;
                } else if cur == 'X' && next_char == 'C' {
                    res += 90;
                    skip = true;
                    continue;
                } else if cur == 'C' && next_char == 'D' {
                    res += 400;
                    skip = true;
                    continue;
                } else if cur == 'C' && next_char == 'M' {
                    res += 900;
                    skip = true;
                    continue;
                }
            }

            let val = match cur {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("should not happen"),
            };

            res += val;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::lc_13_roman_to_integer::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
