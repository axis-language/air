/// X86_64 register.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Register {
	RAX, RBX, RCX, RSP,
	RBP, RDI, RSI, RDX,
	R8,  R9,  R10, R11,
	R12, R13, R14, R15,
}

impl std::fmt::Display for Register {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Register::RAX => "rax",
			Register::RBX => "rbx",
			Register::RCX => "rcx",
			Register::RDX => "rdx",
			Register::RSP => "rsp",
			Register::RBP => "rbp",
			Register::RDI => "rdi",
			Register::RSI => "rsi",
			Register::R8 => "r8",
			Register::R9 => "r9",
			Register::R10 => "r10",
			Register::R11 => "r11",
			Register::R12 => "r12",
			Register::R13 => "r13",
			Register::R14 => "r14",
			Register::R15 => "r15",
		})
	}
}

/// Convert usize to Register.
impl TryFrom<usize> for Register {
	type Error = &'static str;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0..=15 => Ok(unsafe { std::mem::transmute(value) }),
			_ => Err("Ran out of registers!"),
		}
	}
}

/// X86_64 assembly operand (register or immediate).
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Operand {
	Register(Register),
	Immediate(usize),
}

impl std::fmt::Display for Operand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operand::Register(register) => write!(f, "{}", register),
			Operand::Immediate(value) => write!(f, "{}", value),
		}
	}
}

/// X86_64 assembly jump condition.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Jump {
	Zero,
	NotZero,
	Less,
	LessEquals,
	Greater,
	GreaterEquals,
}

impl std::fmt::Display for Jump {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Jump::Zero => "jz",
			Jump::NotZero => "jnz",
			Jump::Less => "jl",
			Jump::LessEquals => "jle",
			Jump::Greater => "jg",
			Jump::GreaterEquals => "jge",
		})
	}
}

/// Convert LIRConditionType to Jump.
impl From<crate::lir::node::LIRConditionType> for Jump {
	fn from(value: crate::lir::node::LIRConditionType) -> Self {
		use crate::lir::node::LIRConditionType;
		match value {
			LIRConditionType::Equals => Jump::Zero,
			LIRConditionType::NotEquals => Jump::NotZero,
			LIRConditionType::LessThan => Jump::Less,
			LIRConditionType::LessThanOrEquals => Jump::LessEquals,
			LIRConditionType::GreaterThan => Jump::Greater,
			LIRConditionType::GreaterThanOrEquals => Jump::GreaterEquals,
		}
	}
}

/// X86_64 assembly instruction.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
	/// Set a register to a value.
	MOV {
		dest: Register,
		a: Operand,
	},
	/// Call a module.
	CALL {
		module_id: usize,
	},
	/// Jump unconditionally to a module.
	JMP {
		module_id: usize,
	},
	/// Jump conditionally to a module.
	Jcc {
		kind: Jump,
		module_id: usize,
	},
	/// Compare 2 values (usually in preparation for a jump.)
	CMP {
		a: Operand,
		b: Operand,
	},
}
