static INPUT: &str = include_str!("../resources/input.txt");

fn main() {
    let memory: Box<[usize]> = INPUT
        .split(',')
        .map(|l| l.trim().parse().expect("unable to parse integer of input"))
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory_copy = memory.clone();
            memory_copy[1] = noun;
            memory_copy[2] = verb;
            process(&mut memory_copy);
            if memory_copy[0] == 19690720 {
                println!("Found answer. Noun: {}, Verb: {}", noun, verb);
                println!("100 * noun + verb = {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn process(memory: &mut [usize]) {
    let mut instruction_pointer = 0;
    loop {
        let instruction = memory[instruction_pointer..]
            .chunks(4)
            .next()
            .expect("unexpected end of file");
        match instruction[0] {
            opcode::ADD => {
                let from1 = instruction[1];
                let from2 = instruction[2];
                let to = instruction[3];
                memory[to] = memory[from1] + memory[from2];
                instruction_pointer += 4;
            }
            opcode::MUL => {
                let from1 = instruction[1];
                let from2 = instruction[2];
                let to = instruction[3];
                memory[to] = memory[from1] * memory[from2];
                instruction_pointer += 4;
            }
            opcode::END => return,
            _ => panic!("unknown operation {}", instruction[0]),
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
