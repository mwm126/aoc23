use std::cmp::min;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Solution part 1:{}", closest_location(&input));
    println!("Solution part 2:{}", closest_location2(&input));
    Ok(())
}

fn closest_location(input: &str) -> usize {
    let mut min_loc = 100000000000;
    let almanac = parse_almanac(input);

    for seed in &almanac.seeds {
        min_loc = min(min_loc, almanac.location(*seed))
    }
    min_loc
}

fn closest_location2(input: &str) -> usize {
    let mut min_loc = 100000000000;
    let almanac = parse_almanac(input);

    for chunk in almanac.seeds.chunks(2) {
        let start = chunk[0];
        let len = chunk[1];
        for seed in start..start + len {
            println!("CONSIDERING SEED: {}", seed);
            min_loc = min(min_loc, almanac.location(seed))
        }
    }

    min_loc
}

fn to_num(nn: &str) -> usize {
    nn.parse().expect("Invalid num")
}

fn parse_map<R>(reader: &mut BufReader<R>) -> Map
where
    R: std::io::Read,
{
    let mut maps = Vec::new();
    let mut line = "".to_string();
    reader.read_line(&mut line).expect("bad header");
    // println!("HEADER: {}", line);
    loop {
        line = "".to_string();
        reader.read_line(&mut line).expect("bad line");
        line = line.trim().to_string();
        if line.is_empty() {
            // println!("BLANK: {}", line);
            break;
        }
        // println!("MAP: {}", line);
        let mut fields = line.split_whitespace();
        let dst = to_num(fields.next().expect("invalid dst"));
        let src = to_num(fields.next().expect("invalid src"));
        let len = to_num(fields.next().expect("invalid len"));
        maps.push((dst, src, len));
    }
    Map { maps }
}

fn parse_almanac(input: &str) -> Almanac {
    let input_s = input.to_string();
    let input = input_s.as_bytes();
    let mut reader = BufReader::new(input);
    let mut line = "".to_string();
    reader.read_line(&mut line).expect("invalid input");
    let (_, seeds) = line.split_once(':').expect("Bad seeds");
    let seeds = seeds.split_whitespace().map(to_num).collect();
    reader.read_line(&mut line).expect("must be blank");
    // println!("EXPECT BLANK: {}", line);
    let seed2soil = parse_map(&mut reader);
    let soil2fert = parse_map(&mut reader);
    let fert2h2oh = parse_map(&mut reader);
    let h2oh2lite = parse_map(&mut reader);
    let lite2temp = parse_map(&mut reader);
    let temp2humi = parse_map(&mut reader);
    let humi2loc = parse_map(&mut reader);

    Almanac {
        seeds,
        seed2soil,
        soil2fert,
        fert2h2oh,
        h2oh2lite,
        lite2temp,
        temp2humi,
        humi2loc,
    }
}

struct Almanac {
    seeds: Vec<usize>,
    seed2soil: Map,
    soil2fert: Map,
    fert2h2oh: Map,
    h2oh2lite: Map,
    lite2temp: Map,
    temp2humi: Map,
    humi2loc: Map,
}

impl Almanac {
    fn location(&self, seed: usize) -> usize {
        self.humi2loc.lookup(
            self.temp2humi.lookup(
                self.lite2temp.lookup(
                    self.h2oh2lite.lookup(
                        self.fert2h2oh
                            .lookup(self.soil2fert.lookup(self.seed2soil.lookup(seed))),
                    ),
                ),
            ),
        )
    }
}

#[derive(Debug)]
struct Map {
    maps: Vec<(usize, usize, usize)>,
}

impl Map {
    fn lookup(&self, src_num: usize) -> usize {
        for (dst, src, len) in &self.maps {
            if src <= &src_num && src_num <= src + len {
                return dst + (src_num - src);
            }
        }
        src_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
seeds: 79 14 55 13

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
56 93 4
";

    #[test]
    fn test_maps() {
        let alma = parse_almanac(EXAMPLE_INPUT);

        assert_eq!(alma.seed2soil.lookup(82), 84);
        assert_eq!(alma.soil2fert.lookup(84), 84);
        assert_eq!(alma.fert2h2oh.lookup(84), 84);
        assert_eq!(alma.h2oh2lite.lookup(84), 77);
        println!("LITE2TEMP: {:#?}", alma.lite2temp);
        assert_eq!(alma.lite2temp.lookup(77), 45);
        assert_eq!(alma.temp2humi.lookup(45), 46);
        assert_eq!(alma.humi2loc.lookup(46), 46);

        assert_eq!(alma.seed2soil.lookup(79), 81);
        assert_eq!(alma.soil2fert.lookup(81), 81);
        assert_eq!(alma.fert2h2oh.lookup(81), 81);
        assert_eq!(alma.h2oh2lite.lookup(81), 74);
        assert_eq!(alma.lite2temp.lookup(74), 78);
        assert_eq!(alma.temp2humi.lookup(78), 78);
        assert_eq!(alma.humi2loc.lookup(78), 82);

        assert_eq!(alma.seed2soil.lookup(14), 14);
        assert_eq!(alma.soil2fert.lookup(14), 53);
        assert_eq!(alma.fert2h2oh.lookup(53), 49);
        assert_eq!(alma.h2oh2lite.lookup(49), 42);
        assert_eq!(alma.lite2temp.lookup(42), 42);
        assert_eq!(alma.temp2humi.lookup(42), 43);
        assert_eq!(alma.humi2loc.lookup(43), 43);

        assert_eq!(alma.seed2soil.lookup(55), 57);
        assert_eq!(alma.soil2fert.lookup(57), 57);
        assert_eq!(alma.fert2h2oh.lookup(57), 53);
        assert_eq!(alma.h2oh2lite.lookup(53), 46);
        assert_eq!(alma.lite2temp.lookup(46), 82);
        assert_eq!(alma.temp2humi.lookup(82), 82);
        assert_eq!(alma.humi2loc.lookup(82), 86);

        assert_eq!(alma.seed2soil.lookup(13), 13);
        assert_eq!(alma.soil2fert.lookup(13), 52);
        assert_eq!(alma.fert2h2oh.lookup(52), 41);
        assert_eq!(alma.h2oh2lite.lookup(41), 34);
        assert_eq!(alma.lite2temp.lookup(34), 34);
        assert_eq!(alma.temp2humi.lookup(34), 35);
        assert_eq!(alma.humi2loc.lookup(35), 35);
    }

    #[test]
    fn test_almanac() {
        let result = closest_location(EXAMPLE_INPUT);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_almanac2() {
        let result = closest_location2(EXAMPLE_INPUT);
        assert_eq!(result, 46);
    }
}
