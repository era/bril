use serde::{Deserialize, Serialize};
use serde_json::Result; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    /// A list of functions declared in the program
    pub functions: Vec<Function>,
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#function>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    /// Any arguments the function accepts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<Argument>,
    /// The instructions of this function
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instrs: Vec<Code>,
    /// The name of the function
    pub name: String,
    /// The position of this function in the original source code
    #[cfg(feature = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<Position>,
    /// The possible return type of this function
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<Type>,
} 

/// Example: a : int
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argument {
    /// a
    pub name: String,
    #[serde(rename = "type")]
    /// int
    pub arg_type: Type,
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#function>
/// Code is a Label or an Instruction
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Code {
    /// <https://capra.cs.cornell.edu/bril/lang/syntax.html#label>
    Label {
        /// The name of the label
        label: String,
        /// Where the label is located in source code
        #[cfg(feature = "position")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pos: Option<Position>,
    },
    /// <https://capra.cs.cornell.edu/bril/lang/syntax.html#instruction>
    Instruction(Instruction),
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#instruction>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Instruction {
    /// <https://capra.cs.cornell.edu/bril/lang/syntax.html#constant>
    Constant {
        /// destination variable
        dest: String,
        /// "const"
        op: ConstOps,
        #[cfg(feature = "position")]
        /// The source position of the instruction if provided
        #[serde(skip_serializing_if = "Option::is_none")]
        pos: Option<Position>,
        /// Type of variable
        #[serde(rename = "type")]
        const_type: Type,
        /// The literal being stored in the variable
        value: Literal,
    },
    /// <https://capra.cs.cornell.edu/bril/lang/syntax.html#value-operation>
    Value {
        /// List of variables as arguments
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
        /// destination variable
        dest: String,
        /// List of strings as function names
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        funcs: Vec<String>,
        /// List of strings as labels
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        labels: Vec<String>,
        /// Operation being executed
        op: ValueOps,
        /// The source position of the instruction if provided
        #[cfg(feature = "position")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pos: Option<Position>,
        /// Type of variable
        #[serde(rename = "type")]
        op_type: Type,
    },
    /// <https://capra.cs.cornell.edu/bril/lang/syntax.html#effect-operation>
    Effect {
        /// List of variables as arguments
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
        /// List of strings as function names
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        funcs: Vec<String>,
        /// List of strings as labels
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        labels: Vec<String>,
        /// Operation being executed
        op: EffectOps,
        /// The source position of the instruction if provided
        #[cfg(feature = "position")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pos: Option<Position>,
    },
}

#[cfg(feature = "position")]
impl Instruction {
    /// A helper function to extract the position value if it exists from an instruction
    #[must_use]
    pub const fn get_pos(&self) -> Option<Position> {
        match self {
            Instruction::Constant { pos, .. }
            | Instruction::Value { pos, .. }
            | Instruction::Effect { pos, .. } => *pos,
        }
    }
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#constant>
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConstOps {
    /// "const"
    #[serde(rename = "const")]
    Const,
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#effect-operation>
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum EffectOps {
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#control>
    #[serde(rename = "jmp")]
    Jump,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#control>
    #[serde(rename = "br")]
    Branch,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#control>
    Call,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#control>
    #[serde(rename = "ret")]
    Return,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#miscellaneous>
    Print,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#miscellaneous>
    Nop,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#operations>
    #[cfg(feature = "memory")]
    Store,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#operations>
    #[cfg(feature = "memory")]
    Free,
    /// <https://capra.cs.cornell.edu/bril/lang/spec.html#operations>
    #[cfg(feature = "speculate")]
    Speculate,
    /// <https://capra.cs.cornell.edu/bril/lang/spec.html#operations>
    #[cfg(feature = "speculate")]
    Commit,
    /// <https://capra.cs.cornell.edu/bril/lang/spec.html#operations>
    #[cfg(feature = "speculate")]
    Guard,
}


/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#value-operation>
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ValueOps {
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#arithmetic>
    Add,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#arithmetic>
    Sub,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#arithmetic>
    Mul,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#arithmetic>
    Div,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#comparison>
    Eq,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#comparison>
    Lt,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#comparison>
    Gt,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#comparison>
    Le,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#comparison>
    Ge,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#logic>
    Not,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#logic>
    And,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#logic>
    Or,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#control>
    Call,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#miscellaneous>
    Id,
    /// <https://capra.cs.cornell.edu/bril/lang/ssa.html#operations>
    #[cfg(feature = "ssa")]
    Phi,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fadd,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fsub,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fmul,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fdiv,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Feq,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Flt,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fgt,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fle,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#operations>
    #[cfg(feature = "float")]
    Fge,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#operations>
    #[cfg(feature = "memory")]
    Alloc,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#operations>
    #[cfg(feature = "memory")]
    Load,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#operations>
    #[cfg(feature = "memory")]
    PtrAdd,
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#type>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#types>
    Int,
    /// <https://capra.cs.cornell.edu/bril/lang/core.html#types>
    Bool,
    /// <https://capra.cs.cornell.edu/bril/lang/float.html#types>
    #[cfg(feature = "float")]
    Float,
    /// <https://capra.cs.cornell.edu/bril/lang/memory.html#types>
    #[cfg(feature = "memory")]
    #[serde(rename = "ptr")]
    Pointer(Box<Self>),
}


/// A JSON number/value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Literal {
    /// Integers
    Int(i64),
    /// Booleans
    Bool(bool),
    /// Floating Points
    #[cfg(feature = "float")]
    Float(f64),
}



impl Literal {
    /// A helper function to get the type of literal values
    #[must_use]
    pub const fn get_type(&self) -> Type {
        match self {
            Literal::Int(_) => Type::Int,
            Literal::Bool(_) => Type::Bool,
            #[cfg(feature = "float")]
            Literal::Float(_) => Type::Float,
        }
    }
}

/// <https://capra.cs.cornell.edu/bril/lang/syntax.html#source-positions>
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Column
    pub col: u64,
    /// Row
    pub row: u64,
}
