//! Definition of each opcode of the EVM.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{error::Error, Target, RW};

/// Opcode enum. One-to-one corresponding to an `u8` value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Opcode(pub u8);

// Core opcodes.
impl Opcode {
    /// `STOP`
    pub const STOP: Opcode = Opcode(0x00);
    /// `ADD`
    pub const ADD: Opcode = Opcode(0x01);
    /// `MUL`
    pub const MUL: Opcode = Opcode(0x02);
    /// `SUB`
    pub const SUB: Opcode = Opcode(0x03);
    /// `DIV`
    pub const DIV: Opcode = Opcode(0x04);
    /// `SDIV`
    pub const SDIV: Opcode = Opcode(0x05);
    /// `MOD`
    pub const MOD: Opcode = Opcode(0x06);
    /// `SMOD`
    pub const SMOD: Opcode = Opcode(0x07);
    /// `ADDMOD`
    pub const ADDMOD: Opcode = Opcode(0x08);
    /// `MULMOD`
    pub const MULMOD: Opcode = Opcode(0x09);
    /// `EXP`
    pub const EXP: Opcode = Opcode(0x0a);
    /// `SIGNEXTEND`
    pub const SIGNEXTEND: Opcode = Opcode(0x0b);

    /// `LT`
    pub const LT: Opcode = Opcode(0x10);
    /// `GT`
    pub const GT: Opcode = Opcode(0x11);
    /// `SLT`
    pub const SLT: Opcode = Opcode(0x12);
    /// `SGT`
    pub const SGT: Opcode = Opcode(0x13);
    /// `EQ`
    pub const EQ: Opcode = Opcode(0x14);
    /// `ISZERO`
    pub const ISZERO: Opcode = Opcode(0x15);
    /// `AND`
    pub const AND: Opcode = Opcode(0x16);
    /// `OR`
    pub const OR: Opcode = Opcode(0x17);
    /// `XOR`
    pub const XOR: Opcode = Opcode(0x18);
    /// `NOT`
    pub const NOT: Opcode = Opcode(0x19);
    /// `BYTE`
    pub const BYTE: Opcode = Opcode(0x1a);

    /// `CALLDATALOAD`
    pub const CALLDATALOAD: Opcode = Opcode(0x35);
    /// `CALLDATASIZE`
    pub const CALLDATASIZE: Opcode = Opcode(0x36);
    /// `CALLDATACOPY`
    pub const CALLDATACOPY: Opcode = Opcode(0x37);
    /// `CODESIZE`
    pub const CODESIZE: Opcode = Opcode(0x38);
    /// `CODECOPY`
    pub const CODECOPY: Opcode = Opcode(0x39);

    /// `SHL`
    pub const SHL: Opcode = Opcode(0x1b);
    /// `SHR`
    pub const SHR: Opcode = Opcode(0x1c);
    /// `SAR`
    pub const SAR: Opcode = Opcode(0x1d);

    /// `POP`
    pub const POP: Opcode = Opcode(0x50);
    /// `MLOAD`
    pub const MLOAD: Opcode = Opcode(0x51);
    /// `MSTORE`
    pub const MSTORE: Opcode = Opcode(0x52);
    /// `MSTORE8`
    pub const MSTORE8: Opcode = Opcode(0x53);
    /// `JUMP`
    pub const JUMP: Opcode = Opcode(0x56);
    /// `JUMPI`
    pub const JUMPI: Opcode = Opcode(0x57);
    /// `PC`
    pub const PC: Opcode = Opcode(0x58);
    /// `MSIZE`
    pub const MSIZE: Opcode = Opcode(0x59);
    /// `JUMPDEST`
    pub const JUMPDEST: Opcode = Opcode(0x5b);

