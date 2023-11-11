/*
A spy is sending encrypted messages.

Your mission is to create a program that decodes the messages.

The messages are words separated by spaces like this:
cat dog dog car Cat doG sun

We need the program to return the number of times each word appears in the message, regardless of whether it is in uppercase or lowercase.

The result will be a text string with the word and the number of times it appears in the message, in this format:
cat2dog3car1sun1

The words are sorted by their first appearance in the message!
*/
use std::collections::HashMap;

pub fn decoder(message: &str) -> String {
    let mut word_count: HashMap<&str, usize> = HashMap::new();
    let mut word_order: Vec<&str> = Vec::new();
    let encoded_msg = message.to_ascii_lowercase();
    for word in encoded_msg.split_ascii_whitespace() {
        let result = word_count
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if *result == 1 {
            word_order.push(word)
        }
    }
    let mut decoded_msg = String::new();
    for word in word_order {
        decoded_msg.push_str(&format!("{}{}", word, word_count.get(word).unwrap()))
    }
    decoded_msg
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correctly_decode() {
        let test_cases = [
            ["cat dog dog car Cat doG sun", "cat2dog3car1sun1"],
            ["keys house HOUSE house keys", "keys2house3"],
            ["cup te a cup", "cup2te1a1"],
            ["houses house housess", "houses1house1housess1"],
        ];
        for [encoded, decoded] in test_cases {
            assert_eq!(decoded, decoder(encoded));
        }
    }
}
