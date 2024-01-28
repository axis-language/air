//! AIR (Axis Intermediate Representation) is a low-level language that compiles to many targets.

/* Modules */
pub mod arch;
pub mod lir;

/* Import */
use std::collections::HashMap;
use std::io::Write;
use arch::Architecture;
use lir::{
    Project,
    node::*,
};

/* Main */
fn main() {
    let lir_code = Project {
        main: vec![
            LIRNode::Call { module_id: 0 },
        ],
        functions: vec![
            vec![
                LIRNode::Set { dest: 0, a: LIROperand::Immediate(42) },
            ],
        ],
    };

    let assembly_code = arch::lower_lir_to_assembly::<arch::x86_64::X86_64>(
        &lir_code,
        &arch::LowerToAssemblyContext {
            register_allocation: HashMap::from([(0,0), (1,1), (2,2)]),
        },
    ).expect("Failed to lower to assembly!");

    let assembly_file_string = arch::x86_64::X86_64::lower_assembly_to_file(
        &assembly_code,
        &arch::LowerToAssemblyFileContext {},
    ).expect("Failed to generate assembly file!");

    let mut assembly_file = std::fs::File::create("target/project.asm").expect("Failed to create assembly file!");
    assembly_file.write(assembly_file_string.as_bytes()).expect("Failed to write to assembly file!");
}
