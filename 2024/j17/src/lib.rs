mod inputs;

pub fn combo(operand: u32, registers: &Vec<u32>) -> u32 {
    match operand {
        0|1|2|3 => operand,
        4|5|6 => registers[operand as usize - 4],
        _ => panic!("invalid operand {operand}")
    }
}

pub fn run(initial_registers: &Vec<u32>, program: &Vec<u32>) -> Vec<u32> {
    let mut registers = initial_registers.clone();
    let mut output: Vec<u32> = vec![];
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
                registers[0] = registers[0] / 2_u32.pow(combo(operand, &registers));
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
                registers[1] = registers[0] / 2_u32.pow(combo(operand, &registers));
            },
            7 => { // cdv
                registers[2] = registers[0] / 2_u32.pow(combo(operand, &registers));
            },
            _ => { panic!("unknow opcode {}", opcode)}
        }
    }
    output
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
    fn it_works() {
        assert_eq!(run(&inputs::EXAMPLE_REGISTERS, &inputs::EXAMPLE_PROGRAM), vec![4,6,3,5,6,3,5,2,1,0]);
        assert_eq!(run(&inputs::INPUT_REGISTERS,&inputs::INPUT_PROGRAM),vec![7,1,2,3,2,6,7,2,5]);
    }
}