    /// `PUSHn`
    pub const PUSH1: Opcode = Opcode(0x60);
    pub const PUSH2: Opcode = Opcode(0x61);
    pub const PUSH3: Opcode = Opcode(0x62);
    pub const PUSH4: Opcode = Opcode(0x63);
    pub const PUSH5: Opcode = Opcode(0x64);
    pub const PUSH6: Opcode = Opcode(0x65);
    pub const PUSH7: Opcode = Opcode(0x66);
    pub const PUSH8: Opcode = Opcode(0x67);
    pub const PUSH9: Opcode = Opcode(0x68);
    pub const PUSH10: Opcode = Opcode(0x69);
    pub const PUSH11: Opcode = Opcode(0x6a);
    pub const PUSH12: Opcode = Opcode(0x6b);
    pub const PUSH13: Opcode = Opcode(0x6c);
    pub const PUSH14: Opcode = Opcode(0x6d);
    pub const PUSH15: Opcode = Opcode(0x6e);
    pub const PUSH16: Opcode = Opcode(0x6f);
    pub const PUSH17: Opcode = Opcode(0x70);
    pub const PUSH18: Opcode = Opcode(0x71);
    pub const PUSH19: Opcode = Opcode(0x72);
    pub const PUSH20: Opcode = Opcode(0x73);
    pub const PUSH21: Opcode = Opcode(0x74);
    pub const PUSH22: Opcode = Opcode(0x75);
    pub const PUSH23: Opcode = Opcode(0x76);
    pub const PUSH24: Opcode = Opcode(0x77);
    pub const PUSH25: Opcode = Opcode(0x78);
    pub const PUSH26: Opcode = Opcode(0x79);
    pub const PUSH27: Opcode = Opcode(0x7a);
    pub const PUSH28: Opcode = Opcode(0x7b);
    pub const PUSH29: Opcode = Opcode(0x7c);
    pub const PUSH30: Opcode = Opcode(0x7d);
    pub const PUSH31: Opcode = Opcode(0x7e);
    pub const PUSH32: Opcode = Opcode(0x7f);

    /// `DUPn`
    pub const DUP1: Opcode = Opcode(0x80);
    pub const DUP2: Opcode = Opcode(0x81);
    pub const DUP3: Opcode = Opcode(0x82);
    pub const DUP4: Opcode = Opcode(0x83);
    pub const DUP5: Opcode = Opcode(0x84);
    pub const DUP6: Opcode = Opcode(0x85);
    pub const DUP7: Opcode = Opcode(0x86);
    pub const DUP8: Opcode = Opcode(0x87);
    pub const DUP9: Opcode = Opcode(0x88);
    pub const DUP10: Opcode = Opcode(0x89);
    pub const DUP11: Opcode = Opcode(0x8a);
    pub const DUP12: Opcode = Opcode(0x8b);
    pub const DUP13: Opcode = Opcode(0x8c);
    pub const DUP14: Opcode = Opcode(0x8d);
    pub const DUP15: Opcode = Opcode(0x8e);
    pub const DUP16: Opcode = Opcode(0x8f);

    /// `SWAPn`
    pub const SWAP1: Opcode = Opcode(0x90);
    pub const SWAP2: Opcode = Opcode(0x91);
    pub const SWAP3: Opcode = Opcode(0x92);
    pub const SWAP4: Opcode = Opcode(0x93);
    pub const SWAP5: Opcode = Opcode(0x94);
    pub const SWAP6: Opcode = Opcode(0x95);
    pub const SWAP7: Opcode = Opcode(0x96);
    pub const SWAP8: Opcode = Opcode(0x97);
    pub const SWAP9: Opcode = Opcode(0x98);
    pub const SWAP10: Opcode = Opcode(0x99);
    pub const SWAP11: Opcode = Opcode(0x9a);
    pub const SWAP12: Opcode = Opcode(0x9b);
    pub const SWAP13: Opcode = Opcode(0x9c);
    pub const SWAP14: Opcode = Opcode(0x9d);
    pub const SWAP15: Opcode = Opcode(0x9e);
    pub const SWAP16: Opcode = Opcode(0x9f);

    /// `RETURN`
    pub const RETURN: Opcode = Opcode(0xf3);
    /// `REVERT`
    pub const REVERT: Opcode = Opcode(0xfd);

    /// `INVALID`
    pub const INVALID: Opcode = Opcode(0xfe);
}

