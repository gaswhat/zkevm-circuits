//! Evm types needed for parsing instruction sets as well

mod exec_step;
mod opcodes;
use std::{
    collections::{BTreeMap, HashMap},
    convert::{TryFrom, TryInto},
    str::FromStr,
    usize,
};

use crate::{error::Error, Target, RW};
pub use exec_step::ExecutionStep;
use num::{BigUint, Num};
use opcodes::Opcode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    assoc_value: Option<EvmWord>,
}

impl Instruction {
    pub const fn new(op: Opcode, val: Option<EvmWord>) -> Self {
        Instruction {
            opcode: op,
            assoc_value: val,
        }
    }

    pub const fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub const fn value(&self) -> Option<&EvmWord> {
        self.assoc_value.as_ref()
    }

    const fn target_and_rw(&self) -> (Target, RW) {
        self.opcode().target_and_rw()
    }
}

impl FromStr for Instruction {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Separate the instruction from the possible Value associated to it.
        let words: Vec<&str> = s.split_whitespace().into_iter().collect();
        // Allocate value
        let val = match words.get(1) {
            Some(val) => Some(EvmWord::from_str(val)?),
            None => None,
        };

        Ok(Instruction::new(Opcode::from_str(words[0])?, val))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ProgramCounter(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct GlobalCounter(pub(crate) usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct MemAddress(pub(crate) usize);

impl FromStr for MemAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MemAddress(
            BigUint::from_str_radix(s, 16)
                .map_err(|_| Error::EvmWordParsing)
                .map(|biguint| {
                    biguint
                        .try_into()
                        .map_err(|_| Error::EvmWordParsing)
                        .expect("Map_err should be applied")
                })
                .map_err(|_| Error::EvmWordParsing)?,
        ))
    }
}

// XXX: Consider to move this to [u8;32] soon
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct EvmWord(pub(crate) BigUint);

impl FromStr for EvmWord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EvmWord(
            BigUint::from_str_radix(s, 16).map_err(|_| Error::EvmWordParsing)?,
        ))
    }
}
