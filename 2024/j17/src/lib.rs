mod inputs;

pub fn combo(operand: u64, registers: &Vec<u64>) -> u64 {
    match operand {
        0|1|2|3 => operand,
        4|5|6 => registers[operand as usize - 4],
        _ => panic!("invalid operand {operand}")
    }
}

pub fn run(initial_registers: &Vec<u64>, program: &Vec<u64>) -> Vec<u64> {
    let mut registers = initial_registers.clone();
    let mut output: Vec<u64> = vec![];
    let mut ip: usize = 0;
    loop {
        if ip >= program.len() {
            break;
        }
        let opcode = program[ip];
        let operand = program[ip + 1];
        ip += 2;
        match opcode {
            0 => { // adv
                registers[0] = registers[0] / 2_u64.pow(combo(operand, &registers) as u32);
            },
            1 => { // bxl
                registers[1] ^= operand;
            },
            2 => { // bst
                registers[1] = combo(operand, &registers) % 8;
            },
            3 => { // jnz
                if registers[0] != 0 {
                    ip = operand as usize;
                }
            },
            4 => { // bxc
                registers[1] ^= registers[2];
            },
            5 => { // out
                output.push(combo(operand, &registers) % 8);
            },
            6 => { // bdv
                registers[1] = registers[0] / 2_u64.pow(combo(operand, &registers) as u32);
            },
            7 => { // cdv
                registers[2] = registers[0] / 2_u64.pow(combo(operand, &registers) as u32);
            },
            _ => { panic!("unknow opcode {}", opcode)}
        }
    }
    output
}

pub fn rprogram(a_initial: u64) -> Vec<u64> {
    let mut a: u64 = a_initial;
    let mut output: Vec<u64> = vec![];
    while a > 0 {
        let mut b = (a % 8) ^ 1;
        let c = (a >> b) % 8;
        b = b ^ 4 ^ c;
        output.push(b);

        a = a >> 3;
    }
    output
}

pub fn solve(program: &Vec<u64>, solution: u64, i: usize) -> Option<u64> {
    if i >= program.len() {
        return Some(solution);
    }
    if i == 0 {
        for a in 1..=1023 {
            let output = rprogram(a);
            if output.len() > 0 && output[0] == program[i] {
                println!("0 {a:o} : {:?}", output);
                match solve(program, a>>3, i + 1) {
                    Some(res) => { return Some((res << 3) + (a % 8)); }
                    None => {}
                }
            }
        }
    } else {
        for highbits in 0..=7 {
            let a = solution | (highbits << 7);
            let output = rprogram(a);
            if output.len() > 0 && output[0] == program[i] && (output.len() + i <= program.len()) {
                println!("{i} {a:o} : {:?} ({:?})", output, output.len() > 0 && output[0] == program[i] );
                match solve(program, a>>3, i + 1) {
                    Some(res) => {
                        let found = (res << 3) + (a % 8);
                        println!("found {found:o}");
                        return Some(found);
                    }
                    None => {}
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_out() {
        assert_eq!(run(&vec![1,2,4], &vec![5,0]), vec![0]);
        assert_eq!(run(&vec![1,2,4], &vec![5,1]), vec![1]);
        assert_eq!(run(&vec![1,2,4], &vec![5,4]), vec![1]);
        assert_eq!(run(&vec![1,2,4], &vec![5,5]), vec![2]);
        assert_eq!(run(&vec![8,2,4], &vec![5,4]), vec![0]);
    }

    #[test]
    fn it_xdv() {
        assert_eq!(run(&vec![6,0,0], &vec![0,1,5,4]), vec![3]);
        assert_eq!(run(&vec![6,0,0], &vec![6,1,5,5]), vec![3]);
        assert_eq!(run(&vec![6,0,0], &vec![7,1,5,6]), vec![3]);
    }

    #[test]
    fn it_bxl() {
        assert_eq!(run(&vec![0,2,0], &vec![1,7,5,5]), vec![5]);
    }

    #[test]
    fn it_bxc() {
        assert_eq!(run(&vec![0,2,7], &vec![4,0,5,5]), vec![5]);
    }

    #[test]
    fn it_jnz() {
        assert_eq!(run(&vec![1,0,0], &vec![3,4,5,0,5,1]), vec![1]);
        assert_eq!(run(&vec![1,0,0], &vec![3,4,5,0,5,1]), vec![1]);
    }

    #[test]
    fn it_bst() {
        assert_eq!(run(&vec![9,0,0], &vec![2,4,5,5]), vec![1]);
    }

    #[test]
    fn it_works_part1() {
        assert_eq!(run(&inputs::EXAMPLE_REGISTERS, &inputs::EXAMPLE_PROGRAM), vec![4,6,3,5,6,3,5,2,1,0]);
        assert_eq!(run(&inputs::INPUT_REGISTERS,&inputs::INPUT_PROGRAM),vec![7,1,2,3,2,6,7,2,5]);
    }

    #[test]
    fn it_works_part2() {
        assert_eq!(solve(&inputs::INPUT_PROGRAM, 0, 0), Some(202356708354602));
    }

    #[test]
    fn it_simulates() {
        for a in 1..2000 {
            let output = run(&vec![a,0,0], &inputs::INPUT_PROGRAM);
            let output2 = rprogram(a);
            assert_eq!(output2, output, "{a}");
        }
    }
}