// External opcodes
impl Opcode {
    /// `SHA3`
    pub const SHA3: Opcode = Opcode(0x20);
    /// `ADDRESS`
    pub const ADDRESS: Opcode = Opcode(0x30);
    /// `BALANCE`
    pub const BALANCE: Opcode = Opcode(0x31);
    /// `SELFBALANCE`
    pub const SELFBALANCE: Opcode = Opcode(0x47);
    /// `ORIGIN`
    pub const ORIGIN: Opcode = Opcode(0x32);
    /// `CALLER`
    pub const CALLER: Opcode = Opcode(0x33);
    /// `CALLVALUE`
    pub const CALLVALUE: Opcode = Opcode(0x34);
    /// `GASPRICE`
    pub const GASPRICE: Opcode = Opcode(0x3a);
    /// `EXTCODESIZE`
    pub const EXTCODESIZE: Opcode = Opcode(0x3b);
    /// `EXTCODECOPY`
    pub const EXTCODECOPY: Opcode = Opcode(0x3c);
    /// `EXTCODEHASH`
    pub const EXTCODEHASH: Opcode = Opcode(0x3f);
    /// `RETURNDATASIZE`
    pub const RETURNDATASIZE: Opcode = Opcode(0x3d);
    /// `RETURNDATACOPY`
    pub const RETURNDATACOPY: Opcode = Opcode(0x3e);
    /// `BLOCKHASH`
    pub const BLOCKHASH: Opcode = Opcode(0x40);
    /// `COINBASE`
    pub const COINBASE: Opcode = Opcode(0x41);
    /// `TIMESTAMP`
    pub const TIMESTAMP: Opcode = Opcode(0x42);
    /// `NUMBER`
    pub const NUMBER: Opcode = Opcode(0x43);
    /// `DIFFICULTY`
    pub const DIFFICULTY: Opcode = Opcode(0x44);
    /// `GASLIMIT`
    pub const GASLIMIT: Opcode = Opcode(0x45);
    /// `SLOAD`
    pub const SLOAD: Opcode = Opcode(0x54);
    /// `SSTORE`
    pub const SSTORE: Opcode = Opcode(0x55);
    /// `GAS`
    pub const GAS: Opcode = Opcode(0x5a);
    /// `LOGn`
    pub const LOG0: Opcode = Opcode(0xa0);
    pub const LOG1: Opcode = Opcode(0xa1);
    pub const LOG2: Opcode = Opcode(0xa2);
    pub const LOG3: Opcode = Opcode(0xa3);
    pub const LOG4: Opcode = Opcode(0xa4);
    /// `CREATE`
    pub const CREATE: Opcode = Opcode(0xf0);
    /// `CREATE2`
    pub const CREATE2: Opcode = Opcode(0xf5);
    /// `CALL`
    pub const CALL: Opcode = Opcode(0xf1);
    /// `CALLCODE`
    pub const CALLCODE: Opcode = Opcode(0xf2);
    /// `DELEGATECALL`
    pub const DELEGATECALL: Opcode = Opcode(0xf4);
    /// `STATICCALL`
    pub const STATICCALL: Opcode = Opcode(0xfa);
    /// `SUICIDE`
    pub const SUICIDE: Opcode = Opcode(0xff);
    /// `CHAINID`
    pub const CHAINID: Opcode = Opcode(0x46);
}

