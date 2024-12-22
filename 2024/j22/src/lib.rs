mod inputs;

pub fn mix_prune(secret: u64, number: u64) -> u64 {
    let mix = secret ^ number;
    mix % 16777216
}

pub fn evolve_secret(secret: u64) -> u64 {
    let mut new_secret = mix_prune(secret, secret * 64);
    new_secret = mix_prune(new_secret, new_secret / 32);
    mix_prune(new_secret, new_secret * 2048)
}

pub fn generate_secret(n: u64, initial_number: u64) -> u64 {
    let mut secret = initial_number;
    for _ in 0..n {
        secret = evolve_secret(secret);
    }
    secret
}

pub fn sum_secrets(input: &str) -> u64 {
    input.split("\n").map(|s| s.parse().expect("Failed to parse number"))
        .map(|initial_number| generate_secret(2000, initial_number))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evolve_a_secret() {
        assert_eq!(evolve_secret(123), 15887950);
    }

    #[test]
    fn it_works() {
        assert_eq!(sum_secrets(inputs::EXAMPLE), 37327623);
        assert_eq!(sum_secrets(inputs::INPUT), 17577894908);
    }
}
