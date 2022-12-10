mod inputs;

#[derive(Debug, PartialEq, Copy, Clone)]
struct SignalStength(i32, i32);

#[allow(dead_code)]
fn compute_signal(program: &str) -> Vec<SignalStength> {
    let mut signal_strengths: Vec<SignalStength> = vec![];
    let mut tick: i32 = 0;
    let mut x: i32 = 1;
    let tick_mesures = vec![20, 60, 100, 140, 180, 220];
    let mut i = 0;
    for line in program.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();
        match instruction[0] {
            "noop" => {
                tick += 1;
                if i < tick_mesures.len() && tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    i += 1;
                }
            }
            "addx" => {
                tick += 1;
                if i < tick_mesures.len() && tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    i += 1;
                }
                tick += 1;
                if i < tick_mesures.len() && tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    i += 1;
                }
                x += instruction[1].parse::<i32>().unwrap();
            }
            _ => panic!("unkown instruction")
        }
    }
    signal_strengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_signal_strength() {
        assert_eq!(compute_signal(inputs::EXAMPLE), vec![
            SignalStength(20, 420), 
            SignalStength(60, 1140),
            SignalStength(100, 1800),
            SignalStength(140, 2940), 
            SignalStength(180, 2880),
            SignalStength(220, 3960)
            ]);
    }

    #[test]
    fn it_solve() {
        assert_eq!(compute_signal(inputs::EXAMPLE).iter().map(|signal_strengths| signal_strengths.1).sum::<i32>(), 13140);
        assert_eq!(compute_signal(inputs::INPUT).iter().map(|signal_strengths| signal_strengths.1).sum::<i32>(), 14860);
    }
}
