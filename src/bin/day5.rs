use std::{iter::from_fn, ops::Deref, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Converter {
    //{{{
    source_start: usize,
    dest_start: usize,
    len: usize,
}

impl Converter {
    fn from_str_opt(s: &str) -> Option<Self> {
        let mut iter = s.split_whitespace();

        let a: usize = iter.next()?.parse().ok()?;
        let b: usize = iter.next()?.parse().ok()?;
        let c: usize = iter.next()?.parse().ok()?;

        if iter.next().is_some() {
            return None;
        }

        Some(Converter {
            dest_start: a,
            source_start: b,
            len: c,
        })
    }

    fn convert(&self, value: usize) -> Option<usize> {
        if value < self.source_start {
            None
        } else {
            let offset = value - self.source_start;
            (offset < self.len).then_some(self.dest_start + offset)
        }
    }
}

impl FromStr for Converter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_opt(s).ok_or_else(|| format!("Failed to parse converter: {s}"))
    }
    //}}}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mapper(Vec<Converter>); //{{{

impl Mapper {
    fn from_str_iter<S: Deref<Target = str>, T: IntoIterator<Item = S>>(iter: T) -> Option<Self> {
        let mut iter = iter.into_iter();
        let mut started = false;

        // let first_line = iter.next().unwrap();
        // let title = first_line.trim().strip_suffix(" map:");
        // let _ = iter.next()?; // Skip first line

        // Use a closure that captures the input `iter` to create a custom iterator which
        // skips any number of lines before thte first converter is successfully parsed, then
        // terminates either at the end of the input or after the first str that fails to parse
        // into a `Converter`.
        let converters_iter = from_fn(|| {
            if started {
                Converter::from_str_opt(&iter.next()?)
            } else {
                while let Some(item) = &iter.next() {
                    let converter_opt = Converter::from_str_opt(item);
                    if converter_opt.is_some() {
                        started = true;
                        return converter_opt;
                    }
                }

                None
            }
        });

        let v: Vec<Converter> = converters_iter.collect();
        (!v.is_empty()).then_some(Self(v))
    }

    fn map(&self, value: usize) -> usize {
        self.0
            .iter()
            .find_map(|converter| converter.convert(value))
            .unwrap_or(value)
    }
} //}}}

fn solutionate<S: Deref<Target = str>, I: IntoIterator<Item = S>>(
    input: I,
) -> Result<usize, String> {
    let mut iter = input.into_iter();

    let first_line = &*iter.next().ok_or("Empty input".to_string())?;
    let seeds = first_line
        .strip_prefix("seeds: ")
        .ok_or_else(|| format!("Failed to parse seeds on the first line: {first_line}."))?
        .split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|_| format!("Failed to parse seeds on the first line: {first_line}."))?;

    if seeds.is_empty() {
        return Err(format!("No seeds found in: {first_line}"));
    }

    let mappers: Vec<Mapper> = from_fn(|| Mapper::from_str_iter(&mut iter)).collect();

    Ok(seeds
        .into_iter()
        .map(|seed| mappers.iter().fold(seed, |acc, mapper| mapper.map(acc)))
        .min()
        .expect("`seeds` is not empty."))
}

fn main() -> Result<(), String> {
    let mut input_iter = std::io::stdin().lines();

    // Create a wrapper iterator around the input iterator to handle IO errors.
    // If no IO errors are encountered, then this will just unwrap each element and return the
    // string within. If an error is encountered, then it will save the error in the `error`
    // reference, then terminate by returning a `None`.
    let mut error = None;
    let iter = {
        let error = &mut error;
        from_fn(|| match input_iter.next() {
            None => None,
            Some(Ok(s)) => Some(s),
            Some(Err(e)) => {
                *error = Some(e);
                None
            }
        })
    };

    let answer = solutionate(iter)?;

    if let Some(e) = error {
        return Err(e.to_string());
    }

    println!("{}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_convert_0() {
        let c = Converter {
            source_start: 52,
            dest_start: 50,
            len: 48,
        };
        assert_eq!(Some(81), c.convert(79));
    }

    #[test]
    fn test_map_from_str_iter_0() {
        let s = "";
        assert_eq!(None, Mapper::from_str_iter(s.lines()));
    }

    #[test]
    fn test_map_from_str_iter_1() {
        let s = "seed-to-soil map:";
        assert_eq!(None, Mapper::from_str_iter(s.lines()));
    }

    #[test]
    fn test_map_from_str_iter_2() {
        let s = "
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:";
        let mut iter = s.lines();
        let map = Mapper::from_str_iter(&mut iter).unwrap();
        assert_eq!(
            vec![
                Converter {
                    dest_start: 50,
                    source_start: 98,
                    len: 2
                },
                Converter {
                    dest_start: 52,
                    source_start: 50,
                    len: 48
                }
            ],
            map.0
        );
        assert_eq!(Some("soil-to-fertilizer map:"), iter.next());
    }

    #[test]
    fn test_map_map_0() {
        let map = Mapper(vec![
            Converter {
                dest_start: 50,
                source_start: 98,
                len: 2,
            },
            Converter {
                dest_start: 52,
                source_start: 50,
                len: 48,
            },
        ]);
        assert_eq!(99, map.map(51))
    }

    #[test]
    fn test_map_map_1() {
        let map = Mapper(vec![
            Converter {
                dest_start: 50,
                source_start: 98,
                len: 2,
            },
            Converter {
                dest_start: 52,
                source_start: 50,
                len: 48,
            },
        ]);
        assert_eq!(11, map.map(11))
    }

    #[test]
    fn test_solution_0() {
        let s = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(Ok(35), solutionate(s.lines()));
    }
}
