mod inputs;

pub fn calcul(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut count = 0;
    let mut curseur = 50;
    for line in lines {
        let nombre: i32 = line[1..].parse().unwrap();
        let direction = if line.starts_with('R') { 1 } else { -1 };
        curseur = (curseur + nombre * direction) % 100;
        if curseur == 0 {
            count += 1;
        }
    }
    count
}

pub fn compte(nombre: i32, direction: i32, curseur: i32) -> (i32, i32) {
    let mut nouveau_curseur = (curseur + nombre * direction) % 100;
    let mut count = 0;
    let target = nombre * direction;
    if direction >= 0 {
        for i in 1..=target {
            nouveau_curseur = (curseur + i) % 100;
            if nouveau_curseur == 0 {
                count += 1;
            }
        }
    } else {
        for i in (target..=-1).rev() {
            nouveau_curseur = (curseur + i) % 100;
            if nouveau_curseur == 0 {
                count += 1;
            }
        }
    }
    if nouveau_curseur < 0 {
        nouveau_curseur += 100;
    }
    (nouveau_curseur, count)
}

pub fn calcul2(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut count = 0;
    let mut curseur = 50;
    for line in lines {
        let nombre: i32 = line[1..].parse().unwrap();
        let direction = if line.starts_with('R') { 1 } else { -1 };
        let c: i32;
        (curseur, c) = compte(nombre, direction, curseur);
        count += c;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcul_secret_j1() {
        assert_eq!(calcul(inputs::EXEMPLE), 3);
        assert_eq!(calcul(inputs::INPUT), 1141);
    }

    #[test]
    fn compte_le() {
        assert_eq!(compte(49, -1, 50), (1, 0));
        assert_eq!(compte(50, -1, 50), (0, 1));
        assert_eq!(compte(100, 1, 50), (50, 1));
        assert_eq!(compte(150, 1, 50), (0, 2));
        assert_eq!(compte(100, -1, 50), (50, 1));
        assert_eq!(compte(51, -1, 50), (99, 1));
        assert_eq!(compte(2, 1, 1), (3, 0));
        assert_eq!(compte(1, -1, 0), (99, 0));
        assert_eq!(compte(2, 1, 99), (1, 1));
        assert_eq!(compte(102, 1, 99), (1, 2));
        assert_eq!(compte(151, -1, 50), (99, 2));
    }

    #[test]
    fn calcul_secret_j2() {
        assert_eq!(calcul2(inputs::EXEMPLE), 6);
        assert_eq!(calcul2(inputs::INPUT), 6634); // 7066 trop haut
    }
}
