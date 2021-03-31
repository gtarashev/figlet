/* Figlet written in rust
 * Author: Georgi Tarashev
 * Version: 0.0.1
 * Date: 31/03/2021
 */

// figlet letters are 6x6, so I can use a letter
// struct which just has 6 strings, for each row
struct Character {
    r1: String,
    r2: String,
    r3: String,
    r4: String,
    r5: String,
    r6: String
}

impl Character {
    // create a new empty Character
    fn new() -> Self {
        Character{ r1: "".to_string(), r2: "".to_string(), r3: "".to_string(), r4: "".to_string(), r5: "".to_string(), r6: "".to_string() }
    }

    // join together the two Characters and return them (as character)
    fn join(char1: Character, char2: Character) -> Character {
        char1
    }
}

fn main() {
}
