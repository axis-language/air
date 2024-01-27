/* Modules */
pub mod x86_64;

/* Imports */
use std::collections::HashMap;
use crate::lir::{
	Module, Project,
	node::LIRNode
};

/* Declarations */
#[derive(Debug, PartialEq, Clone)]
pub struct LowerToAssemblyContext {
	pub register_allocation: HashMap<usize, usize>,
}

impl LowerToAssemblyContext {
	pub fn get_allocated_register_from_ir_variable(&self, ssa_variable: usize) -> Result<usize, &'static str> {
		self.register_allocation.get(&ssa_variable).ok_or("Tried to use variable that was not mapped to a register!").copied()
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct LowerToAssemblyFileContext {

}

/// A targetable architecture.
pub trait Architecture {
	/// The instruction of the targeted architecture.
	type Instruction;

	/// Turn a LIRNode to a module of architecture instructions.
	#[fehler::throws(&'static str)]
	fn lower_lir_node_to_assembly_instruction(node: &LIRNode, context: &LowerToAssemblyContext) -> Module<Self::Instruction>;

	/// Turn an architecture instruction to an assembly string.
	fn lower_assembly_instruction_to_assembly_file_string(instruction: &Self::Instruction, context: &LowerToAssemblyFileContext) -> String;

	#[fehler::throws(&'static str)]
	fn lower_assembly_to_file(assembly: &Project<Self::Instruction>, context: &LowerToAssemblyFileContext) -> String;
}

#[fehler::throws(&'static str)]
pub fn lower_lir_to_assembly<ARCH: Architecture>(lir: &Project<LIRNode>, context: &LowerToAssemblyContext) -> Project<ARCH::Instruction> {
	let mut res = Project {
		main: vec![],
		functions: vec![],
	};

	for instruction in &lir.main {
		res.main.extend(ARCH::lower_lir_node_to_assembly_instruction(&instruction, context)?);
	}

	for (function_index, function) in lir.functions.iter().enumerate() {
		res.functions.push(vec![]);
		for instruction in function {
			res.functions[function_index].extend(ARCH::lower_lir_node_to_assembly_instruction(&instruction, context)?);
		}
	}

	res
}