#[derive(Debug, Clone, Default, PartialEq)]
pub struct Line {
    pub num: i32,
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Variable {
    name: String,
    default_value: i16,
    constant: bool,
    address: u16,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VariableGenerator {
    address: u16,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Reference {
    var: Variable,
    reference_type: ReferenceType,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Program {
    instructions: Vec<SimpleInstruction>,
    variables: Vec<Variable>,
    var_gen: VariableGenerator,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ReferenceType {
    #[default]
    Direct,
    Pointer,
    Offset(Variable),
    Address,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkipcondType {
    GreaterThanZero,
    LessThanZero,
    Zero,
}

type Label = String;

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleInstruction {
    Add(Reference), Subt(Reference), Store(Reference), Load(Reference), Jns(Reference),
    Skipcond(SkipcondType),
    Jump(Label),
    Clear, Input, Output, Halt,
}