use crate::evaluator::operate;
use crate::lexer::Token;
use crate::Value;

pub(crate) const BYTECODE_SCHEMA_VERSION: u32 = 1;
pub(crate) const DEFAULT_STEP_LIMIT: usize = 100_000;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Instruction {
    Const(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Halt,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Program {
    pub(crate) schema_version: u32,
    pub(crate) instructions: Vec<Instruction>,
}

impl Program {
    pub(crate) fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            schema_version: BYTECODE_SCHEMA_VERSION,
            instructions,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Vm {
    stack: Vec<Value>,
    pc: usize,
    steps: usize,
    step_limit: usize,
}

impl Vm {
    pub(crate) fn new(step_limit: usize) -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            steps: 0,
            step_limit,
        }
    }

    pub(crate) fn run(mut self, program: &Program) -> Result<Value, String> {
        if program.schema_version != BYTECODE_SCHEMA_VERSION {
            return Err(format!(
                "bytecode schema {} is unsupported; expected {}",
                program.schema_version, BYTECODE_SCHEMA_VERSION
            ));
        }
        loop {
            if self.steps >= self.step_limit {
                return Err(format!(
                    "bytecode step limit exceeded: maximum is {}",
                    self.step_limit
                ));
            }
            let instruction = program.instructions.get(self.pc).ok_or_else(|| {
                format!(
                    "bytecode program ended without Halt at instruction {}",
                    self.pc
                )
            })?;
            self.steps += 1;
            self.pc += 1;
            match instruction {
                Instruction::Const(value) => self.stack.push(value.clone()),
                Instruction::Add => self.binary(Token::Plus)?,
                Instruction::Subtract => self.binary(Token::Minus)?,
                Instruction::Multiply => self.binary(Token::Star)?,
                Instruction::Divide => self.binary(Token::Slash)?,
                Instruction::Remainder => self.binary(Token::Percent)?,
                Instruction::Halt => {
                    return self
                        .stack
                        .pop()
                        .ok_or_else(|| "bytecode Halt requires one stack value".into())
                }
            }
        }
    }

    fn binary(&mut self, operator: Token) -> Result<(), String> {
        let right = self
            .stack
            .pop()
            .ok_or_else(|| "bytecode binary operation requires a right operand".to_string())?;
        let left = self
            .stack
            .pop()
            .ok_or_else(|| "bytecode binary operation requires a left operand".to_string())?;
        self.stack.push(operate(left, operator, right)?);
        Ok(())
    }
}

pub(crate) fn run(program: &Program) -> Result<Value, String> {
    Vm::new(DEFAULT_STEP_LIMIT).run(program)
}

#[cfg(test)]
mod tests {
    use super::{run, Instruction, Program, Vm, BYTECODE_SCHEMA_VERSION};
    use crate::Value;

    #[test]
    fn evaluates_numeric_expression_deterministically() {
        let program = Program::new(vec![
            Instruction::Const(Value::Number(2)),
            Instruction::Const(Value::Number(3)),
            Instruction::Multiply,
            Instruction::Const(Value::Number(4)),
            Instruction::Add,
            Instruction::Halt,
        ]);
        assert_eq!(
            run(&program).expect("VM should evaluate"),
            Value::Number(10)
        );
    }

    #[test]
    fn rejects_invalid_stack_shape_without_panicking() {
        let program = Program::new(vec![Instruction::Add, Instruction::Halt]);
        let error = run(&program).expect_err("missing operands must fail");
        assert!(error.contains("right operand"));
    }

    #[test]
    fn enforces_schema_and_step_limits() {
        let mut program = Program::new(vec![Instruction::Const(Value::Number(1))]);
        program.schema_version = BYTECODE_SCHEMA_VERSION + 1;
        assert!(run(&program)
            .expect_err("schema must fail")
            .contains("schema"));

        let program = Program::new(vec![Instruction::Const(Value::Number(1))]);
        assert!(Vm::new(1)
            .run(&program)
            .expect_err("missing Halt should hit the step bound")
            .contains("step limit"));
    }

    #[test]
    fn rejects_halt_without_a_result() {
        let program = Program::new(vec![Instruction::Halt]);
        assert!(run(&program)
            .expect_err("empty stack Halt must fail")
            .contains("one stack value"));
    }
}
