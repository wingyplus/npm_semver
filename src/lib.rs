#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    IResult,
};
use std::cmp::{Ord, Ordering, PartialEq};
use std::str;

fn cmp_identifier(a: u32, b: u32) -> Ordering {
    if a == b {
        return Ordering::Equal;
    } else if a > b {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

/// NPM semantic version.
#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Ord for Version {
    fn cmp(&self, rhs: &Self) -> Ordering {
        cmp_identifier(self.major, rhs.major)
            .then(cmp_identifier(self.minor, rhs.minor))
            .then(cmp_identifier(self.patch, rhs.patch))
    }
}

#[derive(Debug)]
pub struct ParseError;

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

/// Parse NPM semantic version.
pub fn parse(input: &str) -> IResult<&str, Version> {
    let input = input.trim();
    let (input, _) = opt(tag("v"))(input)?;
    let (input, major) = take_while(is_digit)(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, minor) = take_while(is_digit)(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, patch) = take_while(is_digit)(input)?;

    Ok((
        input,
        Version {
            major: major.parse().unwrap(),
            minor: minor.parse().unwrap(),
            patch: patch.parse().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (_, version) = parse("1.2.3").unwrap();
        assert_eq!(
            version,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );

        let (_, version) = parse("v1.2.3").unwrap();
        assert_eq!(
            version,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[quickcheck]
    fn parse_normal(
        major: u32,
        minor: u32,
        patch: u32,
        padding: usize,
        trailing: usize,
        has_v: bool,
    ) {
        let limit = 100_000;
        // Do not make spaces consume to much memory.
        let padding_spaces = " ".repeat(padding % limit);
        let trailing_spaces = " ".repeat(trailing % limit);

        let version = format!("{}.{}.{}", major, minor, patch);
        let version = if has_v {
            format!("v{}", version)
        } else {
            version
        };
        let version = format!("{}{}{}", padding_spaces, version, trailing_spaces);

        let (_, version) = parse(&version).unwrap();
        assert_eq!(
            version,
            Version {
                major: major,
                minor: minor,
                patch: patch
            }
        )
    }

    #[test]
    fn test_compare() {
        let a = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert!(a == a);
        assert!(a <= a);

        let b = Version {
            major: 1,
            minor: 2,
            patch: 4,
        };
        assert!(a < b);
        assert!(a <= b);
        assert!(b >= a);
        assert!(b > a);

        let b = Version {
            major: 1,
            minor: 3,
            patch: 3,
        };
        assert!(a < b);
        assert!(a <= b);
        assert!(b >= a);
        assert!(b > a);

        let b = Version {
            major: 2,
            minor: 2,
            patch: 3,
        };
        assert!(a < b);
        assert!(a <= b);
        assert!(b >= a);
        assert!(b > a);
    }
}
