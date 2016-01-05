

pub enum Instruction {
    NOP,
}

pub struct State {
    registers: u32
}

fn eval(state: State, instruction: &Instruction) -> State {
    match *instruction {
        _ => State { registers: state.registers },
    }
}

#[test]
fn it_works() {
    let instructions = vec![Instruction::NOP];
    instructions.iter().fold(State { registers: 1 }, eval);
}
