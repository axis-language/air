use std::collections::HashMap;
use super::*;

pub type Register = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operand {
	Register(Register),
	Data(usize),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Node {
	/* Inline */
	InlineString { string: String },
	InlineByte { data: u8 },

	/* Control */
	Call { function: FunctionID },
	Branch { condition: bool, function: FunctionID },

	Set { destination: Register, a: Operand },
	ReadMemory { destination: Register, a: Operand },
	WriteMemory { destination: Operand, a: Operand },

	/* Arithmetic */
	Add { destination: Register, a: Operand, b: Operand },
	Subtract { destination: Register, a: Operand, b: Operand },
	Multiply { destination: Register, a: Operand, b: Operand },
	Divide { destination: Register, a: Operand, b: Operand },

	/* Logic */
	And { destination: Register, a: Operand, b: Operand },
	Or{ destination: Register, a: Operand, b: Operand },
	Xor { destination: Register, a: Operand, b: Operand },
	Not { destination: Register, a: Operand },
	LogicalShiftLeft { destination: Register, a: Operand, b: Operand },
	LogicalShiftRight { destination: Register, a: Operand, b: Operand },
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