impl Opcode {
    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl FromStr for Opcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "STOP" => Opcode(0x00),
            "ADD" => Opcode(0x01),
            "MUL" => Opcode(0x02),
            "SUB" => Opcode(0x03),
            "DIV" => Opcode(0x04),
            "SDIV" => Opcode(0x05),
            "MOD" => Opcode(0x06),
            "SMOD" => Opcode(0x07),
            "ADDMOD" => Opcode(0x08),
            "MULMOD" => Opcode(0x09),
            "EXP" => Opcode(0x0a),
            "SIGNEXTEND" => Opcode(0x0b),
            "LT" => Opcode(0x10),
            "GT" => Opcode(0x11),
            "SLT" => Opcode(0x12),
            "SGT" => Opcode(0x13),
            "EQ" => Opcode(0x14),
            "ISZERO" => Opcode(0x15),
            "AND" => Opcode(0x16),
            "OR" => Opcode(0x17),
            "XOR" => Opcode(0x18),
            "NOT" => Opcode(0x19),
            "BYTE" => Opcode(0x1a),
            "CALLDATALOAD" => Opcode(0x35),
            "CALLDATASIZE" => Opcode(0x36),
            "CALLDATACOPY" => Opcode(0x37),
            "CODESIZE" => Opcode(0x38),
            "CODECOPY" => Opcode(0x39),
            "SHL" => Opcode(0x1b),
            "SHR" => Opcode(0x1c),
            "SAR" => Opcode(0x1d),
            "POP" => Opcode(0x50),
            "MLOAD" => Opcode(0x51),
            "MSTORE" => Opcode(0x52),
            "MSTORE8" => Opcode(0x53),
            "JUMP" => Opcode(0x56),
            "JUMPI" => Opcode(0x57),
            "PC" => Opcode(0x58),
            "MSIZE" => Opcode(0x59),
            "JUMPDEST" => Opcode(0x5b),
            "PUSH1" => Opcode(0x60),
            "PUSH2" => Opcode(0x61),
            "PUSH3" => Opcode(0x62),
            "PUSH4" => Opcode(0x63),
            "PUSH5" => Opcode(0x64),
            "PUSH6" => Opcode(0x65),
            "PUSH7" => Opcode(0x66),
            "PUSH8" => Opcode(0x67),
            "PUSH9" => Opcode(0x68),
            "PUSH10" => Opcode(0x69),
            "PUSH11" => Opcode(0x6a),
            "PUSH12" => Opcode(0x6b),
            "PUSH13" => Opcode(0x6c),
            "PUSH14" => Opcode(0x6d),
            "PUSH15" => Opcode(0x6e),
            "PUSH16" => Opcode(0x6f),
            "PUSH17" => Opcode(0x70),
            "PUSH18" => Opcode(0x71),
            "PUSH19" => Opcode(0x72),
            "PUSH20" => Opcode(0x73),
            "PUSH21" => Opcode(0x74),
            "PUSH22" => Opcode(0x75),
            "PUSH23" => Opcode(0x76),
            "PUSH24" => Opcode(0x77),
            "PUSH25" => Opcode(0x78),
            "PUSH26" => Opcode(0x79),
            "PUSH27" => Opcode(0x7a),
            "PUSH28" => Opcode(0x7b),
            "PUSH29" => Opcode(0x7c),
            "PUSH30" => Opcode(0x7d),
            "PUSH31" => Opcode(0x7e),
            "PUSH32" => Opcode(0x7f),
            "DUP1" => Opcode(0x80),
            "DUP2" => Opcode(0x81),
            "DUP3" => Opcode(0x82),
            "DUP4" => Opcode(0x83),
            "DUP5" => Opcode(0x84),
            "DUP6" => Opcode(0x85),
            "DUP7" => Opcode(0x86),
            "DUP8" => Opcode(0x87),
            "DUP9" => Opcode(0x88),
            "DUP10" => Opcode(0x89),
            "DUP11" => Opcode(0x8a),
            "DUP12" => Opcode(0x8b),
            "DUP13" => Opcode(0x8c),
            "DUP14" => Opcode(0x8d),
            "DUP15" => Opcode(0x8e),
            "DUP16" => Opcode(0x8f),
            "SWAP1" => Opcode(0x90),
            "SWAP2" => Opcode(0x91),
            "SWAP3" => Opcode(0x92),
            "SWAP4" => Opcode(0x93),
            "SWAP5" => Opcode(0x94),
            "SWAP6" => Opcode(0x95),
            "SWAP7" => Opcode(0x96),
            "SWAP8" => Opcode(0x97),
            "SWAP9" => Opcode(0x98),
            "SWAP10" => Opcode(0x99),
            "SWAP11" => Opcode(0x9a),
            "SWAP12" => Opcode(0x9b),
            "SWAP13" => Opcode(0x9c),
            "SWAP14" => Opcode(0x9d),
            "SWAP15" => Opcode(0x9e),
            "SWAP16" => Opcode(0x9f),
            "RETURN" => Opcode(0xf3),
            "REVERT" => Opcode(0xfd),
            "INVALID" => Opcode(0xfe),
            _ => return Err(Error::OpcodeParsing),
        })
    }
}

impl Opcode {
    pub const fn target_and_rw(&self) -> (RW, Target) {
        // match self {
        //
        // }

        unimplemented!()
    }
}
