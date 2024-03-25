use std::collections::HashMap;
use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct CompilationContext {
	pub register_map: HashMap<lir::Register, Register>,
}

impl CompilationContext {
	pub fn new() -> Self {
		Self {
			register_map: HashMap::new(),
		}
	}

	#[fehler::throws(&'static str)]
	pub fn get_register(&self, register: lir::Register) -> Register {
		self.register_map.get(&register).ok_or("Failed to map register!")?.clone()
	}
}

#[fehler::throws(&'static str)]
pub fn compile_lir_operand(operand: &lir::Operand, context: &CompilationContext) -> Operand {
	match operand {
		lir::Operand::Register(register) => Operand::Register(context.get_register(*register)?),
		lir::Operand::Data(data) => Operand::Immediate(*data),
	}
}



#[fehler::throws(&'static str)]
pub fn compile_lir_node(node: &lir::Node, context: &CompilationContext) -> Block {
	match node {
		/* Inline */
		lir::Node::InlineString { string } => todo!(),
		lir::Node::InlineByte { data } => todo!(),

		/* Control */
		lir::Node::Call { function } => todo!(),
		lir::Node::Branch { condition, function } => todo!(),

		lir::Node::Set { destination, a } => vec![ Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? } ],
		lir::Node::ReadMemory { destination, a } => todo!(),
		lir::Node::WriteMemory { destination, a } => todo!(),

		/* Arithmetic */
		lir::Node::Add { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Add { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Subtract { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Sub { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Multiply { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Mul { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Divide { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Div { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		
		/* Logic */
		lir::Node::And { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::And { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Or { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Or { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Xor { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Xor { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::Not { destination, a } => vec![ 
			Node::Not { destination: context.get_register(*destination)? }
		],
		lir::Node::LogicalShiftLeft { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Shl { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
		lir::Node::LogicalShiftRight { destination, a, b } => vec![ 
			Node::Mov { destination: context.get_register(*destination)?, a: compile_lir_operand(a, context)? },
			Node::Shr { destination: context.get_register(*destination)?, a: compile_lir_operand(b, context)? }
		],
	}
}

#[fehler::throws(&'static str)]
pub fn compile_lir_program(program: &lir::Program, context: &CompilationContext) -> Program {
	let mut result = Program::new();

	for node in &program.main {
		result.main.extend(compile_lir_node(&node, context)?)
	}
	
	result
}