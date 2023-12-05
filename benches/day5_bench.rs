use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    iter::from_fn,
    ops::{Add, Deref, Range},
    str::FromStr,
};

// Copying the code here because I can't figure out how to access interfaces of a binary target
// from here. //{{{
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

pub fn solutionate<S: Deref<Target = str>, I: IntoIterator<Item = S>>(
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

    let mappers = from_fn(|| Mapper::from_str_iter(&mut input_iter));
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
} //}}}

// Input: //{{{
const INPUT: &str = "seeds: 3489262449 222250568 2315397239 327729713 1284963 12560465 1219676803 10003052 291763704 177898461 136674754 107182783 2917625223 260345082 1554280164 216251358 3900312676 5629667 494259693 397354410

seed-to-soil map:
0 262295201 34634737
910271444 3030771176 70771974
1897698334 3827766493 333942393
2835207376 3155028665 271883030
3622783763 1954868220 45665001
413310490 329609945 44648194
4240712808 2423731337 1828518
1316121579 1728110187 80499681
2250966941 2228145658 118984728
2516210021 2142630093 85515565
1864536239 3101543150 33162095
34634737 296929938 32680007
329609945 374258139 83700545
3329626425 2142012590 617503
2601725586 1531355997 196754190
3709721314 2000533221 47790630
3583007889 3526843691 39775874
4060705901 1494770107 36585890
3330243928 2694462608 4855344
3668448764 1219440337 41272550
3872593248 2347130386 76600951
67314744 0 262295201
1396621260 4161708886 65683132
3549978104 3678131267 33029785
3949194199 3566619565 111511702
1482627812 2425559855 249576539
3757511944 4236759076 58208220
875524978 2831649840 34746466
4242541326 4227392018 9367058
3107090406 1260712887 107855696
4097291791 2887350159 143421017
4251908384 3483784779 43058912
981043418 749361185 218472720
749361185 1368568583 126163793
3335099272 1004561505 214878832
2798479776 967833905 36727600
3214946102 1494732376 37731
3308672572 2866396306 20953853
2369951669 1808609868 146258352
1199516138 3711161052 116605441
1732204351 2699317952 132331888
3214983833 2048323851 93688739
1462304392 3134705245 20323420
3815720164 3426911695 56873084
2231640727 2675136394 19326214

soil-to-fertilizer map:
1819561283 2841304997 237877444
4006405251 2649445491 24162567
212683490 0 763350919
1389184545 2619909475 29536016
1221487606 2673608058 167696939
3182207211 2119363521 157025339
2057563716 1221487606 435495557
976034409 1008691842 1136514
2493059273 3079182441 511127728
3339232550 1835003373 284360148
3623592698 3912154743 382812553
3004187001 1656983163 178020210
977170923 987370593 21321249
58558242 763350919 154125248
998492172 976034409 11336184
4030567818 2276513849 264399478
0 917476167 58558242
2057438727 2276388860 124989
1418720561 2540913327 78996148
1497716709 3590310169 321844574

fertilizer-to-water map:
252374398 77740491 188270615
1590959511 1400999811 20005707
1019974286 266011106 27332620
1085156732 1443065767 85008355
4080487124 1647556561 104750479
3094480335 3707305578 360771904
4185237603 4248557616 46409680
3828418017 2355725650 21816275
1626753532 4068077482 180480134
1568899262 1421005518 22060249
1807233666 3705123159 2182419
1809416085 1626753532 20803029
3998962584 2274201110 81524540
440645013 1177871937 219366450
1520096568 1397238387 3761424
1523857992 1528074122 45041270
174633907 0 77740491
1170165087 293343726 349931481
3850234292 2125472818 148728292
2953301907 3563944731 141178428
660011463 817909114 359962823
4231647283 3500624718 63320013
0 643275207 174633907
1830219114 2377541925 1123082793
1047306906 1573115392 37849826
3455252239 1752307040 373165778

