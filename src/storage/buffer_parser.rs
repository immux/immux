use crate::storage::instruction::Instruction;

pub struct InstructionBufferParser<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> InstructionBufferParser<'a> {
    pub fn new(buffer: &[u8], index: usize) -> InstructionBufferParser {
        return InstructionBufferParser { buffer, index };
    }
}

impl<'a> Iterator for InstructionBufferParser<'a> {
    type Item = (Instruction, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match Instruction::parse(&self.buffer[self.index..]) {
            Ok((instruction, instruction_length)) => {
                self.index += instruction_length;
                return Some((instruction, instruction_length));
            }
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::buffer_parser::InstructionBufferParser;
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::instruction::Instruction;
    use crate::storage::kvkey::KVKey;
    use crate::storage::kvvalue::KVValue;

    fn get_instructions() -> Vec<Instruction> {
        let instructions: Vec<Instruction> = vec![
            Instruction::Set {
                key: KVKey::new(&[0x00, 0x01]),
                value: KVValue::new(&[0xff, 0xf3]),
            },
            Instruction::RevertOne {
                key: KVKey::new(&[0x11, 0x22]),
                height: ChainHeight::new(3),
            },
            Instruction::RevertAll {
                height: ChainHeight::new(6),
            },
            Instruction::RemoveOne {
                key: KVKey::new(&[0x88]),
            },
            Instruction::RemoveAll,
        ];

        return instructions;
    }

    fn serialize_instructions(instructions: &[Instruction]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        &instructions
            .to_vec()
            .iter()
            .fold(&mut buffer, |acc, instruction| {
                let instruction_bytes: Vec<u8> = instruction.serialize();
                acc.extend_from_slice(&instruction_bytes);
                return acc;
            });

        return buffer;
    }

    #[test]
    fn iterate_buffer_parser() {
        let instructions = get_instructions();
        let buffer = serialize_instructions(&instructions);

        let instruction_buffer_parser = InstructionBufferParser {
            buffer: &buffer,
            index: 0,
        };

        for (index, (actual_instruction, _)) in instruction_buffer_parser.enumerate() {
            let expected_instruction = &instructions[index];
            assert_eq!(&actual_instruction, expected_instruction);
        }
    }

    #[test]
    fn iterate_buffer_parser_with_broken_instruction() {
        let instructions = get_instructions();
        let mut buffer = serialize_instructions(&instructions);

        let some_broken_bytes = [0xff, 0x00, 0xfa];
        buffer.extend_from_slice(&some_broken_bytes);

        let mut instruction_buffer_parser = InstructionBufferParser {
            buffer: &buffer,
            index: 0,
        };

        for expected_instruction in instructions {
            let (actual_instruction, _) = &instruction_buffer_parser.next().unwrap();
            assert_eq!(actual_instruction, &expected_instruction);
        }

        let broken_piece = instruction_buffer_parser.next();
        assert_eq!(broken_piece, None);
    }
}
