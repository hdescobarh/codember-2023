/*
** Mini Compiler Challenge **
In the world of espionage, a coding language is used with symbols that perform simple mathematical operations.

Your task is to create a mini compiler that interprets and executes programs written in this symbol language.

The operations of the language are as follows:

"#" Increases the numeric value by 1.
"@" Decreases the numeric value by 1.
"*" Multiplies the numeric value by itself.
"&" Prints the current numeric value.
The initial numeric value is 0 and the operations should be applied in the order in which they appear in the string of symbols.


** Your Challenge: **
Develop a mini compiler that takes a text string and returns another text string with the result of executing the program.
*/

pub struct Compiler {
    value: isize,
    output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            value: 0,
            output: String::new(),
        }
    }
    pub fn build(&mut self, token_stream: &str) -> &str {
        for token in token_stream.chars() {
            self.map_symbols(token);
        }
        &self.output
    }

    fn map_symbols(&mut self, instruction: char) {
        match instruction {
            '#' => self.value += 1,
            '@' => self.value -= 1,
            '*' => self.value = self.value.pow(2),
            '&' => self.output.push_str(&self.value.to_string()),
            _ => panic!("Invalid instruction."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /*
    ** Input Examples: **
    Input: "##*&"
    Expected Output: "4"
    Explanation: Increment (1), increment (2), multiply (4), print (4).

    Input: "&##&*&@&"
    Expected Output: "0243"
    Explanation: Print (0), increment (1), increment (2), print (2), multiply (4), print (4), decrement (3), print (3).
     */

    #[test]
    fn simple_input() {
        let mut compiler = Compiler::new();
        assert_eq!("4", compiler.build("##*&"));
        let mut compiler = Compiler::new();
        assert_eq!("0243", compiler.build("&##&*&@&"));
    }
}
