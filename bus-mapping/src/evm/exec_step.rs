//! Doc this

use super::{EvmWord, GlobalCounter, Instruction, MemAddress, ProgramCounter};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionStep {
    memory: BTreeMap<MemAddress, EvmWord>,
    stack: Vec<EvmWord>,
    opcode: Instruction,
    pc: ProgramCounter,
    gc: GlobalCounter,
}

impl ExecutionStep {
    pub const fn new(
        memory: BTreeMap<MemAddress, EvmWord>,
        stack: Vec<EvmWord>,
        opcode: Instruction,
        pc: ProgramCounter,
        gc: GlobalCounter,
    ) -> Self {
        ExecutionStep {
            memory,
            stack,
            opcode,
            pc,
            gc,
        }
    }
}

impl<'a> TryFrom<(&ParsedExecutionStep<'a>, GlobalCounter)> for ExecutionStep {
    type Error = Error;

    fn try_from(
        parse_info: (&ParsedExecutionStep<'a>, GlobalCounter),
    ) -> Result<Self, Self::Error> {
        // Memory part
        let mut mem_map = BTreeMap::new();
        parse_info
            .0
            .memory
            .iter()
            .map(|(mem_addr, word)| {
                mem_map.insert(MemAddress::from_str(mem_addr)?, EvmWord::from_str(word)?);
                Ok(())
            })
            .collect::<Result<(), Error>>()?;

        // Stack part
        let mut stack = vec![];
        parse_info
            .0
            .stack
            .iter()
            .map(|word| {
                stack.push(EvmWord::from_str(word)?);
                Ok(())
            })
            .collect::<Result<(), Error>>()?;

        Ok(ExecutionStep::new(
            mem_map,
            stack,
            Instruction::from_str(parse_info.0.opcode)?,
            parse_info.0.pc,
            parse_info.1,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[doc(hidden)]
struct ParsedExecutionStep<'a> {
    memory: HashMap<&'a str, &'a str>,
    stack: Vec<&'a str>,
    opcode: &'a str,
    pc: ProgramCounter,
}

mod tests {
    use super::*;
    use crate::evm::Opcode;
    use num::BigUint;

    #[test]
    fn parse_single_step() {
        let step_json = r#"
        {
            "memory": {
                "0": "0000000000000000000000000000000000000000000000000000000000000000",
                "20": "0000000000000000000000000000000000000000000000000000000000000000",
                "40": "0000000000000000000000000000000000000000000000000000000000000080"
            },
            "stack": [],
            "opcode": "JUMPDEST",
            "pc": 53
        }
        "#;

        let trace_loaded = ExecutionStep::try_from((
            &serde_json::from_str::<ParsedExecutionStep>(step_json).expect("Error on parsing"),
            GlobalCounter(0usize),
        ))
        .expect("Error on conversion");

        let expected_trace = {
            let mut mem_map = BTreeMap::new();
            mem_map.insert(MemAddress(0x00), EvmWord(BigUint::from(0u8)));
            mem_map.insert(MemAddress(0x20), EvmWord(BigUint::from(0u8)));
            mem_map.insert(MemAddress(0x40), EvmWord(BigUint::from(0x80u8)));

            ExecutionStep::new(
                mem_map,
                vec![],
                Instruction::new(Opcode::JUMPDEST, None),
                ProgramCounter(53),
                GlobalCounter(0),
            )
        };

        assert_eq!(trace_loaded, expected_trace)
    }

    #[test]
    fn parse_execution_trace() {
        let input_trace = r#"
        [
            {
                "memory": {
                    "00": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080"
                },
                "stack": [],
                "opcode": "JUMPDEST",
                "pc": 53
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080"
                },
                "stack": [
                    "40"
                ],
                "opcode": "PUSH1 40",
                "pc": 54
            },
            {
                "memory": {
                    "00": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080"
                },
                "stack": [
                    "80"
                ],
                "opcode": "MLOAD",
                "pc": 56
            },
            {
                "memory": {
                    "00": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080"
                },
                "stack": [
                    "deadbeef",  
                    "80"
                ],
                "opcode": "PUSH4 deadbeaf",
                "pc": 57
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080"
                },
                "stack": [
                    "80",   
                    "deadbeef",  
                    "80"  
                ],
                "opcode": "DUP2",
                "pc": 62
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "80"
                ],
                "opcode": "MSTORE",
                "pc": 63
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "faceb00c",
                    "80"
                ],
                "opcode": "PUSH4 faceb00c",
                "pc": 64
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "80",
                    "faceb00c",
                    "80"
                ],
                "opcode": "DUP2",
                "pc": 69
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "deadbeef",   
                    "faceb00c",  
                    "80"
                ],
                "opcode": "MLOAD",
                "pc": 70
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "1d97c6efb",
                    "80"
                ],
                "opcode": "ADD",
                "pc": 71
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000000deadbeef"
                },
                "stack": [
                    "80",
                    "1d97c6efb",
                    "80"
                ],
                "opcode": "DUP2",
                "pc": 72
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb"
                },
                "stack": [
                    "80"
                ],
                "opcode": "MSTORE",
                "pc": 73
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb"
                },
                "stack": [
                    "cafeb0ba",
                    "80"
                ],
                "opcode": "PUSH4 cafeb0ba",
                "pc": 74
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb"
                },
                "stack": [
                    "20",
                    "cafeb0ba",
                    "80"
                ],
                "opcode": "PUSH1 20",
                "pc": 79
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb"
                },
                "stack": [
                    "80",
                    "20",
                    "cafeb0ba",
                    "80"
                ],
                "opcode": "DUP3",
                "pc": 81
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb"
                },
                "stack": [
                    "a0",
                    "cafeb0ba",
                    "80"
                ],
                "opcode": "ADD",
                "pc": 82
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb",
                    "a0": "00000000000000000000000000000000000000000000000000000000cafeb0ba"

                },
                "stack": [
                    "80"
                ],
                "opcode": "MSTORE",
                "pc": 83
            },
            {
                "memory": {
                    "0": "0000000000000000000000000000000000000000000000000000000000000000",
                    "20": "0000000000000000000000000000000000000000000000000000000000000000",
                    "40": "0000000000000000000000000000000000000000000000000000000000000080",
                    "80": "00000000000000000000000000000000000000000000000000000001d97c6efb",
                    "a0": "00000000000000000000000000000000000000000000000000000000cafeb0ba"
                },
                "stack": [],
                "opcode": "POP",
                "pc": 84
            }
        ]"#;

        let trace_loaded: Vec<ExecutionStep> =
            serde_json::from_str::<Vec<ParsedExecutionStep>>(input_trace)
                .expect("Error on parsing")
                .iter()
                .enumerate()
                .map(|(idx, step)| ExecutionStep::try_from((step, GlobalCounter(idx))))
                .collect::<Result<Vec<ExecutionStep>, Error>>()
                .expect("Error on conversion");

        let expected_trace = {
            let mut mem_map = BTreeMap::new();
            mem_map.insert(MemAddress(0x00), EvmWord(BigUint::from(0u8)));
            mem_map.insert(MemAddress(0x20), EvmWord(BigUint::from(0u8)));
            mem_map.insert(MemAddress(0x40), EvmWord(BigUint::from(0x80u8)));
            mem_map.insert(MemAddress(0x80), EvmWord(BigUint::from(0x1d97c6efbu128)));
            mem_map.insert(MemAddress(0xa0), EvmWord(BigUint::from(0xcafeb0bau32)));

            ExecutionStep::new(
                mem_map,
                vec![],
                Instruction::new(Opcode::POP, None),
                ProgramCounter(84),
                GlobalCounter(trace_loaded.len()),
            )
        };

        assert_eq!(*trace_loaded.last().unwrap(), expected_trace)
    }
}
