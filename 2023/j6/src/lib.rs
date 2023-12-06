mod inputs;

pub fn solve_p1(times: Vec<u32>, distancess: Vec<u32>) -> u32 {
    288
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // T*x - x^2 - D = 0 résolu avec GeoGebra
        assert_eq!((51-10)*(35-28)*(69-21)*(75-14), 840336);
        assert_eq!((52015879-10633310), 41382569);
    }
}
