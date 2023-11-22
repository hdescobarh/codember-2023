/*
** The Spy Encryption Challenge **
A group of spies has discovered that their message encryption system is compromised.

They have found some passwords that do not comply with theEncryption Security Policy that was established when they were created.

To solve the problem, they have created a list (your entry to the challenge) of passwords (according to the corrupted database) and the security policy when that key was established.

Example of the list:

2-4 f: fgff
4-6 z: zzzsg
1-6 h: hhhhhh
Each line indicates, separated by :, the key policy and the key itself.

The key policy specifies the minimum and maximum number of times a given character must appear for the key to be valid. For example, 2-4 f means that the key must contain f at least 2 times and at most 4 times.

Knowing this, in the previous example, there are 2 valid keys but one is not:

The second key, zzzsg, is not valid; it contains the letter z 3 times, but needs at least 4. The first and third keys are valid: they contain the appropriate amount of f and h, respectively, according to their policies.

** Your challenge: **
Determine how many encryption keys are valid according to their policies.

** How to solve it **
1. Analyze the list of policies and encryption keys that you will find in this file: https://codember.dev/data/encryption_policies.txt

2. Create a program that returns the 42nd invalid key (of all the invalid keys, the 42nd in order of appearance). For example:
submit bqamidgewtbuz
*/
pub fn validate_policies(policies_content: &str) -> Vec<&str> {
    let mut invalid_keys = Vec::new();
    for line in policies_content.lines() {
        let (min, max, character, key) = parse_line_policy(line);
        if !check_policy_compliancy(&min, &max, &character, key) {
            invalid_keys.push(key)
        }
    }
    invalid_keys
}

fn check_policy_compliancy(min: &usize, max: &usize, character: &char, key: &str) -> bool {
    let repeats_number = key.chars().filter(|c| *c == *character).count();
    (*min <= repeats_number) && (repeats_number <= *max)
}

// Takes a line from the policies file and returns the min, max repeats, the character and the key
fn parse_line_policy(line: &str) -> (usize, usize, char, &str) {
    // each valid line is of the form: ^[0-9]+-[0-9]+\s[a-z]:\s[a-z]+
    let line_words: Vec<&str> = line.split_ascii_whitespace().collect();
    if line_words.len() != 3 {
        panic!("Invalid line formatting!")
    };

    // the interval must be numbers. The first number must be lower or equal than the second
    let interval: Vec<usize> = line_words[0]
        .split('-')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    if interval.len() != 2 || (interval[0] > interval[1]) {
        panic!("Invalid key policy formatting!")
    }

    // the next value must be a valid Unicode code point
    let character: char = line_words[1]
        .trim_end_matches(':')
        .parse()
        .expect("Invalid character policy formatting!");

    // the last value correspond to the key and must have at least one element
    let key: &str = line_words[2];
    if key.is_empty() {
        panic!("Invalid key formatting!.")
    }
    (interval[0], interval[1], character, key)
}

#[cfg(test)]
mod test {
    use crate::challenge03::{check_policy_compliancy, parse_line_policy};

    #[test]
    fn parse_works_as_expected() {
        let test_cases = [
            ("2-4 f: fgff", (2, 4, 'f', "fgff")),
            ("4-6 z: zzzsg", (4, 6, 'z', "zzzsg")),
            ("1-6 h: hhhhhh", (1, 6, 'h', "hhhhhh")),
        ];
        for (case, expected) in test_cases {
            assert_eq!(expected, parse_line_policy(case))
        }
    }

    #[test]
    fn evaluates_correctly_policies() {
        let test_cases = [
            (2, 4, 'f', "fgff", true),
            (4, 6, 'z', "zzzsg", false),
            (1, 6, 'h', "hhhhhh", true),
        ];
        for (min, max, character, key, expected) in test_cases {
            assert_eq!(
                expected,
                check_policy_compliancy(&min, &max, &character, key)
            )
        }
    }
}
