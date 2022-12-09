use std::collections::HashSet;

mod utils;

type VS = Vec<String>;
type VVS = Vec<VS>;

fn main() {
    let data = utils::fast_get_file_data_as_string()
        .split('\n')
        .map(String::from)
        .collect::<VS>();
    part1(&data);
    part2(&data);
}

fn part1(data: &VS) {
    let required = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let mut cur = required.clone();
    let mut valids = 0;
    for line in data {
        if line.is_empty() {
            valids += (cur.len() == 0) as i32;
            cur = required.clone();
        }
        let entries = line
            .split_ascii_whitespace()
            .map(|x| x.split(':').map(String::from).collect::<VS>())
            .collect::<VVS>();
        for entry in entries {
            cur.remove(entry[0].as_str());
        }
    }
    println!("Number of valid passports: {valids}");
}

fn part2(data: &VS) {
    let required = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let mut cur = required.clone();
    let mut valids = 0;
    let mut i = 0;
    for line in data {
        if line.is_empty() {
            println!("i:{i}, is_valid:{}", cur.is_empty());
            valids += cur.is_empty() as i32;
            cur = required.clone();
            i += 1;
        }
        let entries = line
            .split_ascii_whitespace()
            .map(|x| x.split(':').map(String::from).collect::<VS>())
            .collect::<VVS>();
        for entry in entries {
            let (key, value) = (entry[0].clone(), entry[1].clone());
            let mut should_remove = false;
            match key.as_str() {
                "byr" => {
                    if let Ok(value) = value.parse::<u32>() {
                        if (1930..=2002).contains(&value) {
                            should_remove = true;
                        }
                    }
                }
                "iyr" => {
                    if let Ok(value) = value.parse::<u32>() {
                        if (2010..=2020).contains(&value) {
                            should_remove = true;
                        }
                    }
                }
                "eyr" => {
                    if let Ok(value) = value.parse::<u32>() {
                        if (2020..=2030).contains(&value) {
                            should_remove = true;
                        }
                    }
                }
                "hgt" => {
                    if value.len() >= 4 {
                        let ty = &value[value.len() - 2..];
                        if let Ok(value) = value[0..value.len() - 2].parse::<u32>() {
                            if (ty == "cm" && (150..=193).contains(&value))
                                || (ty == "in" && (59..=76).contains(&value))
                            {
                                should_remove = true;
                            }
                        }
                    }
                }
                "hcl" => {
                    if value.len() == 7 && value.starts_with('#') {
                        let value = value[1..].chars().collect::<Vec<char>>();
                        let mut is_valid = true;
                        for c in value {
                            if !"0123456789abcdef".contains(c) {
                                is_valid = false;
                                break;
                            }
                        }
                        if is_valid {
                            should_remove = true;
                        }
                    }
                }
                "ecl" => {
                    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()) {
                        should_remove = true;
                    }
                }
                "pid" => {
                    if value.len() == 9 && value.parse::<u32>().is_ok() {
                        should_remove = true;
                    }
                }
                _ => (),
            }
            if should_remove {
                cur.remove(key.as_str());
            } else {
                println!("not removing {key}:{value}");
            }
        }
    }
    println!("Number of valid passports: {valids}");
}
