#![allow(unused_mut, unused_variables, dead_code, unused_must_use)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input-prod.txt");
    println!("smallest part2_optimized: {}", part2_optimized(input));
    // println!("smallest: {}", part2(input));
}

fn part2_optimized(input: &str) -> u64 {
    let mut lines = input.lines();
    // parse_seeds
    let seeds = parse_seeds(&mut lines);

    // skip one line.
    lines.next().unwrap();
    let seed_to_soil_map = parse_seed_to_soil_map(&mut lines);
    let soil_to_seed_map = invert_map_rows(seed_to_soil_map);

    let soil_to_fertilizers_map = parse_soil_to_fertilizers_map(&mut lines);
    let fertilizer_to_soil_map = invert_map_rows(soil_to_fertilizers_map);

    let fertilizer_to_water_map = parse_fertilizer_to_water_map(&mut lines);
    let water_to_fertilizer_map = invert_map_rows(fertilizer_to_water_map);

    lines.next().unwrap();
    let water_to_light_map = parse_map(&mut lines);
    let light_to_water_map = invert_map_rows(water_to_light_map);
    lines.next().unwrap();

    let light_to_temperature_map = parse_map(&mut lines);
    let temperature_to_light_map = invert_map_rows(light_to_temperature_map);
    lines.next().unwrap();

    let temperatur_to_humidity_map = parse_map(&mut lines);
    let humidity_to_temperature_map = invert_map_rows(temperatur_to_humidity_map);
    lines.next().unwrap();
    let humidity_to_location_map = parse_map(&mut lines);
    let mut location_to_humidity_map = invert_map_rows(humidity_to_location_map);
    location_to_humidity_map.sort_by(|a, b| a.src.cmp(&b.src));
    // println!("location_to_humidity_map: {:?}", location_to_humidity_map);
    // println!(
    //     "humidity_to_temperature_map: {:?}",
    //     humidity_to_temperature_map
    // );
    // println!("temperature_to_light_map: {:?}", temperature_to_light_map);

    let seed_ranges = seeds
        .chunks(2)
        .map(|ch| (ch[0], ch[1]))
        .collect::<Vec<(u64, u64)>>();
    for row in location_to_humidity_map {
        for counter in 0..row.range {
            let location = row.src + counter;
            let humiditiy = row.dest + counter;

            let temprature = get_dest(humidity_to_temperature_map.iter(), humiditiy);
            let light = get_dest(temperature_to_light_map.iter(), temprature);
            let water = get_dest(light_to_water_map.iter(), light);
            let fertilizer = get_dest(water_to_fertilizer_map.iter(), water);
            let soil = get_dest(fertilizer_to_soil_map.iter(), fertilizer);
            let seed = get_dest(soil_to_seed_map.iter(), soil);
            // println!(">>> location: {location}, humiditiy: {humiditiy}, temprature: {temprature}, light: {light}, water: {water}, fertilizer: {fertilizer}, soil: {soil}, seed: {seed}");
            if does_seed_exists(&seed_ranges, seed) {
                return location;
            }
        }
    }
    0
    // unreachable!()
}

fn does_seed_exists(seeds_range: &Vec<(u64, u64)>, seed: u64) -> bool {
    for seed_range in seeds_range.iter() {
        if seed >= seed_range.0 && seed < seed_range.0 + seed_range.1 {
            println!("found seed: {seed}, in range: {seed_range:?}");
            return true;
        }
    }
    false
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    // parse_seeds
    let seeds = parse_seeds(&mut lines);

    // skip one line.
    lines.next().unwrap();
    let seed_to_soil_map = parse_seed_to_soil_map(&mut lines);
    let soil_to_fertilizers_map = parse_soil_to_fertilizers_map(&mut lines);
    let fertilizer_to_water_map = parse_fertilizer_to_water_map(&mut lines);
    lines.next().unwrap();
    let water_to_light_map = parse_map(&mut lines);
    lines.next().unwrap();
    let light_to_temperature_map = parse_map(&mut lines);
    lines.next().unwrap();
    let temperatur_to_humidity_map = parse_map(&mut lines);
    lines.next().unwrap();
    let humidity_to_location_map = parse_map(&mut lines);

    let seed_ranges = seeds
        .chunks(2)
        .map(|ch| (ch[0], ch[1]))
        .collect::<Vec<(u64, u64)>>();
    // println!("seeds_range: {:?}", seed_ranges);

    let mut lowest_seed: Option<u64> = None;
    for seed_range in seed_ranges.iter() {
        for seed in seed_range.0..seed_range.0 + seed_range.1 {
            let soil = get_dest(seed_to_soil_map.iter(), seed);
            let fertilizer = get_dest(soil_to_fertilizers_map.iter(), soil);
            let water = get_dest(fertilizer_to_water_map.iter(), fertilizer);
            let light = get_dest(water_to_light_map.iter(), water);
            let temperature = get_dest(light_to_temperature_map.iter(), light);
            let humidity = get_dest(temperatur_to_humidity_map.iter(), temperature);
            let location = get_dest(humidity_to_location_map.iter(), humidity);

            lowest_seed = match lowest_seed {
                Some(low) => {
                    if location < low {
                        Some(location)
                    } else {
                        Some(low)
                    }
                }
                None => Some(location),
            }
        }
    }
    lowest_seed.unwrap()
}

