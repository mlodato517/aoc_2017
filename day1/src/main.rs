fn main() {
    let input = include_str!("../input");

    println!("Part 1 - {:?}", part1(input));
}

/// Cheap way to convert a byte that represents an ASCII digit to its digit value.
/// Preferred over `char.to_digit` just to save on the `unwrap`s.
fn to_digit(byte: u8) -> usize {
    (byte - b'0') as usize
}

/// Sum the digits in the input string that equal the next digit. The list is circular so the last
/// digit must be checked against the first.
fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    let mut sum = 0;

    for window in input.windows(2) {
        if window[0] == window[1] {
            sum += to_digit(window[0]);
        }
    }
    if input[0] == input[input.len() - 1] {
        sum += to_digit(input[0]);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn example1() {
            assert_eq!(part1("1122"), 3);
        }

        #[test]
        fn example2() {
            assert_eq!(part1("1111"), 4);
        }

        #[test]
        fn example3() {
            assert_eq!(part1("1234"), 0);
        }

        #[test]
        fn example4() {
            assert_eq!(part1("91212129"), 9);
        }
    }
}
