use std::collections::HashMap;
use std::io::Lines;
use std::iter::Map;
use std::os::unix::raw::off_t;
use regex::Regex;

#[derive(Debug)]
struct Almanac<'a> {
    pub seeds: Vec<usize>,
    pub dicts: HashMap<(&'a str, &'a str), Vec<RangeMapper>>,
}

impl<'a> Almanac<'a> {
    pub fn create_path_to(self: &Self, dest: &str) -> Option<Vec<&str>> {
        let mappers = self.dicts.keys().collect::<Vec<&(&str, &str)>>();

        let mut open = Vec::<&str>::new();
        let mut visited = Vec::<&str>::new();
        let mut paths = Vec::<Vec<&str>>::new();

        open.push("seed");
        paths.push(vec!["seed"]);

        while !visited.contains(&dest) && !open.is_empty() { // while we haven't found the destination and there are still nodes to explore
            let curr = open.pop().unwrap();

            // We already have a shorter path to this node
            if visited.contains(&curr) {
                continue;
            }

            for &mapper in &mappers {
                if mapper.0 == curr { // we found a transition from curr -> other

                    // push new state to visited and open queue
                    visited.push(mapper.1);
                    open.push(mapper.1);

                    // Append to paths whose current state is new state
                    for mut path in &mut paths {
                        if path.last().unwrap() == &curr {
                            path.push(mapper.1);
                        }
                    }
                }
            }
        }
        
        let path = paths.iter().find(|x| x.last().unwrap() == &dest);
        println!("path: {:?}", path);
        
        if let Some(path) = path {
            return Some(path.clone());
        }
        
        None
        


        // given start node "seeds"
        // find transitions from "seeds" to other nodes
        // while transitions does not contain end state
        // find transitions from current leaves to other nodes not in visited list
    }

    // pub fn translate(self: &Self, id: usize, to: &str) -> usize {
    //     let mut curr = "seed";
    // 
    //     while curr != to {
    //         if let Some(mapper) = self.dicts.get(curr) {}
    //     }
    // }
}

#[derive(Debug)]
struct RangeMapper {
    pub src: usize,
    pub dest: usize,
    pub range: usize,
}

impl RangeMapper {
    fn new(line: &str) -> RangeMapper {
        let regex = Regex::new(r"([\s\d]*)$").unwrap();
        if let Some(captures) = regex.captures(line.trim()) {
            let numbers = captures.get(1).unwrap()
                .as_str().split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();


            // let numbers = captures.iter()
            //     .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
            //     .collect::<Vec<usize>>();

            if numbers.is_empty() {
                panic!("no numbers found in line: {}", line);
            }

            let dest = numbers[0];
            let src = numbers[1];
            let range = numbers[2];

            return RangeMapper {
                src,
                dest,
                range,
            };
        }

        panic!("no numbers found in line: {}", line)
    }

    pub fn contains(self: &Self, id: usize) -> bool {
        id >= self.src && id < self.src + self.range
    }

    pub fn map(self: &Self, id: usize) -> Option<usize> {
        if !self.contains(id) {
            return None;
        }

        let offset = id - self.src;

        Some(self.dest + offset)
    }
}

fn get_section_name(input: &str) -> Option<(&str, &str)> {
    if !input.contains(":") {
        return None;
    }

    if input.contains("seeds") {
        return Some(("seeds", ""));
    }

    let section_name_regex = Regex::new(r"(\w+)-to-(\w+)").unwrap();
    if let Some(caps) = section_name_regex.captures(input) {
        let from = caps.get(1).unwrap().as_str();
        let to = caps.get(2).unwrap().as_str();

        println!("from: {}, to: {}", from, to);
        return Some((from, to));
    }

    return None;
}

fn get_numbers(input: &str) -> Option<Vec<usize>> {
    let regex = Regex::new(r"([\s\d]+)$").unwrap();


    if let Some(captures) = regex.captures(input) {
        let numbers = captures.get(1).unwrap().as_str().split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        // 
        // let numbers = captures.iter()
        //     .map(|c| c.unwrap().as_str().trim().parse::<usize>().unwrap())
        //     .collect::<Vec<usize>>();

        println!("captured numbers: {:?}", captures.get(1));

        if numbers.is_empty() { return None; } else { return Some(numbers); }
    }

    None
}

pub fn parse(input: &str) -> Almanac {
    let mut maps = HashMap::<(&str, &str), Vec<RangeMapper>>::new();
    let mut seeds = Vec::<usize>::new();

    let mut section: Option<(&str, &str)> = None;
    for line in input.lines() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Found a section header - assign as current
        if let Some(result) = get_section_name(line) {
            section = Some(result);
        }

        // Does line contain numbers?
        if let Some(numbers) = get_numbers(line) {
            // No active section - parsing issue
            if section == None {
                panic!("no section found for line: {}", line);
            }

            let section_name = section.unwrap();
            if section_name == ("seeds", "") { // handle special "seeds" section
                seeds = numbers
            } else { // handle all other sections
                if maps.contains_key(&section_name) {
                    maps.get_mut(&section_name).unwrap()
                        .push(RangeMapper::new(line));
                } else {
                    maps.insert(section.unwrap(), vec![RangeMapper::new(line)]);
                }
            }
        }
    }

    Almanac {
        seeds,
        dicts: maps,
    }
}

