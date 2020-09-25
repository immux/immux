use crate::storage::instruction::Instruction;

pub struct InstructionIterator {
    buffer: Vec<u8>,
    index: usize,
}

impl From<Vec<u8>> for InstructionIterator {
    fn from(buffer: Vec<u8>) -> Self {
        return InstructionIterator { buffer, index: 0 };
    }
}

impl Iterator for InstructionIterator {
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
mod instruction_iterator_tests {
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::instruction::Instruction;
    use crate::storage::instruction_iterator::InstructionIterator;
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
    fn iterate_iterator() {
        let instructions = get_instructions();
        let buffer = serialize_instructions(&instructions);

        let iterator = InstructionIterator::from(buffer);

        for (index, (actual_instruction, _)) in iterator.enumerate() {
            let expected_instruction = &instructions[index];
            assert_eq!(&actual_instruction, expected_instruction);
        }
    }

    #[test]
    fn iterate_with_broken_instruction() {
        let instructions = get_instructions();
        let mut buffer = serialize_instructions(&instructions);

        let some_broken_bytes = [0xff, 0x00, 0xfa];
        buffer.extend_from_slice(&some_broken_bytes);

        let mut iterator = InstructionIterator::from(buffer);

        for expected_instruction in instructions {
            let (actual_instruction, _) = &iterator.next().unwrap();
            assert_eq!(actual_instruction, &expected_instruction);
        }

        let broken_piece = iterator.next();
        assert_eq!(broken_piece, None);
    }
}
