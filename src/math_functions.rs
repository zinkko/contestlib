use std::cmp::min;
use std::cmp::max;

pub fn gcd_recursive(x: i64, y: i64) -> i64 {
    let (a, b) = (min(x,y), max(x,y));
    if a == 0 {
        return 0;
    }
    match b % a {
        0 => a,
        _ => gcd_recursive(b%a, a),
    }
}

pub fn gcd(x: i64, y: i64) -> i64 {
    let (mut a, mut b) = (min(x,y), max(x,y));
    if a == 0 {
        return 0;
    }
    while b % a != 0 {
        let temp = b % a;
        b = a;
        a = temp;
    }
    a
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn gcd_small_coprime() {
        assert_eq!(gcd(54,35), 1);
    }

    #[test]
    fn gcd_small_not_coprime() {
        assert_eq!(gcd(105, 30), 15);
    }

    #[test]
    fn gcd_large_coprime() {
        let a = 862192863;
        let b = 3251705600;
        assert_eq!(gcd(a,b), 1)
    }

    #[test]
    fn gcd_large_not_coprime() {
        let a = 204857452800;
        let b = 8045539896240;
        let g = 942480;
        assert_eq!(gcd(a,b), g);
    }
}