//         
// pub fn parse(input: &str) -> HashMap<(&str, &str), Vec<RangeMapper>> {
//     let mut maps = HashMap::<(&str, &str), Vec<RangeMapper>>::new();
//     let mut seeds = Vec::<usize>::new();
//     
//     let mut section: (&str, &str) = ("","");
//     input.lines().for_each(|line| {
//         if line.is_empty() {
//             return;
//         }
// 
//         let parts = line.split(":").collect::<Vec<&str>>();
//         let mut section_parts = parts
//             .get(0).unwrap()
//             
//             .trim()
//             // .replace(" map", "")
//             .split("-")
//             // .map(|s| s.replace(" map", ""))
//             .into_iter()
//             .collect::<Vec<&str>>();
// 
//         match section_parts.as_slice() {
//             ["seeds"] => {
//                 seeds = parts[1].split_whitespace()
//                     .map(|s| s.parse::<usize>().unwrap())
//                     .collect::<Vec<usize>>();
//             },
//             [from, "to", to] => {
//                 // let v = to.replace(" map", "");
//                 section = (from, to);
//             }
//             _ => {
//                 let range = RangeMapper::new(line);
//                 if let Some(map) = maps.get_mut(&section) {
//                     map.push(range);
//                 } else {
//                     maps.insert(section, Vec::<RangeMapper>::new());
//                 }
//             }
//         }
//     });
//     
//     maps
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mapper() {
        let foo = RangeMapper::new("50 98 2");
        assert_eq!(foo.src, 98);
        assert_eq!(foo.dest, 50);
        assert_eq!(foo.range, 2);
    }

    #[test]
    fn contains() {
        let foo = RangeMapper::new("50 98 2");
        assert_eq!(foo.contains(97), false);
        assert!(foo.contains(98));
        assert!(foo.contains(99));
        assert_eq!(foo.contains(100), false);
    }

    #[test]
    fn map() {
        let foo = RangeMapper::new("50 98 2");
        assert_eq!(foo.map(97), None);
        assert_eq!(foo.map(98), Some(50));
        assert_eq!(foo.map(99), Some(51));
        assert_eq!(foo.map(100), None);
    }

    #[test]
    fn parse2() {
        let input = r"seeds:
                1 2   03   4 5
                
                foo-to-bar map: 
                7 8 9
                
                bar-to-baz map:   10 11 12
                    13    14   15";

        let atlas = parse(input);
        println!("{:?}", atlas);

        assert_eq!(atlas.seeds, vec![1, 2, 3, 4, 5]);

        assert!(atlas.dicts.contains_key(&("foo", "bar")));
        assert_eq!(atlas.dicts.get(&("foo", "bar")).unwrap().len(), 1);
        assert_eq!(atlas.dicts.get(&("foo", "bar")).unwrap().get(0).unwrap().src, 8);
        assert_eq!(atlas.dicts.get(&("foo", "bar")).unwrap().get(0).unwrap().dest, 7);
        assert_eq!(atlas.dicts.get(&("foo", "bar")).unwrap().get(0).unwrap().range, 9);

        assert!(atlas.dicts.contains_key(&("bar", "baz")));
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().len(), 2);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(0).unwrap().src, 11);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(0).unwrap().dest, 10);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(0).unwrap().range, 12);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(1).unwrap().src, 14);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(1).unwrap().dest, 13);
        assert_eq!(atlas.dicts.get(&("bar", "baz")).unwrap().get(1).unwrap().range, 15);
    }
    
    #[test]
    fn test_path_1() {
        let input = r"seeds: 1
           foo-to-bar map:
        20 0 10
        
          bar-to-baz map:
        20 30 10";
        
        let almanac = parse(input);
        println!("{:?}", almanac);
        let path = almanac.create_path_to("bar");
        println!("{:?}", path);
    }

    #[test]
    fn pdsaarse2() {
        /* given seed ids 79, 14, 55, 13
         *      
         * each map is encoded as ranges:
         * e.g. seed-to-soil-map:
         *   convert a seedid to soilid
         *   50 98 2
         *     soilid range starts at 50 (e.g. 50, 51)
         *     seedid range starts at 98  (e.g 98, 99)
         *     soilid and seedid range is 2 
         *       so seed 98->soil 50,
         *          seed 99->soil 51
         *
         * e.g. soil-to-fertilizer map:
         *    0 15 37
         *    fertilizerid starts at 0 (0->36)
         *    soilid starts at 15 (15->51)
         */

        let input = r"seeds: 79 14 55 13

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

        let atlas = parse(input);
        println!("{:?}", atlas);
    }
}
