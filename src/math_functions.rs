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

pub fn gcd_imperative(x: i64, y: i64) -> i64 {
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

    #[test]
    fn it_works() {

    }
}
