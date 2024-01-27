/* Declarations */
/// An X86_64 register.
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
			Register::RAX => "rax".to_string(),
			Register::RBX => "rbx".to_string(),
			Register::RCX => "rcx".to_string(),
			Register::RDX => "rdx".to_string(),
			Register::RSP => "rsp".to_string(),
			Register::RBP => "rbp".to_string(),
			Register::RDI => "rdi".to_string(),
			Register::RSI => "rsi".to_string(),
			Register::R8 => "r8".to_string(),
			Register::R9 => "r9".to_string(),
			Register::R10 => "r10".to_string(),
			Register::R11 => "r11".to_string(),
			Register::R12 => "r12".to_string(),
			Register::R13 => "r13".to_string(),
			Register::R14 => "r14".to_string(),
			Register::R15 => "r15".to_string(),
		})
	}
}

// TODO: tidy/remove
impl TryFrom<usize> for Register {
	type Error = &'static str;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::RAX),
			1 => Ok(Self::RBX),
			2 => Ok(Self::RCX),
			3 => Ok(Self::RSP),
			4 => Ok(Self::RBP),
			5 => Ok(Self::RDI),
			6 => Ok(Self::RSI),
			7 => Ok(Self::RDX),
			8 => Ok(Self::R8),
			9 => Ok(Self::R9),
			10 => Ok(Self::R10),
			11 => Ok(Self::R11),
			12 => Ok(Self::R12),
			13 => Ok(Self::R13),
			14 => Ok(Self::R14),
			15 => Ok(Self::R15),
			_ => Err("Ran out of registers!")
		}
	}
}

/// An X86_64 assembly operand.
/// Only accepts registers or immediates, no memory or relative values.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Operand {
	Register(Register),
	Immediate(usize),
}

impl std::fmt::Display for Operand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Operand::Register(register) => format!("{}", register),
			Operand::Immediate(value) => format!("{}", value),
		})
	}
}

/// An X86_64 assembly jump (condition.)
/// Used to combine `CMP` and `Jcc` instructions.
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

// TODO: tidy/remove
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

/// An X86_64 assembly instruction.
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