use std::{fmt, io::stdin};

const TAPE_SIZE: usize = 30;
const CELL_SIZE: u8 = 27;

#[derive(Debug, Copy, Clone)]
enum Cmd {
    Add(u8),
    Sub(u8),
    Right(usize),
    Left(usize),
    Put,
}

impl Cmd {
    fn len(self) -> usize {
        match self {
            Cmd::Add(n) => n as usize,
            Cmd::Sub(n) => n as usize,
            Cmd::Right(n) => n,
            Cmd::Left(n) => n,
            Cmd::Put => 1,
        }
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! n_times {
            ($f:ident, $n:expr, $c:expr) => {
                for _ in 0..$n {
                    $f.write_str($c)?;
                }
            };
        }
        match self {
            Cmd::Add(n) => n_times!(f, *n, "+"),
            Cmd::Sub(n) => n_times!(f, *n, "-"),
            Cmd::Right(n) => n_times!(f, *n, ">"),
            Cmd::Left(n) => n_times!(f, *n, "<"),
            Cmd::Put => f.write_str(".")?,
        }
        Ok(())
    }
}

fn closest_u8_change(now: u8, target: u8) -> Cmd {
    let a = if now <= target {
        Cmd::Add(target - now)
    } else {
        Cmd::Sub(now - target)
    };
    let b = if now <= target {
        Cmd::Sub(now + CELL_SIZE - target)
    } else {
        Cmd::Add(CELL_SIZE - now + target)
    };
    if a.len() <= b.len() {
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

fn in_one_u8(target: &[u8]) -> (usize, Vec<Cmd>) {
    let mut len = 0;
    let mut cmds = Vec::new();
    let mut now = 0;
    for &c in target.iter() {
        let cmd = closest_u8_change(now, c);
        len += cmd.len() + 1;
        cmds.push(cmd);
        cmds.push(Cmd::Put);
        now = c;
    }
    (len, cmds)
}

fn main() {
    let target = get_target();
    for cmd in in_one_u8(&target).1 {
        print!("{}", cmd);
    }
    println!()
}
