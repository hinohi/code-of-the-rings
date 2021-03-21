use std::{fmt, io::stdin};

const TAPE_SIZE: usize = 30;
const CELL_SIZE: u8 = 27;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Add,
    Sub,
    Right,
    Left,
    Put,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmd::Add => f.write_str("+"),
            Cmd::Sub => f.write_str("-"),
            Cmd::Right => f.write_str(">"),
            Cmd::Left => f.write_str("<"),
            Cmd::Put => f.write_str("."),
        }
    }
}

fn closest_u8_change(now: u8, target: u8) -> (u8, Cmd) {
    let a = if now <= target {
        (target - now, Cmd::Add)
    } else {
        (now - target, Cmd::Sub)
    };
    let b = if now <= target {
        (now + CELL_SIZE - target, Cmd::Sub)
    } else {
        (CELL_SIZE - now + target, Cmd::Add)
    };
    if a.0 <= b.0 {
        a
    } else {
        b
    }
}

fn get_target() -> Vec<u8> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let target_str = buf.trim_matches('\n');
    let map = vec![
        ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    target_str
        .chars()
        .map(|c| map.binary_search(&c).unwrap() as u8)
        .collect()
}

fn in_one_u8(target: &[u8]) -> Vec<Cmd> {
    let mut cmds = Vec::new();
    let mut now = 0;
    for &c in target.iter() {
        let (n, cmd) = closest_u8_change(now, c);
        for _ in 0..n {
            cmds.push(cmd)
        }
        cmds.push(Cmd::Put);
        now = c;
    }
    cmds
}

fn main() {
    let target = get_target();
    for cmd in in_one_u8(&target) {
        print!("{}", cmd);
    }
    println!()
}
