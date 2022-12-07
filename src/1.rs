mod utils;

const TARGET: i32 = 2020;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<i32>) {
    let n = data.len();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                if data[i] + data[j] == TARGET {
                    println!("Answer: {}", data[i]*data[j]);
                    return;
                }
            }
        }
    }
}

fn part2(data: &Vec<i32>) {
    let n = data.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i != j && j != k {
                    if data[i] + data[j] + data[k] == TARGET {
                        println!("Answer: {}", data[i]*data[j]*data[k]);
                        return;
                    }
                }
            }
        }
    }
}