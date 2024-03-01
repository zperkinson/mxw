use std::{ops::RangeInclusive, str::FromStr};

pub fn contains(range: &'static RangeInclusive<usize>) -> impl Fn(&str) -> Result<(), String> {
    |str: &str| {
        usize::from_str(str)
            .map(|value| range.contains(&value))
            .map_err(|error| error.to_string())
            .and_then(|result| {
                if result {
                    return Ok(());
                }

                Err(format!("not in range {}-{}", range.start(), range.end()))
            })
    }
}
