pub struct Instruction {
  pub op_code: OpCode,
  pub first_arg_position: usize,
  pub second_arg_position: usize,
  pub result_position: usize,
}

pub enum OpCode {
  Halt = 99,
  Add = 1,
  Multiply = 2,
}

impl OpCode {
  pub fn from_i32(value: i32) -> OpCode {
    match value {
      99 => OpCode::Halt,
      1 => OpCode::Add,
      2 => OpCode::Multiply,
      _ => panic!("Unknown value: {}", value),
    }
  }
}
