use super::instruction::*;
use std::collections::BTreeMap;

pub type ID = i32;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Move(i32),
    Add(i32),
    Write,
    Read,
    JumpZero(ID, ID),    // id, id to jump
    JumpNonZero(ID, ID), // id, id to jump
    Clear,
    Copy(i32),
}

pub fn is_command(c: char) -> bool {
    decode(c).is_some()
}

pub fn decode(c: char) -> Option<Command> {
    match c {
        '<' => Some(Command::Move(-1)),
        '>' => Some(Command::Move(1)),
        '+' => Some(Command::Add(1)),
        '-' => Some(Command::Add(-1)),
        '.' => Some(Command::Write),
        ',' => Some(Command::Read),
        '[' => Some(Command::JumpZero(0, 0)),
        ']' => Some(Command::JumpNonZero(0, 0)),
        _ => None,
    }
}

pub fn parse(s: &str) -> Option<Vec<Command>> {
    let mut next_id = 0 as ID;
    let mut jump_id = Vec::<ID>::new();
    let mut jump_pos = BTreeMap::<ID, usize>::new();
    let mut cs = s.chars().filter_map(decode).collect::<Vec<Command>>();

    for i in 0..cs.len() {
        cs[i] = match cs[i].clone() {
            Command::JumpZero(_, _) => {
                next_id += 1;
                jump_id.push(next_id);
                jump_pos.insert(next_id, i);
                Command::JumpZero(next_id, 0)
            }
            Command::JumpNonZero(_, _) => {
                next_id += 1;
                match jump_id.pop() {
                    Some(to_id) => {
                        if !jump_pos.contains_key(&to_id) {
                            return None;
                        }
                        cs[jump_pos[&to_id]] = Command::JumpZero(to_id, next_id);
                        jump_pos.remove(&to_id);
                        Command::JumpNonZero(next_id, to_id)
                    }
                    None => return None,
                }
            }
            c => c,
        }
    }

    if jump_pos.len() > 0 {
        return None;
    }

    Some(cs)
}

pub fn compile(cmds: Vec<Command>) -> Vec<Instruction> {
    let mut pos = BTreeMap::<ID, usize>::new();

    for i in 0..cmds.len() {
        let c = cmds[i].clone();
        match c {
            Command::JumpZero(id, _) => {
                pos.insert(id, i);
            }
            Command::JumpNonZero(id, _) => {
                pos.insert(id, i);
            }
            _ => {}
        }
    }

    cmds.iter()
        .map(|c| match *c {
            Command::Add(x) => Instruction::Add(x),
            Command::Move(x) => Instruction::Move(x),
            Command::Write => Instruction::Write,
            Command::Read => Instruction::Read,
            Command::Clear => Instruction::Clear,
            Command::Copy(x) => Instruction::Copy(x),
            Command::JumpZero(_, id_to) => Instruction::JumpZero(pos[&id_to]),
            Command::JumpNonZero(_, id_to) => Instruction::JumpNonZero(pos[&id_to]),
        }).collect()
}