water-to-light map:
3713102322 3195199062 109343869
940512817 264084495 97517772
2334334472 1383468484 100556669
465645319 1958405710 14984685
3303747025 4294646763 320533
2616072044 2768562044 426637018
1599136731 361602267 86797445
480630004 53689017 41315440
244424239 1902592526 55813184
521945444 1484025153 418567373
3208097188 2507524081 95649837
3933959318 3799485319 28619103
3822446191 4288382866 6263897
53689017 2385517268 49373873
1109731683 1973390395 151930940
3865952749 4063808346 68006569
2013172224 496719573 124139241
2223150247 620858814 76066751
196104378 448399712 48319861
2299216998 2350399794 35117474
3710229881 3304542931 2872441
2137311465 1005624092 85838782
1516868628 923355989 82268103
103062890 830314501 93041488
2459504093 4131814915 156567951
1457198065 2290729231 59670563
3042709062 2603173918 165388126
1261662623 1357013080 26455404
419510562 2244594474 46134757
3304067558 2459504093 48019988
3828710088 3665557707 37242661
3352087546 3307415372 358142335
1819323112 1091462874 193849112
3962578421 3702800368 96684951
1038030589 1285311986 71701094
4059263372 3828104422 235703924
300237423 2125321335 119273139
1288118027 95004457 169080038
1685934176 696925565 133388936

light-to-temperature map:
933106075 308278269 212548971
3133283890 2353712179 197530061
2425741949 3555777393 99769003
932513834 722519986 592241
2970285248 1818047303 41216585
2702880712 2943782997 36451052
1420185365 1065216599 145812917
1172773874 1429440708 136557574
3565704029 2551242240 35862216
0 723112227 1274616
354926437 520827240 17525539
3896280620 3932683708 23800931
891510185 724386843 1792027
1145655046 1211029516 27118828
1587093819 3655546396 28108512
372451976 682239540 40280446
95189373 726178870 259737064
3672435146 2980234049 58823724
2559751700 2800653985 143129012
3920081551 3363893446 191883947
1274616 538352779 2436403
1615202331 3685523873 66027002
2231672737 2159642967 194069212
3601566245 4075105446 70868901
2028619121 3160839830 203053616
2739331764 1587093819 230953484
3811900561 3956484639 84380059
893302212 540789182 39211622
412732422 1361179950 68260758
4111965498 3751550875 181132833
2525510952 4040864698 34240748
789271449 580000804 102238736
480993180 0 308278269
1309331448 1329626698 31553252
3731258870 2775912020 24741965
3756000835 1859263888 55899726
1681229333 2587104456 188807564
3330813951 4145974347 148992949
1340884700 985915934 79300665
3711019 1238148344 91478354
3011501833 3039057773 121782057
1870036897 1915163614 158582224
4293098331 3683654908 1868965
3479806900 2073745838 85897129

temperature-to-humidity map:
3171909692 2207313208 125557542
3910448973 3971234589 267130124
2271924206 3732981386 64142303
1112427243 457977609 299980445
533481406 191448702 131397640
2336066509 3020855282 21496528
26829166 1125772826 208642920
3547574211 3901910422 69324167
235472086 0 100639414
3346614381 3623211928 109769458
3472548902 3256633041 51082372
2371444946 4238364713 56602583
0 100639414 26829166
3658517236 3410551372 212660556
3102881240 3307715413 69028452
2428047529 2368475416 534316082
336111500 322846342 55397842
4250847842 2163193754 44119454
3871177792 3042351810 39271181
3616898378 3105565928 41618858
2962363611 3376743865 33807507
1412407688 1334415746 499813714
3523631274 3081622991 23942937
1032693818 378244184 79733425
2161270365 2902791498 80399705
3456383839 3147184786 16165063
2998094507 3797123689 104786733
664879046 757958054 367814772
2996171118 2161270365 1923389
391509342 1834229460 77991942
2357563037 3212496996 13881909
469501284 127468580 63980122
3297467234 3163349849 49147147
4177579097 2332870750 35604666
2241670070 3226378905 30254136
4213183763 2983191203 37664079

humidity-to-location map:
4240687605 3509581493 54279691
3450687144 1997031321 128004903
3703408300 2316680098 55200017
2797906577 2125036224 66927621
3758608317 1680202206 316829115
2970872896 1200387958 479814248
2864834198 1094349260 106038698
3578692047 2191963845 124716253
4075437432 3563861184 165250173
2232050638 3729111357 565855939
1094349260 2371880115 1137701378
"; //}}}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 5", |b| {
        b.iter(|| solutionate(black_box(INPUT.lines())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