fn part1(input: &str) -> u64 {
    let mut smallest = 0;
    let mut lines = input.lines();
    // parse_seeds
    let seeds = parse_seeds(&mut lines);

    // skip one line.
    lines.next().unwrap();
    let seed_to_soil_map = parse_seed_to_soil_map(&mut lines);
    // println!("{:?}", seed_to_soil_map);
    // lines.next().unwrap();
    let soil_to_fertilizers_map = parse_soil_to_fertilizers_map(&mut lines);
    // println!("{:?}", soil_to_fertilizers_map);
    let fertilizer_to_water_map = parse_fertilizer_to_water_map(&mut lines);
    lines.next().unwrap();
    let water_to_light_map = parse_map(&mut lines);
    // println!("{:?}", water_to_light_map);

    lines.next().unwrap();
    let light_to_temperature_map = parse_map(&mut lines);
    // println!("{:?}", light_to_temperature_map);
    // let keys_count = seed_to_soil_map.keys().len();
    lines.next().unwrap();
    let temperatur_to_humidity_map = parse_map(&mut lines);
    // println!("{:?}", temperatur_to_humidity_map);

    lines.next().unwrap();
    let humidity_to_location_map = parse_map(&mut lines);
    println!("{:?}", humidity_to_location_map);

    let mut lowest_seed: Option<u64> = None;
    for seed in seeds.iter() {
        let soil = get_dest(seed_to_soil_map.iter(), *seed);
        let fertilizer = get_dest(soil_to_fertilizers_map.iter(), soil);
        let water = get_dest(fertilizer_to_water_map.iter(), fertilizer);
        let light = get_dest(water_to_light_map.iter(), water);
        let temperature = get_dest(light_to_temperature_map.iter(), light);
        let humidity = get_dest(temperatur_to_humidity_map.iter(), temperature);
        let location = get_dest(humidity_to_location_map.iter(), humidity);

        lowest_seed = match lowest_seed {
            Some(low) => {
                if location < low {
                    Some(location)
                } else {
                    Some(low)
                }
            }
            None => Some(location),
        }
    }
    // let keys_count = seed_to_soil_map.keys().len();

    // println!("keys: {keys_count}");

    // parse_seed_to_soil_map
    // skip one line
    // parse_soil_to_fertilizers_map
    // skip one line
    // ....
    //

    lowest_seed.unwrap()
}

fn get_dest<'a, T>(mut iterator: T, src: u64) -> u64
where
    T: Iterator<Item = &'a MapRow>,
{
    iterator
        .find(|row| row.get_dest(src).is_some())
        .map(|row| row.get_dest(src))
        .or_else(|| Some(Some(src)))
        .unwrap()
        .unwrap()
    // .or_else(|| src)
    // todo!()
}

fn parse_fertilizer_to_water_map<'a, T>(mut iterator: T) -> Vec<MapRow>
where
    T: Iterator<Item = &'a str>,
{
    iterator.next().unwrap();
    parse_map(&mut iterator)
}

fn parse_soil_to_fertilizers_map<'a, T>(mut iterator: T) -> Vec<MapRow>
where
    T: Iterator<Item = &'a str>,
{
    iterator.next().unwrap();
    parse_map(&mut iterator)
}
fn parse_seed_to_soil_map<'a, T>(mut iterator: T) -> Vec<MapRow>
where
    T: Iterator<Item = &'a str>,
{
    iterator.next().unwrap();
    parse_map(&mut iterator)
}

#[derive(Debug)]
struct MapRow {
    dest: u64,
    src: u64,
    range: u64,
}

impl MapRow {
    fn get_dest(&self, number: u64) -> Option<u64> {
        if number < self.src || number > self.src + self.range - 1 {
            return None;
        }
        Some(number - self.src + self.dest)
    }
}

fn invert_map_rows(mut rows: Vec<MapRow>) -> Vec<MapRow> {
    for row in rows.iter_mut() {
        let temp = row.dest;
        row.dest = row.src;
        row.src = temp;
    }
    rows
}

fn parse_map<'a, T>(mut iterator: T) -> Vec<MapRow>
where
    T: Iterator<Item = &'a str>,
{
    let mut map: Arc<Mutex<HashMap<u64, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut rows: Vec<MapRow> = iterator
        .take_while(|line| line.trim() != "")
        .map(|line| {
            let mut items = line.split(' ');
            let mut dest = items.next().unwrap().parse::<u64>().unwrap();
            let mut source = items.next().unwrap().parse::<u64>().unwrap();
            let mut range = items.next().unwrap().parse::<u64>().unwrap();
            MapRow {
                dest,
                src: source,
                range,
            }
        })
        .collect::<Vec<MapRow>>();
    rows
}

fn parse_seeds<'a>(mut iterator: impl Iterator<Item = &'a str>) -> Vec<u64> {
    let seeds_line = iterator.next().unwrap();
    seeds_line
        .split(' ')
        .skip(1)
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .collect()
}
