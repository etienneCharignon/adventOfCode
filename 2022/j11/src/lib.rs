mod inputs;

#[allow(dead_code)]
fn count_inspect<F>(monkeys: &mut Vec<&mut Vec<u64>>, operate: F, data: Vec<inputs::Monkey>, round: u64) -> Vec<u64>
where
    F: Fn(u64, u64) -> u64
{
    let mut visited: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let lowerer: u64 = data.iter().map(|monkey| monkey.test).product();
    for _round in 0..round {
        for i in 0..monkeys.len() {
            for w in 0..monkeys[i].len() {
                visited[i] = visited[i] + 1;
                let new_item = operate(i as u64, monkeys[i][w]) % lowerer;
                if new_item % data[i].test == 0 {
                    monkeys[data[i].dest.0].push(new_item);
                }
                else {
                    monkeys[data[i].dest.1].push(new_item);
                }
            }
            monkeys[i].clear();
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_count_inspect() {
        let mut monkey1 = vec![79, 98];
        let mut monkey2 = vec![54, 65, 75, 74];
        let mut monkey3 = vec![79, 60, 97];
        let mut monkey4 = vec![74];
        let mut monkeys: Vec<&mut Vec<u64>> = vec![
            &mut monkey1,
            &mut monkey2,
            &mut monkey3,
            &mut monkey4,
        ];
        let data = inputs::example();
        let inspected = count_inspect(&mut monkeys, inputs::operate_example, data, 10000);
        assert_eq!(inspected, vec![52166, 47830, 1938, 52013, 0, 0, 0, 0]);
    }

    #[test]
    fn it_solve() {
        let mut monkey0 = vec![91, 54, 70, 61, 64, 64, 60, 85];
        let mut monkey1 = vec![82];
        let mut monkey2 = vec![84, 93, 70];
        let mut monkey3 = vec![78, 56, 85, 93];
        let mut monkey4 = vec![64, 57, 81, 95, 52, 71, 58];
        let mut monkey5 = vec![58, 71, 96, 58, 68, 90];
        let mut monkey6 = vec![56, 99, 89, 97, 81];
        let mut monkey7 = vec![68, 72];
        let mut monkeys: Vec<&mut Vec<u64>> = vec![
            &mut monkey0,
            &mut monkey1,
            &mut monkey2,
            &mut monkey3,
            &mut monkey4,
            &mut monkey5,
            &mut monkey6,
            &mut monkey7,
        ];
        let data = inputs::input();
        let mut inspected = count_inspect(&mut monkeys, inputs::operate_input, data, 10000);
        inspected.sort();
        assert_eq!(inspected, vec![20018, 40008, 99986, 119967, 119979, 119992, 119995, 139947]);
        assert_eq!(inspected[6] * inspected[7], 16792940265);
    }

    #[test]
    fn it_do_a_round() {
        let mut monkey1 = vec![79, 98];
        let mut monkey2 = vec![54, 65, 75, 74];
        let mut monkey3 = vec![79, 60, 97];
        let mut monkey4 = vec![74];
        let mut monkeys: Vec<&mut Vec<u64>> = vec![
            &mut monkey1,
            &mut monkey2,
            &mut monkey3,
            &mut monkey4,
        ];
        let data = inputs::example();
        let inspected = count_inspect(&mut monkeys, inputs::operate_example, data, 1);
        assert_eq!(inspected, vec![2, 4, 3, 6, 0, 0, 0, 0]);
        assert_eq!(monkeys, vec![
            &vec![60, 71, 81, 80],
            &vec![77, 1504, 1865, 6244, 3603, 9412],
            &vec![],
            &vec![],
        ]);
    }

    #[test]
    fn it_do_twenty_round() {
        let mut monkey1 = vec![79, 98];
        let mut monkey2 = vec![54, 65, 75, 74];
        let mut monkey3 = vec![79, 60, 97];
        let mut monkey4 = vec![74];
        let mut monkeys: Vec<&mut Vec<u64>> = vec![
            &mut monkey1,
            &mut monkey2,
            &mut monkey3,
            &mut monkey4,
        ];
        let data = inputs::example();
        let inspected = count_inspect(&mut monkeys, inputs::operate_example, data, 20);
        assert_eq!(inspected, vec![99, 97, 8, 103, 0, 0, 0, 0]);
    }
}
