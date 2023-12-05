use std::{
    iter::from_fn,
    ops::{Add, Deref, Range},
    str::FromStr,
};

// Helper extension traits{{{
trait RangeUniformAdd<T> {
    fn uniform_add(self, v: T) -> Self;
}

impl<T: Add<Output = T> + Copy> RangeUniformAdd<T> for Range<T> {
    fn uniform_add(self, v: T) -> Self {
        (self.start + v)..(self.end + v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubtractionResult<T> {
    None,
    Full,
    Leftover0 {
        subtracted: T,
        leftover: T,
    },
    Leftover1 {
        subtracted: T,
        leftover0: T,
        leftover1: T,
    },
}

trait SubtractRange {
    fn subtract_range(&self, other: &Self) -> SubtractionResult<Self>
    where
        Self: Sized;
}

impl<T: Ord + Copy> SubtractRange for Range<T> {
    fn subtract_range(&self, other: &Self) -> SubtractionResult<Self> {
        if self.end <= other.start || other.end <= self.start {
            SubtractionResult::<Self>::None
        } else if other.start <= self.start {
            if self.end <= other.end {
                SubtractionResult::<Self>::Full
            } else {
                SubtractionResult::<Self>::Leftover0 {
                    subtracted: self.start..other.end,
                    leftover: other.end..self.end,
                }
            }
        } else if self.end <= other.end {
            SubtractionResult::<Self>::Leftover0 {
                subtracted: other.start..self.end,
                leftover: self.start..other.start,
            }
        } else {
            SubtractionResult::<Self>::Leftover1 {
                subtracted: other.start..other.end,
                leftover0: self.start..other.start,
                leftover1: other.end..self.end,
            }
        }
    }
}

trait DefragmentRanges {
    fn defragment_ranges(&mut self);
}

impl<T: Ord + Copy> DefragmentRanges for Vec<Range<T>> {
    fn defragment_ranges(&mut self) {
        self.sort_unstable_by(|a, b| b.start.cmp(&a.start));
        for i in (0..self.len() - 1).rev() {
            if self[i + 1].end >= self[i].start {
                self[i].start = self.swap_remove(i + 1).start;
            }
        }
    }
} //}}}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Converter {
    //{{{
    source_range: Range<isize>,
    offset: isize,
}

impl Converter {
    fn from_str_opt(s: &str) -> Option<Self> {
        let mut iter = s.split_whitespace();

        let dest_start: isize = iter.next()?.parse().ok()?;
        let source_start: isize = iter.next()?.parse().ok()?;
        let len: isize = iter.next()?.parse().ok()?;

        if iter.next().is_some() {
            return None;
        }

        Some(Converter {
            offset: dest_start - source_start,
            source_range: source_start..source_start + len,
        })
    }

    fn process_ranges(&self, ranges: &mut Vec<Range<isize>>, out: &mut Vec<Range<isize>>) {
        for i in (0..ranges.len()).rev() {
            match ranges[i].subtract_range(&self.source_range) {
                SubtractionResult::None => (),
                SubtractionResult::Full => out.push(ranges.swap_remove(i).uniform_add(self.offset)),
                SubtractionResult::Leftover0 {
                    subtracted,
                    leftover,
                } => {
                    ranges[i] = leftover;
                    out.push(subtracted.uniform_add(self.offset));
                }
                SubtractionResult::Leftover1 {
                    subtracted,
                    leftover0,
                    leftover1,
                } => {
                    ranges[i] = leftover0;
                    ranges.push(leftover1);
                    out.push(subtracted.uniform_add(self.offset));
                }
            }
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

    fn process_ranges(&self, ranges: &mut Vec<Range<isize>>, out: &mut Vec<Range<isize>>) {
        for converter in &self.0 {
            converter.process_ranges(ranges, out);
        }
    }
} //}}}

fn solutionate<S: Deref<Target = str>, I: IntoIterator<Item = S>>(
    input: I,
) -> Result<isize, String> {
    let mut input_iter = input.into_iter();

    let first_line = &*input_iter.next().ok_or("Empty input".to_string())?;
    let first_line = first_line
        .strip_prefix("seeds: ")
        .ok_or_else(|| format!("Failed to parse seeds on the first line: {first_line}."))?;
    let mut first_line_iter = first_line.split_whitespace();

    let mut seed_parsing_has_error = false;
    let seed_iter = {
        let error = &mut seed_parsing_has_error;
        let mut next_int = || {
            if let Ok(v) = first_line_iter.next()?.parse() {
                Some(v)
            } else {
                *error = true;
                None
            }
        };
        from_fn(move || {
            let start = next_int()?;
            let len = next_int()?;
            Some(start..(start + len))
        })
    };

    let mut seed_ranges: Vec<Range<isize>> = seed_iter.collect();
    if seed_parsing_has_error {
        return Err(format!(
            "Failed to parse seeds on the first line: {first_line}."
        ));
    }

    if seed_ranges.is_empty() {
        return Err(format!("No seeds found in: {first_line}"));
    }

    let mappers: Vec<Mapper> = from_fn(|| Mapper::from_str_iter(&mut input_iter)).collect();
    let mut staging = Vec::new();
    for mapper in mappers {
        mapper.process_ranges(&mut seed_ranges, &mut staging);

        seed_ranges.append(&mut staging);
        seed_ranges.defragment_ranges();
    }

    Ok(seed_ranges
        .into_iter()
        .map(|x| x.start)
        .min()
        .expect("`seed_ranges` is not empty"))
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
    //{{{
    use super::*;

    #[test]
    fn test_subtract_range_0() {
        assert_eq!(SubtractionResult::None, (0..10).subtract_range(&(20..30)))
    }
    #[test]
    fn test_subtract_range_1() {
        assert_eq!(SubtractionResult::None, (50..100).subtract_range(&(20..30)))
    }
    #[test]
    fn test_subtract_range_2() {
        assert_eq!(SubtractionResult::Full, (70..75).subtract_range(&(50..100)))
    }
    #[test]
    fn test_subtract_range_3() {
        assert_eq!(
            SubtractionResult::Leftover1 {
                subtracted: 70..75,
                leftover0: 50..70,
                leftover1: 75..100
            },
            (50..100).subtract_range(&(70..75))
        )
    }
    #[test]
    fn test_subtract_range_4() {
        assert_eq!(
            SubtractionResult::Leftover0 {
                subtracted: 50..75,
                leftover: 75..100
            },
            (50..100).subtract_range(&(50..75))
        )
    }
    #[test]
    fn test_subtract_range_5() {
        assert_eq!(
            SubtractionResult::Leftover0 {
                subtracted: 90..100,
                leftover: 50..90
            },
            (50..100).subtract_range(&(90..1000))
        )
    }
    #[test]
    fn test_subtract_range_6() {
        assert_eq!(
            SubtractionResult::Leftover0 {
                subtracted: 79..80,
                leftover: 80..93
            },
            (79..93).subtract_range(&(60..80))
        )
    }
    #[test]
    fn test_subtract_range_7() {
        assert_eq!(
            SubtractionResult::Leftover0 {
                subtracted: 60..68,
                leftover: 55..60
            },
            (55..68).subtract_range(&(60..80))
        )
    }

    #[test]
    fn test_converter_process_ranges_0() {
        let converter = Converter {
            source_range: 60..80,
            offset: 100,
        };

        let mut input = vec![79..93, 55..68];
        let mut out = Vec::new();
        converter.process_ranges(&mut input, &mut out);

        assert!(input.contains(&(80..93)));
        assert!(input.contains(&(55..60)));
        assert!(out.contains(&(179..180)));
        assert!(out.contains(&(160..168)));
    }

    #[test]
    fn test_mapper_from_str_iter_0() {
        let s = "";
        assert_eq!(None, Mapper::from_str_iter(s.lines()));
    }

    #[test]
    fn test_mapper_from_str_iter_1() {
        let s = "seed-to-soil map:";
        assert_eq!(None, Mapper::from_str_iter(s.lines()));
    }

    #[test]
    fn test_mapper_from_str_iter_2() {
        let s = "
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:";
        let mut iter = s.lines();
        let mapper = Mapper::from_str_iter(&mut iter).unwrap();
        assert_eq!(
            vec![
                Converter {
                    source_range: 98..100,
                    offset: -48
                },
                Converter {
                    source_range: 50..98,
                    offset: 2
                }
            ],
            mapper.0
        );
        assert_eq!(Some("soil-to-fertilizer map:"), iter.next());
    }

    // #[test]
    // fn test_mapper_map_0() {
    //     let mut ranges = vec![79..93, 55..68];
    //     let mut out = Vec::new();
    //     let mapper = Mapper(vec![
    //         Converter {
    //             source_range: 98..100,
    //             offset: -48,
    //         },
    //         Converter {
    //             source_range: 50..98,
    //             offset: 2,
    //         },
    //     ]);
    //     mapper.process_ranges(&mut ranges, &mut out);
    // }

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
        assert_eq!(Ok(46), solutionate(s.lines()));
    }
} //}}}
