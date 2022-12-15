mod inputs;
use std::cmp;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(i32, i32);

#[allow(dead_code)]
fn distance(sensor: (i32, i32), beacon: (i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

#[allow(dead_code)]
fn seen(position: (i32, i32), sensor: (i32, i32), d: i32) -> bool {
    distance(position, sensor) <= d
}

#[allow(dead_code)]
fn count_seens(y: i32, input: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut width: [i32; 2] = [0, 0];
    for (s, b) in input {
        let d = distance(*s, *b);
        width[0] = cmp::min(width[0], s.0 - d);
        width[1] = cmp::max(width[1], s.0 + d);
    }
    println!("{}, {}", width[0], width[1]);
    let beacons: HashSet<(i32, i32)> = HashSet::from_iter(input.iter().map(|line| line.1));
    println!("{:?}", beacons);
    let mut count = 0;
    for x in width[0]..=width[1] {
        if ! beacons.contains(&(x, y)) {
            for (s, b) in input {
                if seen((x, y), *s, distance(*s, *b)) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_distance() {
        assert_eq!(distance((8, 7), (2, 10)), 9);
        assert_eq!(distance((8, 7), (8, 7)), 0);
        assert_eq!(distance((2, 10), (8, 7)), 9);
    }

    #[test]
    fn it_seen_by_sensor() {
        assert!(seen((-1, 7), (8, 7), 9));
        assert!(!seen((-2, 7), (8, 7), 9));
        assert!(seen((17, 7), (8, 7), 9));
        assert!(!seen((18, 7), (8, 7), 9));
        assert!(seen((8, -2), (8, 7), 9));
        assert!(!seen((8, -3), (8, 7), 9));
        assert!(!seen((9, -2), (8, 7), 9));
        assert!(seen((8, 16), (8, 7), 9));
        assert!(seen((7, -1), (8, 7), 9));
        assert!(seen((2, 10), (8, 7), 9));
        assert!(seen((8, 7), (8, 7), 9));
    }

    #[test]
    // #[ignore]
    fn it_count_seens() {
        let example: Vec<((i32, i32), (i32, i32))> = vec![
            ((2, 18), (-2, 15)),
            ((9, 16), (10, 16)),
            ((13,2 ), (15, 3)),
            ((12,14), (10, 16)),
            ((10,20), (10, 16)),
            ((14,17), (10, 16)),
            ((8, 7 ), (2,  10)),
            ((2, 0 ), (2,  10)),
            ((0, 11), (2,  10)),
            ((20,14), (25, 17)),
            ((17,20), (21, 22)),
            ((16,7 ), (15, 3)),
            ((14,3 ), (15, 3)),
            ((20,1 ), (15, 3)),
        ];
        assert_eq!(count_seens(10, &example), 26);

        let input: Vec<((i32, i32), (i32, i32))> = vec![
            ((2924811, 3544081), (3281893,  3687621)),
            ((2719183, 2520103), (2872326,  2415450)),
            ((3754787, 3322726), (3281893,  3687621)),
            ((1727202, 1485124), (1329230,  1133797)),
            ((2517008, 2991710), (2454257,  2594911)),
            ((1472321, 3123671), (2216279,  3414523)),
            ((3456453, 3959037), (3281893,  3687621)),
            ((3997648, 2624215), (4401794,  2000000)),
            ((463281,  967584 ), (1329230,  1133797)),
            ((2395529, 1897869), (2454257,  2594911)),
            ((3094466, 3888307), (3281893,  3687621)),
            ((2737812, 3928154), (2744537,  4159197)),
            ((813538,  3874308), (2216279,  3414523)),
            ((822358,  1997263), (1329230,  1133797)),
            ((3993754, 3951321), (3281893,  3687621)),
            ((2585409, 3541887), (2216279,  3414523)),
            ((3269796, 3730504), (3281893,  3687621)),
            ((3075750, 2873879), (2872326,  2415450)),
            ((1357,    2747918), (-1077481, 3057204)),
            ((2256257, 344800 ), (1854450,  -900998)),
            ((2779742, 2308087), (2872326,  2415450)),
            ((867692,  64146  ), (1329230,  1133797)),
            ((3454465, 966419 ), (4401794,  2000000)),
            ((1902550, 2398376), (2454257,  2594911)),
        ];
        assert_eq!(count_seens(2000000, &input), 5040643)
    }
}
