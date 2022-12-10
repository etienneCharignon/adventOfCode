mod inputs;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SignalStength(i32, i32);

fn print(tick: i32, x: i32) -> String {
    let mut pixel = String::new();
    if tick % 40 == 0 { pixel.push_str("\n"); }
    pixel.push_str(if (x-1..=x+1).contains(&(tick % 40)) { "#" } else { " " });
    pixel
}

#[allow(dead_code)]
pub fn compute_signal(program: &str, screen: &mut String) -> Vec<SignalStength> {
    let mut signal_strengths: Vec<SignalStength> = vec![];
    let mut tick: i32 = 0;
    let mut x: i32 = 1;
    let tick_mesures = vec![20, 60, 100, 140, 180, 220];
    let mut i = 0;
    for line in program.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();
        match instruction[0] {
            "noop" => {
                screen.push_str(&print(tick, x));
                tick += 1;
                if tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    if i < tick_mesures.len() - 1 { i += 1; }
                }
            }
            "addx" => {
                screen.push_str(&print(tick, x));
                tick += 1;
                if tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    if i < tick_mesures.len() - 1 { i += 1; }
                }
                screen.push_str(&print(tick, x));
                tick += 1;
                if tick == tick_mesures[i] {
                    signal_strengths.push(SignalStength(tick, x * tick));
                    if i < tick_mesures.len() - 1 { i += 1; }
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
        assert_eq!(compute_signal(inputs::EXAMPLE, &mut String::new()), vec![
            SignalStength(20, 420), 
            SignalStength(60, 1140),
            SignalStength(100, 1800),
            SignalStength(140, 2940), 
            SignalStength(180, 2880),
            SignalStength(220, 3960)
            ]);
    }

    #[test]
    fn it_solve_part1() {
        assert_eq!(compute_signal(inputs::EXAMPLE, &mut String::new()).iter().map(|signal_strengths| signal_strengths.1).sum::<i32>(), 13140);
        //assert_eq!(compute_signal(inputs::INPUT).iter().map(|signal_strengths| signal_strengths.1).sum::<i32>(), 14860);
    }

    #[test]
    fn it_print() {
        assert_eq!(41/40, 1);
        assert_eq!(print(1, 1), "#");
        assert_eq!(print(2, 1), "#");
        assert_eq!(print(3, 1), " ");
        assert_eq!(print(40, 1), "\n#");
        assert_eq!(print(41, 1), "#");
    }

    #[test]
    fn it_draw() {
        let mut screen = String::new();
        compute_signal(inputs::EXAMPLE, &mut screen);
        assert_eq!(screen, "
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ");
        println!("{}", screen);
    }
}
