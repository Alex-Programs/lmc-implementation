use std::collections::HashMap;

#[derive(Debug)]
pub struct Instruction {
    name: String,
    code: u16,
    operand: Option<u16>,
}

pub fn split_and_trim(s: &str) -> Vec<&str> {
    let lines = s.split("\n");
    lines.map(|line| line.trim()).collect()
}

pub fn str_to_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut name_to_code: HashMap<&str, u16> = HashMap::new();
    name_to_code.insert("ADD", 1);
    name_to_code.insert("SUB", 2);
    name_to_code.insert("STA", 3);
    name_to_code.insert("LDA", 5);
    name_to_code.insert("BRA", 6);
    name_to_code.insert("BRZ", 7);
    name_to_code.insert("BRP", 8);
    name_to_code.insert("INP", 901);
    name_to_code.insert("OUT", 902);
    name_to_code.insert("HLT", 0);
    name_to_code.insert("DAT", 1000);

    for line in split_and_trim(s) {
        if line.starts_with("//") {
            continue;
        }

        let mut parts = line.split(" ");

        let name = parts.next().unwrap();
        let operand_opt = parts.next();

        let code = match name_to_code.get(name) {
            Some(code) => *code,
            None => {
                if name.chars().count() == 0 {
                    continue;
                } else {
                    panic!("Unknown instruction: {}", name);
                }
            }
        };

        let operand = match operand_opt {
            Some(operand) => Some(match operand.parse::<u16>() {
                Ok(operand) => operand,
                Err(_) => panic!("Invalid operand for u16 parse: {}", operand),
            }),
            None => None,
        };

        instructions.push(Instruction {
            name: name.to_string(),
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
    fn test_add() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("ADD {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "ADD");
            assert_eq!(instructions[0].code, 1);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_sub() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("SUB {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "SUB");
            assert_eq!(instructions[0].code, 2);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_sta() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("STA {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "STA");
            assert_eq!(instructions[0].code, 3);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_lda() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("LDA {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "LDA");
            assert_eq!(instructions[0].code, 5);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_bra() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("BRA {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "BRA");
            assert_eq!(instructions[0].code, 6);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_brz() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("BRZ {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "BRZ");
            assert_eq!(instructions[0].code, 7);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_brp() {
        let mut i: u16 = 0;
        loop {
            i += 1;

            if i > 99 {
                break;
            }

            let instructions = str_to_instructions(&format!("BRP {}", i));
            assert_eq!(instructions.len(), 1);
            assert_eq!(instructions[0].name, "BRP");
            assert_eq!(instructions[0].code, 8);
            assert_eq!(instructions[0].operand, Some(i));
        }
    }

    #[test]
    fn test_inp() {
        let instructions = str_to_instructions("INP");
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].name, "INP");
        assert_eq!(instructions[0].code, 901);
        assert_eq!(instructions[0].operand, None);
    }

    #[test]
    fn test_out() {
        let instructions = str_to_instructions("OUT");
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].name, "OUT");
        assert_eq!(instructions[0].code, 902);
        assert_eq!(instructions[0].operand, None);
    }

    #[test]
    fn test_hlt() {
        let instructions = str_to_instructions("HLT");
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].name, "HLT");
        assert_eq!(instructions[0].code, 0);
        assert_eq!(instructions[0].operand, None);
    }

    #[test]
    fn test_dat() {
        let instructions = str_to_instructions("DAT 1");
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0].name, "DAT");
        assert_eq!(instructions[0].code, 1000);
        assert_eq!(instructions[0].operand, Some(1));
    }

    #[test]
    fn test_instructions_together() {
        let s = "ADD 1
            SUB 2
            // This is a comment
            STA 3
            LDA 5
            BRA 6
            BRZ 7
            BRP 8
            INP
            OUT
            HLT
            DAT 1000";
        let instructions = str_to_instructions(s);

        assert_eq!(instructions[0].name, "ADD");
        assert_eq!(instructions[0].code, 1);
        assert_eq!(instructions[0].operand, Some(1));

        assert_eq!(instructions[1].name, "SUB");
        assert_eq!(instructions[1].code, 2);
        assert_eq!(instructions[1].operand, Some(2));

        assert_eq!(instructions[2].name, "STA");
        assert_eq!(instructions[2].code, 3);
        assert_eq!(instructions[2].operand, Some(3));

        assert_eq!(instructions[3].name, "LDA");
        assert_eq!(instructions[3].code, 5);
        assert_eq!(instructions[3].operand, Some(5));

        assert_eq!(instructions[4].name, "BRA");
        assert_eq!(instructions[4].code, 6);
        assert_eq!(instructions[4].operand, Some(6));

        assert_eq!(instructions[5].name, "BRZ");
        assert_eq!(instructions[5].code, 7);
        assert_eq!(instructions[5].operand, Some(7));

        assert_eq!(instructions[6].name, "BRP");
        assert_eq!(instructions[6].code, 8);
        assert_eq!(instructions[6].operand, Some(8));

        assert_eq!(instructions[7].name, "INP");
        assert_eq!(instructions[7].code, 901);
        assert_eq!(instructions[7].operand, None);

        assert_eq!(instructions[8].name, "OUT");
        assert_eq!(instructions[8].code, 902);
        assert_eq!(instructions[8].operand, None);

        assert_eq!(instructions[9].name, "HLT");
        assert_eq!(instructions[9].code, 0);
        assert_eq!(instructions[9].operand, None);

        assert_eq!(instructions[10].name, "DAT");
        assert_eq!(instructions[10].code, 1000);
        assert_eq!(instructions[10].operand, Some(1000));
    }
}
