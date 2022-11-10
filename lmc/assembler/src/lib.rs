#[derive(Debug)]
struct Instruction {
    name: &str,
    code: u16,
    operand: Option<u8>,
}

pub fn split_and_trim(s: &str) -> Vec<&str> {
    s.split("\n").collect().map(|s| s.trim())
}

pub fn str_to_instructions(s: &str) -> Vec<&Instruction> {
    let mut instructions = Vec::new();

    let name_to_code = HashMap::new();
    name_to_instruction.insert("ADD", 1);
    name_to_instruction.insert("SUB", 2);
    name_to_instruction.insert("STA", 3);
    name_to_instruction.insert("LDA", 5);
    name_to_instruction.insert("BRA", 6);
    name_to_instruction.insert("BRZ", 7);
    name_to_instruction.insert("BRP", 8);
    name_to_instruction.insert("INP", 901);
    name_to_instruction.insert("OUT", 902);
    name_to_instruction.insert("HLT", 0);
    name_to_instruction.insert("DAT", 1000);

    for line in split_and_trim(s) {
        if line.starts_with("//") {
            continue;
        }

        let mut parts = line.split(" ");

        let name = parts.next().unwrap();
        let operand_opt = parts.next();

        let code = name_to_code.get(name).unwrap();
        let operand = match operand_opt {
            Some(operand) => Some(operand.parse::<u16>().unwrap()),
            None => None,
        };

        instructions.push(Instruction {
            name,
            code,
            operand,
        });
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_instructions() {
        // TODO write this test
    }
}