static ROMAN: [(&'static str, u32); 13] = [
    ("M",  1000),
    ("CM", 900),
    ("D",  500),
    ("CD", 400),
    ("C",  100),
    ("XC", 90),
    ("L",  50),
    ("XL", 40),
    ("X",  10),
    ("IX", 9),
    ("V",  5),
    ("IV", 4),
    ("I",  1) ];

pub fn to_roman(n: u32) -> Option<String> {
    if n == 0 || n > 10000 { return None }
    let mut out = String::new();
    let mut n = n;
    for i in ROMAN.iter() {
        let (name, value) = *i;
        while n >= value {
            n -= value;
            out.push_str(name);
        }
    }
    assert!(n == 0);
    Some(out)
}

#[test]
fn test_to_roman() {
    assert_eq!(to_roman(1984).unwrap(), "MCMLXXXIV")
}
