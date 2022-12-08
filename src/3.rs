mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    part1(&data);
    part2(&data);
}

fn get_encounters(data: &Vec<Vec<char>>, step_sizes: (usize, usize)) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut count = 0 as usize;

    while i < data.len() {
        count += (data[i][j] == '#') as usize;
        j = (j+step_sizes.1) % data[i].len();
        i += step_sizes.0;
    }
    count
}

fn part1(data: &Vec<Vec<char>>) {
    let trees = get_encounters(&data, (1, 3));
    println!("Total tree encounters: {trees}");
}

fn part2(data: &Vec<Vec<char>>) {
    let mut trees = 1;
    trees *= get_encounters(data, (1, 1));
    trees *= get_encounters(data, (1, 3));
    trees *= get_encounters(data, (1, 5));
    trees *= get_encounters(data, (1, 7));
    trees *= get_encounters(data, (2, 1));
    println!("Total multiplied tree encounters: {trees}");
}