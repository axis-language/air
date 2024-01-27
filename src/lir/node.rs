/* Declarations */
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum LIROperand {
	Immediate(usize),
	Variable(usize),
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum LIRConditionType {
	Equals,
	NotEquals,
	LessThan,
	LessThanOrEquals,
	GreaterThan,
	GreaterThanOrEquals,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum LIRNode {
	/* Control */
	Set {
		dest: usize,
		a: LIROperand,
	},
	Call {
		module_id: usize,
	},
	Branch {
		condition: LIRConditionType,
		a: LIROperand,
		b: LIROperand,
		module_id: usize,
	}
}