mod inputs;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Triplet(pub i64, pub i64, pub i64);

impl Triplet {
    fn new(input: &str) -> Triplet {
        let numbers: Vec<i64> = input.split(", ").map(|d| d.trim().parse::<i64>().unwrap()).collect();
        Triplet(numbers[0], numbers[1], numbers[2])
    }
}

pub fn read(input: &str) -> Vec<Vec<Triplet>> {
    input.lines().map(|l| l.split(" @ ").map(|t| Triplet::new(t)).collect()).collect()
}

pub fn intersection(d1: &Vec<Triplet>, d2: &Vec<Triplet>) -> Option<(f64, f64)> {
    let p1 = (d1[0].0 as f64, d1[0].1 as f64);
    let v1 = (d1[1].0 as f64, d1[1].1 as f64);

    let p2 = (d2[0].0 as f64, d2[0].1 as f64);
    let v2 = (d2[1].0 as f64, d2[1].1 as f64);

    let c1 = v1.1 * p1.0 - v1.0 * p1.1;
    let c2 = v2.1 * p2.0 - v2.0 * p2.1;

    let y = (c1 / v1.1 - c2 / v2.1) / (-v1.0 / v1.1 + v2.0 / v2.1);
    let x = (c1 + v1.0 * y) / v1.1;
    if x == std::f64::INFINITY || y == std::f64::INFINITY  {
        None
    }
    else if (v1.0 < 0.0) != (x < p1.0) || (v1.1 < 0.0) != (y < p1.1) ||
       (v2.0 < 0.0) != (x < p2.0) || (v2.1 < 0.0) != (y < p2.1) {
        None
    }
    else {
        Some((x, y))
    }
}

pub fn count_intersections(hails: &Vec<Vec<Triplet>>, min_test_area: f64, max_test_area: f64) -> usize {
    let mut count = 0;
    for h1 in 0..hails.len() {
        for h2 in (h1+1)..hails.len() {
            match intersection(&hails[h1], &hails[h2]) {
                None => {},
                Some(i) => {
                    if i.0 >= min_test_area && i.0 <= max_test_area && i.1 >= min_test_area && i.1 <= max_test_area {
                        println!("{:?}, {:?}", hails[h1], hails[h2]);
                        count += 1;
                    }
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
    fn it_find_intersection() {
        assert_eq!(format!("{:?}", intersection(&vec![Triplet::new("18, 19, 22"), Triplet::new("-1, -1, -2")],
                                &vec![Triplet::new("20, 25, 34"), Triplet::new("-2, -2, -4")])),
                   "None");
        assert_eq!(format!("{:?}", intersection(&vec![Triplet::new("19, 13, 30"), Triplet::new("-2, 1, -2")],
                                &vec![Triplet::new("20, 19, 15"), Triplet::new("1, -5, -3")])),
                   "None");
        assert_eq!(format!("{:?}", intersection(&vec![Triplet::new("19, 13, 30"), Triplet::new("-2, 1, -2")],
                                &vec![Triplet::new("12, 31, 28"), Triplet::new("-1, -2, -1")])),
                   "Some((6.200000000000003, 19.4))");
        assert_eq!(format!("{:?}", intersection(&vec![Triplet::new("19, 13, 30"), Triplet::new("-2, 1, -2")],
                                &vec![Triplet::new("20, 25, 34"), Triplet::new("-2, -2, -4")])),
                   "Some((11.666666666666664, 16.666666666666668))");
        assert_eq!(format!("{:?}", intersection(&vec![Triplet::new("19, 13, 30"), Triplet::new("-2, 1, -2")],
                                &vec![Triplet::new("18, 19, 22"), Triplet::new("-1, -1, -2")])),
                   "Some((14.333333333333332, 15.333333333333334))");
    }
    #[test]
    fn it_solve() {
        assert_eq!(count_intersections(&read(inputs::EXAMPLE), 7.0, 27.0), 2);
        assert_eq!(count_intersections(&read(inputs::INPUT), 200000000000000.0, 400000000000000.0), 14672);
    }
}
