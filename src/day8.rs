use crate::util;
use crate::make_map;
use std::collections::HashMap;

pub fn run_easy() {
    let program = parse_program();
    let mut state = make_program_state();

    let mut executed: Vec<bool> = program.iter().map(|_| false).collect();

    while state.pc < program.len() {
        if executed[state.pc] {
            break;
        }
        executed[state.pc] = true;

        let (_, insn) = &program[state.pc];
        insn.execute(&mut state);
        insn.add_pc(&mut state);
    }

    println!("{}", state.acc);
}

pub fn run_hard() {
    let original_program = parse_program();
    let programs: Vec<Program> = (0..original_program.len())
        .filter(|index| original_program[*index].0 == "jmp" ||original_program[*index].0 == "nop")
        .map(|index| {
            let mut program: Program = original_program.iter().map(|(opcode, insn)| (opcode.clone(), insn.copy_to_box())).collect();
            let (opcode, insn) = &original_program[index];
            if opcode == "jmp" {
                program[index] = ("nop".to_owned(), make_nop(insn.get_operand()))
            } else {
                program[index] = ("jmp".to_owned(), make_jmp(insn.get_operand()))
            }
            program
        })
        .collect();
    let mut states: Vec<_> = programs.iter().map(|_| make_program_state()).collect();
    'outer: loop {
        for (program, mut state) in programs.iter().zip(states.iter_mut()) {
            if state.pc >= program.len() {
                println!("{}", state.acc);
                break 'outer;
            }
            let (_, insn) = &program[state.pc];
            insn.execute(&mut state);
            insn.add_pc(&mut state);
        }
    }
}

fn parse_program() -> Program {
    let opcodes = make_opcodes();
    return util::get_input_lines()
        .map(|line| {
            let (opcode, rest) = line.split_at(line.find(" ").unwrap());
            let insn = opcodes.get(opcode).unwrap()(rest.strip_prefix(" ").unwrap().parse().unwrap());
            (opcode.to_owned(), insn)
        })
        .collect();
}

fn make_opcodes() -> HashMap<&'static str, fn(i32) -> Box<dyn Insn>> {
    make_map!{
        "nop" => make_nop as fn(i32) -> Box<dyn Insn>,
        "acc" => make_acc as fn(i32) -> Box<dyn Insn>,
        "jmp" => make_jmp as fn(i32) -> Box<dyn Insn>
    }
}

trait Insn {
    fn get_operand(&self) -> i32;
    fn execute(&self, state: &mut ProgramState);
    fn add_pc(&self, state: &mut ProgramState) {
        state.pc += 1;
    }
    fn copy_to_box(&self) -> Box<dyn Insn>;
}

struct NopInsn {
    operand: i32
}
impl Insn for NopInsn {
    fn get_operand(&self) -> i32 {
        self.operand
    }
    fn execute(&self, _state: &mut ProgramState) {
    }
    fn copy_to_box(&self) -> Box<dyn Insn> {
        Box::new(NopInsn {
            operand: self.operand
        })
    }
}
fn make_nop(operand: i32) -> Box<dyn Insn> {
    Box::new(NopInsn {
        operand
    })
}

struct AccInsn {
    amt: i32
}
impl Insn for AccInsn {
    fn get_operand(&self) -> i32 {
        self.amt
    }
    fn execute(&self, state: &mut ProgramState) {
        state.acc += self.amt;
    }
    fn copy_to_box(&self) -> Box<dyn Insn> {
        Box::new(AccInsn {
            amt: self.amt
        })
    }
}
fn make_acc(amt: i32) -> Box<dyn Insn> {
    Box::new(AccInsn {
        amt
    })
}

struct JmpInsn {
    by: i32
}
impl Insn for JmpInsn {
    fn get_operand(&self) -> i32 {
        self.by
    }
    fn execute(&self, state: &mut ProgramState) {
        state.pc = (state.pc as i32 + self.by) as usize;
    }
    fn add_pc(&self, _state: &mut ProgramState) {
    }
    fn copy_to_box(&self) -> Box<dyn Insn> {
        Box::new(JmpInsn {
            by: self.by
        })
    }
}
fn make_jmp(by: i32) -> Box<dyn Insn> {
    Box::new(JmpInsn {
        by
    })
}

type Program = Vec<(String, Box<dyn Insn>)>;

struct ProgramState {
    pc: usize,
    acc: i32
}
fn make_program_state() -> ProgramState {
    ProgramState {
        pc: 0,
        acc: 0
    }
}
