use command::*;

pub fn compact(cmds: Vec<Command>) -> Vec<Command> {
    let mut compacted = Vec::<Command>::new();

    for c in cmds {
        match c {
            Command::Add(x) => {
                if let Some(Command::Add(ref mut y)) = compacted.last_mut() {
                    *y += x;
                    continue;
                } else {
                    compacted.push(c);
                }
            }
            Command::Move(x) => {
                if let Some(Command::Move(ref mut y)) = compacted.last_mut() {
                    *y += x;
                    continue;
                } else {
                    compacted.push(c);
                }
            }
            _ => compacted.push(c),
        }
    }

    compacted
}

pub fn remove_noop(cmds: Vec<Command>) -> Vec<Command> {
    cmds.into_iter()
        .filter(|x| match x {
            Command::Add(0) => false,
            Command::Move(0) => false,
            _ => true,
        }).collect()
}
