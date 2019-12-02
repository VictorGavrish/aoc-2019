static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let mut input: Box<[usize]> = INPUT
        .split(',')
        .map(|l| l.trim().parse().expect("unable to parse integer of input"))
        .collect();
    input[1] = 12;
    input[2] = 2;
    process(&mut input);
    println!("{}", input[0]);
}

fn process(input: &mut [usize]) {
    let mut position = 0;
    loop {
        let op = input[position..]
            .chunks(4)
            .next()
            .expect("unexpected end of file");
        dbg!(op);
        match op[0] {
            opcode::ADD => {
                let from1 = op[1];
                let from2 = op[2];
                let to = op[3];
                input[to] = input[from1] + input[from2];
                position += 4;
            }
            opcode::MUL => {
                let from1 = op[1];
                let from2 = op[2];
                let to = op[3];
                input[to] = input[from1] * input[from2];
                position += 4;
            }
            opcode::END => return,
            _ => panic!("unknown operation {}", op[0]),
        }
    }
}

mod opcode {
    pub const ADD: usize = 1;
    pub const MUL: usize = 2;
    pub const END: usize = 99;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program(program: &[usize], expected: &[usize]) {
        let mut output = program.to_vec();
        process(&mut output);
        assert_eq!(output, expected);
    }

    #[test]
    fn tests() {
        test_program(
            &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        test_program(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        test_program(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        test_program(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        test_program(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
