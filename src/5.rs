use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let seats = get_seat_ids(&data);
    get_max_seat_id(&seats);
    get_my_seat_id(&seats);
}

fn get_seat_ids(data: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut seats: Vec<(i32, i32)> = vec![];
    for seat in data {
        let mut i = 0;
        let mut j = 127;
        let mut k = 0;
        while k < 7 {
            let mid = (i + j) / 2;
            if seat[k] == 'F' {
                j = mid - 1;
            } else if seat[k] == 'B' {
                i = mid + 1;
            }
            k += 1;
        }
        let row = i;
        i = 0;
        j = 7;
        while k < 10 {
            let mid = (i + j) / 2;
            if seat[k] == 'L' {
                j = mid - 1;
            } else if seat[k] == 'R' {
                i = mid + 1;
            }
            k += 1;
        }
        let col = i;

        seats.push((row, col));
    }
    seats
}

fn get_max_seat_id(seats: &Vec<(i32, i32)>) {
    let mut max_seat = 0;
    for (row, col) in seats {
        max_seat = std::cmp::max(max_seat, *row * 8 + *col);
    }
    println!("Highest seat ID: {max_seat}");
}

fn get_my_seat_id(seats: &Vec<(i32, i32)>) {
    let mut possible_seats: Vec<i32> = vec![];
    for (row, col) in seats {
        if *row > 0 && *row < 127 {
            possible_seats.push(*row * 8 + *col);
        }
    }
    let seat_set: HashSet<i32> = HashSet::from_iter(possible_seats.iter().cloned());
    possible_seats = Vec::from_iter(seat_set.iter().cloned());
    possible_seats.sort();

    for i in 1..possible_seats.len() {
        if possible_seats[i] + 1 != possible_seats[i + 1] {
            println!("Your seat ID: {}", possible_seats[i] + 1);
            return;
        }
    }
}
