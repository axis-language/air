/* Modules */
pub mod instruction;

/* Imports */
use instruction::*;
use crate::arch::*;
use crate::lir::{
	Module,
	node::*,
};

/* Declarations */
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct X86_64;

#[fehler::throws(&'static str)]
fn ir_operand_to_x86_64_operand(operand: &LIROperand, context: &LowerToAssemblyContext) -> Operand {
	match operand {
		LIROperand::Variable(id) => Operand::Register(
			context.get_allocated_register_from_ir_variable(*id)?.try_into()?
		),
		LIROperand::Immediate(value) => Operand::Immediate(*value),
	}
}

impl Architecture for X86_64 {
	type Instruction = Instruction;

	#[fehler::throws(&'static str)]
	fn lower_lir_node_to_assembly_instruction(node: &LIRNode, context: &LowerToAssemblyContext) -> Module<Self::Instruction> {
		match node {
			LIRNode::Set { dest, a } => vec![
				Instruction::MOV {
					dest: context.get_allocated_register_from_ir_variable(*dest)?.try_into()?,
					a: ir_operand_to_x86_64_operand(a, context)?,
				},
			],
			LIRNode::Call { module_id } => vec![
				Instruction::CALL {
					module_id: *module_id,
				},
			],
			LIRNode::Branch { condition, a, b, module_id } => vec![
				Instruction::CMP {
					a: ir_operand_to_x86_64_operand(a, context)?,
					b: ir_operand_to_x86_64_operand(b, context)?,
				},
				Instruction::Jcc {
					kind: (*condition).into(),
					module_id: *module_id,
				},
			],
		}
	}

	fn lower_assembly_instruction_to_assembly_file_string(instruction: &Self::Instruction, context: &LowerToAssemblyFileContext) -> String {
		match instruction {
			Instruction::MOV { dest, a } => format!("mov {}, {}", dest, a),
			Instruction::CALL { module_id } => format!("call function_{}", module_id),
			Instruction::JMP { module_id } => format!("jmp function_{}", module_id),
			Instruction::Jcc { kind, module_id } => format!("{} function_{}", kind, module_id),
			Instruction::CMP { a, b } => format!("cmp {}, {}", a, b),
		}
	}

	#[fehler::throws(&'static str)]
	fn lower_assembly_to_file(assembly: &Project<Self::Instruction>, context: &LowerToAssemblyFileContext) -> String {
		// Allocate 8 bytes for each instruction (should be enough)
		let mut res = String::with_capacity((assembly.main.len() * 8) + (assembly.functions.iter().map(|f| f.len() * 8).sum::<usize>()));

		res.push_str("section .data\n\n");
		res.push_str("section .text\n\n");
		res.push_str("global main\n\n");
		res.push_str("main:\n");

		for instruction in &assembly.main {
			res.push_str(&format!("\t{}\n", Self::lower_assembly_instruction_to_assembly_file_string(&instruction, context)));
		}
		res.push_str("\tjmp end\n\n");

		for (function_index, function) in assembly.functions.iter().enumerate() {
			res.push_str(&format!("function_{}:\n", function_index));

			for instruction in function {
				res.push_str(&format!("\t{}\n", Self::lower_assembly_instruction_to_assembly_file_string(&instruction, context)));
			}

			res.push_str("\tret\n\n");
		}

		res.push_str("end:\n");
		res.push_str("\tret\n");
		res
	}
}