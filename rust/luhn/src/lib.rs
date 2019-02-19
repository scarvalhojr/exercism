/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match code
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| ch.to_digit(10))
        .collect::<Option<Vec<_>>>()
    {
        None => false,
        Some(digits) => {
            if digits.len() <= 1 {
                return false;
            }

            digits
                .iter()
                .rev()
                .enumerate()
                .map(|(pos, &dig)| if pos % 2 == 0 { dig } else { 2 * dig })
                .map(|value| if value > 9 { value - 9 } else { value })
                .sum::<u32>()
                % 10
                == 0
        }
    }
}

/// Alternative version using fold
pub fn is_valid_fold(code: &str) -> bool {
    code.chars()
        .filter(|ch| !ch.is_whitespace())
        .rev()
        .try_fold((false, 0_i32, 0_u32), |(dbl, count, acc), ch| {
            ch.to_digit(10).map(|dig| {
                (
                    !dbl,
                    count + 1,
                    acc + if dbl && dig >= 5 {
                        2 * dig - 9
                    } else if dbl {
                        2 * dig
                    } else {
                        dig
                    },
                )
            })
        })
        .map_or(false, |(_, count, acc)| count > 1 && acc % 10 == 0)
}
