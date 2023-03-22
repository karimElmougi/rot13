// 6.5 GiB/s
#[inline]
pub fn nop(c: char) -> char { c }

// 2.1 GiB/s
#[inline]
pub const fn naive(c: char) -> char {
    if c.is_ascii_alphabetic() {
        if c.is_ascii_lowercase() {
            compute(c as u8, b'a') as char
        } else {
            compute(c as u8, b'A') as char
        }
    } else {
        c
    }
}

// 3.1 GiB/s
#[inline]
pub fn naive_sub(c: char) -> char {
    match c {
        'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
        'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
        _ => c,
    }
}

// 4.9 GiB/s
#[inline]
pub fn naive_sub_byte(c: char) -> char {
    let c = c as u8;
    match c {
        b'A'..=b'M' | b'a'..=b'm' => (c + 13) as char,
        b'N'..=b'Z' | b'n'..=b'z' => (c - 13) as char,
        _ => c as char,
    }
}

#[inline]
const fn compute(c: u8, base: u8) -> u8 {
    base + wrap(c.wrapping_sub(base).wrapping_add(13))
}

#[inline]
const fn wrap(n: u8) -> u8 {
    n % 26
}

// 0.5 GiB/s
#[inline]
pub fn table(c: char) -> char {
    const TABLE: &[u8] =  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes();
    const MAPPED: &[u8] = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm".as_bytes();
    TABLE
        .iter()
        .position(|x| *x == c as u8)
        .map(|i| *unsafe { MAPPED.get_unchecked(i) } as char)
        .unwrap_or(c)
}

// 2 GiB/s
#[inline]
pub fn const_table(c: char) -> char {
    const TABLE: [u8; u8::MAX as usize] = init_table();
    TABLE[c as usize] as char
}

const fn init_table() -> [u8; u8::MAX as usize] {
    let mut table = [0u8; u8::MAX as usize];
    let mut i = 0;
    while i < u8::MAX {
        table[i as usize] = naive(i as char) as u8;
        i += 1;
    }
    table
}

// 2.5 GiB/s
#[inline]
pub fn branchless(c: char) -> char {
    let c = c as u8;
    let is_lower = c >= b'a' && c <= b'z';
    let is_upper = c >= b'A' && c <= b'Z';
    let cl = is_lower as u8 * compute(c, b'a');
    let cu = is_upper as u8 * compute(c, b'A');
    let c_else = !(is_lower || is_upper) as u8 * c;
    (cl + cu + c_else) as char
}

// 4.5 GiB/s
#[inline]
pub fn branchless_sub(c: char) -> char {
    let c = c as u8;
    let is_first_half = matches!(c, b'a' ..= b'm' | b'A' ..= b'M') as u8;
    let is_last_half = matches!(c, b'n' ..= b'z' | b'N' ..= b'Z') as u8;
    let is_neither = (is_first_half | is_last_half) ^ 1;
    let a = [is_first_half, is_last_half, is_neither];
    let b = [c + 13, c - 13, c];
    dot_product(&a, &b) as char
}

// 4.3 GiB/s
#[inline]
pub fn branchless_sub_mul(c: char) -> char {
    let c = c as u8;
    let is_first_half = matches!(c, b'a' ..= b'm' | b'A' ..= b'M') as i16;
    let is_last_half = matches!(c, b'n' ..= b'z' | b'N' ..= b'Z') as i16;
    let sel = is_first_half - is_last_half; // 1 if is_first_half, -1 if is_last_half, 0 if neither
    (c as i16 + 13 * sel) as u8 as char
}

#[inline]
fn dot_product(a: &[u8], b: &[u8]) -> u8 {
    a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
}

// 0.6 GiB/s
#[inline]
pub fn jump_table(c: char) -> char {
    match c {
        'a' => 'n',
        'b' => 'o',
        'c' => 'p',
        'd' => 'q',
        'e' => 'r',
        'f' => 's',
        'g' => 't',
        'h' => 'u',
        'i' => 'v',
        'j' => 'w',
        'k' => 'x',
        'l' => 'y',
        'm' => 'z',
        'n' => 'a',
        'o' => 'b',
        'p' => 'c',
        'q' => 'd',
        'r' => 'e',
        's' => 'f',
        't' => 'g',
        'u' => 'h',
        'v' => 'i',
        'w' => 'j',
        'x' => 'k',
        'y' => 'l',
        'z' => 'm',
        'A' => 'N',
        'B' => 'O',
        'C' => 'P',
        'D' => 'Q',
        'E' => 'R',
        'F' => 'S',
        'G' => 'T',
        'H' => 'U',
        'I' => 'V',
        'J' => 'W',
        'K' => 'X',
        'L' => 'Y',
        'M' => 'Z',
        'N' => 'A',
        'O' => 'B',
        'P' => 'C',
        'Q' => 'D',
        'R' => 'E',
        'S' => 'F',
        'T' => 'G',
        'U' => 'H',
        'V' => 'I',
        'W' => 'J',
        'X' => 'K',
        'Y' => 'L',
        'Z' => 'M',

        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use crate as rot13;

    macro_rules! test_cases {
        ($m:ident::$f:ident) => {
            let s = "AbCd M wXyZ";
            assert_eq!(s.chars().map($m::$f).collect::<String>(), "NoPq Z jKlM");
            assert_eq!(
                s.chars().map($m::$f).map(rot13::naive).collect::<String>(),
                s
            );
        };
    }

    #[test]
    fn naive() {
        test_cases!(rot13::naive);
    }

    #[test]
    fn naive_sub() {
        test_cases!(rot13::naive_sub);
    }

    #[test]
    fn table() {
        test_cases!(rot13::table);
    }

    #[test]
    fn branchless() {
        test_cases!(rot13::branchless);
    }

    #[test]
    fn branchless_sub() {
        test_cases!(rot13::branchless_sub);
    }

    #[test]
    fn jump_table() {
        test_cases!(rot13::jump_table);
    }

    #[test]
    fn const_table() {
        test_cases!(rot13::const_table);
    }

    #[test]
    fn branchless_sub_mul() {
        test_cases!(rot13::branchless_sub_mul);
    }
}
