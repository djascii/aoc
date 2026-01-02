use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let ranges = _input
        .split(',')
        .map(|range| {
            let range_tuple = range.split('-').collect_tuple().unwrap_or(("0", "0"));
            (
                range_tuple.0.parse::<u64>().unwrap_or(0),
                range_tuple.1.parse::<u64>().unwrap_or(0),
            )
        })
        .collect_vec();

    let res = ranges
        .iter()
        .flat_map(|(start, end)| {
            (*start..=*end)
                .map(|x| x.to_string())
                .filter(|x| {
                    x.len() % 2 == 0
                        && x.chars().take(x.len() / 2).eq(x
                            .chars()
                            .rev()
                            .take(x.len() / 2)
                            .collect::<String>()
                            .chars()
                            .rev())
                })
                .map(|x| x.parse::<u64>().unwrap_or(0))
                .collect::<Vec<u64>>()
        })
        .sum::<u64>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
