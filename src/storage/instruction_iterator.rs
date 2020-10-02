use crate::storage::instruction::{unpack_instruction, Instruction};

pub struct InstructionLogIterator {
    log_bytes: Vec<u8>,
    index: usize,
}

impl From<Vec<u8>> for InstructionLogIterator {
    fn from(buffer: Vec<u8>) -> Self {
        return InstructionLogIterator {
            log_bytes: buffer,
            index: 0,
        };
    }
}

impl Iterator for InstructionLogIterator {
    type Item = (Instruction, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        return if index >= self.log_bytes.len() {
            None
        } else {
            let (instruction, bytes_consumed) =
                unpack_instruction(&self.log_bytes[index..]).ok()?;
            self.index += bytes_consumed;
            Some((instruction, bytes_consumed))
        };
    }
}

#[cfg(test)]
mod instruction_iterator_tests {
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::ecc::ECCMode;
    use crate::storage::instruction::{pack_instruction, Instruction};
    use crate::storage::instruction_iterator::InstructionLogIterator;
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

    fn generate_instruction_log(instructions: &[Instruction]) -> Vec<u8> {
        let pack: Vec<u8> = instructions
            .iter()
            .map(|instruction| pack_instruction(instruction, ECCMode::Identity))
            .flatten()
            .collect();

        return pack;
    }

    #[test]
    fn iterate_iterator() {
        let instructions = get_instructions();
        let log = generate_instruction_log(&instructions);

        let iterator = InstructionLogIterator::from(log);

        for (index, (actual_instruction, _)) in iterator.enumerate() {
            let expected_instruction = &instructions[index];
            assert_eq!(&actual_instruction, expected_instruction);
        }
    }

    #[test]
    fn iterate_with_trailing_trash_bytes() {
        let instructions = get_instructions();
        let mut log = generate_instruction_log(&instructions);

        let trailing_trash_bytes = [0xff, 0x00, 0xfa];
        log.extend_from_slice(&trailing_trash_bytes);

        let mut iterator = InstructionLogIterator::from(log);

        for expected_instruction in instructions {
            let (actual_instruction, _) = &iterator.next().unwrap();
            assert_eq!(actual_instruction, &expected_instruction);
        }

        let broken_piece = iterator.next();
        assert_eq!(broken_piece, None);
    }
}
