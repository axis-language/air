use super::*;

impl ToString for Register {
	fn to_string(&self) -> String {
		match self {
			Register::RAX => "rax",
			Register::RBX => "rbx",
			Register::RCX => "rcx",
			Register::RDX => "rdx",

			Register::RSI => "rsi",
			Register::RDI => "rdi",
			Register::RSP => "rsp",
			Register::RBP => "rbp",

			Register::R8 => "r8",
			Register::R9 => "r9",
			Register::R10 => "r10",
			Register::R11 => "r11",
			Register::R12 => "r12",
			Register::R13 => "r13",
			Register::R14 => "r14",
			Register::R15 => "r15",

			Register::RIP => "rip",
		}.to_string()
	}
}

impl ToString for Operand {
	fn to_string(&self) -> String {
		match self {
			Operand::Register(register) => register.to_string(),
			Operand::Immediate(immediate) => immediate.to_string(),
		}
	}
}

impl ToString for Node {
	fn to_string(&self) -> String {
		match self {
			/* Inline */
			Node::InlineString { string } => string.clone(),
			Node::InlineByte { data } => format!("db {}", data),

			/* Control */
			Node::Call { function } => format!("call {}", function),

			Node::Mov { destination, a } => format!("mov {}, {}", destination.to_string(), a.to_string()),
			
			Node::Jmp { target } => format!("jmp {}", target),

			/* Arithmetic */
			Node::Add { destination, a } => format!("add {}, {}", destination.to_string(), a.to_string()),
			Node::Sub { destination, a } => format!("sub {}, {}", destination.to_string(), a.to_string()),
			Node::Mul { destination, a } => format!("mul {}, {}", destination.to_string(), a.to_string()),
			Node::Div { destination, a } => format!("div {}, {}", destination.to_string(), a.to_string()),

			/* Logic */
			Node::And { destination, a } => format!("and {}, {}", destination.to_string(), a.to_string()),
			Node::Or { destination, a } => format!("or {}, {}", destination.to_string(), a.to_string()),
			Node::Xor { destination, a } => format!("xor {}, {}", destination.to_string(), a.to_string()),
			Node::Not { destination } => format!("not {}", destination.to_string()),
			Node::Shl { destination, a } => format!("shl {}, {}", destination.to_string(), a.to_string()),
			Node::Shr { destination, a } => format!("shr {}, {}", destination.to_string(), a.to_string()),
		}
	}
}

impl ToString for Program {
	fn to_string(&self) -> String {
		let mut result = String::new();

		result.push_str("SECTION .DATA\n\n");
		result.push_str("SECTION .TEXT\n\n");

		result.push_str("global main\n");
		result.push_str("main:\n\t");
		for node in &self.main {
			result.push_str(&node.to_string());
			result.push_str("\n\t");
		}
		result.push_str("jmp end\n\n");

		for (function_id, function_block) in &self.functions {
			result.push_str(&format!("{}:\n\t", function_id));
			for node in function_block {
				result.push_str(&node.to_string());
				result.push_str("\n\t");
			}
			result.push_str("ret\n\n");
		}
		
		result.push_str("end:\n\t");
		result.push_str("ret\n");

		result
	}
}

