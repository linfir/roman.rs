static ROMAN: [(&'static str, i32); 13] = [
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

/// The largest number representable as a roman numeral.
pub static MAX: i32 = 4999;

/// Converts an integer into a roman numeral.
///
/// Works for integer between 1 and 4999 inclusive, returns None otherwise.
///
/// # Example
///
/// ```
/// use roman;
///
/// let x = roman::to(14);
/// assert_eq!(x.unwrap(), "XIV");
/// ```
///
pub fn to(n: i32) -> Option<String> {
    if n <= 0 || n >= 5000 { return None }
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
    assert_eq!(to(1984).unwrap(), "MCMLXXXIV");
}
