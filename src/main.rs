pub mod arch;
pub mod lir;

use std::collections::HashMap;

pub type FunctionID = String;

#[fehler::throws(&'static str)]
fn main() {
	let mut lir = lir::Program::new();
	lir.set_main(vec![
		lir::Node::Set { destination: 0, a: lir::Operand::Data(42) }
	]);

	let assembly = arch::x86_64::compile::compile_lir_program(
		&lir,
		&arch::x86_64::compile::CompilationContext {
			register_map: HashMap::from([
				(0, arch::x86_64::Register::RAX),
			])
		},
	)?;

	println!("{}", assembly.to_string());
}
