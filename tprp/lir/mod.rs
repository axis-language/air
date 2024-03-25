/* Modules */

pub mod node;

/* Declarations */
/// A collection of instructions (a function body, a branch path, etc.)
pub type Module<T> = Vec<T>;

/// The units that get compiled to assembly.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Project<T> {
	pub main: Module<T>,
	pub functions: Vec<Module<T>>,
}
