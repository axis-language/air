pub mod compile;
pub mod optimize;
pub mod output;

use std::collections::HashMap;
use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Register {
	RAX, RBX, RCX, RDX,
	RSI, RDI, RSP, RBP,
	R8, R9, R10, R11, R12, R13, R14, R15,
	RIP,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operand {
	Register(Register),
	Immediate(usize)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Node {
	/* Inline */
	InlineString { string: String },
	InlineByte { data: u8 },

	/* Control */
	Call { function: FunctionID },

	Mov { destination: Register, a: Operand },
	
	Jmp { target: FunctionID },

	/* Arithmetic */
	Add { destination: Register, a: Operand },
	Sub { destination: Register, a: Operand },
	Mul { destination: Register, a: Operand },
	Div { destination: Register, a: Operand },
	
	/* Logic */
	And { destination: Register, a: Operand },
	Or { destination: Register, a: Operand },
	Xor { destination: Register, a: Operand },
	Not { destination: Register },
	Shl { destination: Register, a: Operand },
	Shr { destination: Register, a: Operand },
	
}

pub type Block = Vec<Node>;

#[derive(Clone, PartialEq, Debug)]
pub struct Program {
	pub main: Block,
	pub functions: HashMap<FunctionID, Block>,
}

impl Program {
	pub fn new() -> Self {
		Program {
			main: vec![],
			functions: HashMap::new(),
		}
	}

	pub fn set_main(&mut self, code: Block) {
		self.main = code;
	}

	pub fn set_function(&mut self, name: String, code: Block) {
		self.functions.insert(name, code);
	}
}