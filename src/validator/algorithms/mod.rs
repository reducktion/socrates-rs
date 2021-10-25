use std::borrow::Borrow;

pub fn compute_luhn_10_check_digit(code: &str) -> u32 {
    let mut code_with_check_digit = code.to_owned();
    code_with_check_digit.push_str("0");
    let sum: u32 = luhn_10_sum(code_with_check_digit.borrow());
    return (sum * 9) % 10;
}

pub fn validate_luhn_10(code: &str) -> bool {
    let sum: u32 = luhn_10_sum(code);
    return (sum % 10) == 0;
}

// https://en.wikipedia.org/wiki/Luhn_algorithm
fn luhn_10_sum(code: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut is_odd = (code.chars().count() % 2) != 0;
    for char in code.chars() {
        if is_odd {
            sum += char.to_digit(36).unwrap()
        } else {
            let digit = char.to_digit(36).unwrap() * 2;
            sum += if digit > 9 { digit - 9 } else { digit }
        }
        is_odd = !is_odd;
    }
    return sum;
}

const VERHOEFF_DIHEDRAL: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];

const VERHOEFF_PERMUTATION: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
    [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
    [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
    [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];

const VERHOEFF_INV: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

pub fn validate_verhoeff(code: &str) -> bool {
    let c = verhoeff_check_digit(code);
    return c == 0;
}

pub fn compute_verhoeff_check_digit(code: &str) -> u32 {
    let mut owned_code = String::from(code);
    owned_code.push_str("0");
    let c: usize = verhoeff_check_digit(owned_code.borrow());
    return VERHOEFF_INV[c] as u32;
}

// https://en.wikipedia.org/wiki/Verhoeff_algorithm
fn verhoeff_check_digit(code: &str) -> usize {
    let mut c: usize = 0;
    for (idx, char) in code.chars().rev().into_iter().enumerate() {
        let digit: usize = char.to_digit(36).unwrap() as usize;
        let p = VERHOEFF_PERMUTATION[idx % 8][digit] as usize;
        c = VERHOEFF_DIHEDRAL[c][p] as usize;
    }
    return c;
}

#[cfg(test)]
mod tests {
    use crate::validator::algorithms::{
        compute_luhn_10_check_digit, compute_verhoeff_check_digit, validate_luhn_10,
        validate_verhoeff,
    };

    #[test]
    fn luhn_validation_algorithm() {
        assert_eq!(3, compute_luhn_10_check_digit("7992739871"));
        assert!(validate_luhn_10("79927398713"));
        assert!(validate_luhn_10("1983081246783"));
    }

    #[test]
    fn verhoeff_validation_algorithm() {
        assert_eq!(3, compute_verhoeff_check_digit("236"));
        assert!(validate_verhoeff("2363"));
        assert_eq!(validate_verhoeff("5971654782313"), false);
    }
}
