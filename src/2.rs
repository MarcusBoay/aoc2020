mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|line| line.split_ascii_whitespace().map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<Vec<String>>) {
    let mut valids = 0;
    for line in data {
        let mut count = 0;
        let nums = line[0].split('-').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let n = line[1].chars().collect::<Vec<char>>()[0].to_digit(36).unwrap() - 10;
        for c in line[2].chars() {
            let c = c.to_digit(36).unwrap() - 10;
            count += (c == n) as usize;
        }
        valids += (count >= nums[0] && count <= nums[1]) as i32;
    }
    println!("Number of valid passwords: {valids}");
}

fn part2(data: &Vec<Vec<String>>)  {
    let mut valids = 0;
    for line in data {
        let nums = line[0].split('-').map(|x| x.parse::<usize>().unwrap()-1).collect::<Vec<usize>>();
        let c = line[1].chars().collect::<Vec<char>>()[0];
        let password = line[2].chars().collect::<Vec<char>>();
        valids += ((password[nums[0]] == c) ^ (password[nums[1]] == c)) as i32;
    }
    println!("Number of valid passwords: {valids}");
}