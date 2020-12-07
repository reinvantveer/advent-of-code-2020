use std::fs;
use std::ops::Range;

fn main() {
    let lines = read_lines("input.txt");
    let seat_ids = lines.iter()
        .map(|line| parse_seat_id(line.to_string()));

    let highest_seat_id = seat_ids.clone().max().unwrap();
    println!("Highest seat id: {}", &highest_seat_id);

    let seat_ids_as_vec: Vec<usize> = seat_ids.collect();
    let full_list: Range<i32> = 0..1024;
    let mut missing: Vec<i32> = full_list
        .filter(|seat| !seat_ids_as_vec.contains(&(*seat as usize)))
        .collect();
    missing.sort();

    println!("Missing: {:?}, {} seats", &missing, &missing.len());

}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn parse_seat_id(seat: String) -> usize {
    println!("{}: {} chars", seat, seat.len());
    let mut rows: Vec<i32> = (0..128).collect();
    let mut columns: Vec<i32> = (0..8).collect();
    println!("columns: {:?}", columns);

    let mut seat_id: usize = 0;

    for (idx, element) in seat.chars().enumerate() {
        // Parse the row
        println!("char id {}", idx);
        if idx <= 6 {
            let halfway = rows.len() / 2;

            if element.to_string() == "F" {
                rows = rows[0..halfway].to_vec();
                // println!("{} rows", rows.len());
            } else {
                rows = rows[halfway..].to_vec();
                // println!("{} rows", rows.len());
            }

        } else {
            if seat_id == 0 {
                seat_id = partial_seat_id_from_rows(&mut rows);
            }

            let halfway_cols = columns.len() / 2;
            println!("{} halfway columns", halfway_cols);

            if element.to_string() == "L" {
                columns = columns[0..halfway_cols].to_vec();
            } else {
                columns = columns[halfway_cols..].to_vec();
            }

            if idx == 9 { // Last element is done
                assert_eq!(columns.len(), 1);
                let remaining_column = columns.iter().clone().next().unwrap();
                let col_as_usize = *remaining_column as usize;
                println!("column: {}", col_as_usize);
                seat_id += col_as_usize;
            }
        }
    }
    seat_id
}

fn partial_seat_id_from_rows(rows: &mut Vec<i32>) -> usize {
    assert_eq!(rows.len(), 1);
    let remaining_row = rows.iter().clone().next().unwrap();
    println!("{}", &remaining_row);
    *remaining_row as usize * 8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_works() {
        let lines = read_lines("example.txt");
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn correct_seat_id(){
        assert_eq!(parse_seat_id("BFFFBBFRRR".to_string()), 567)
    }

    #[test]
    fn glue_together() {
        let lines = read_lines("example.txt");
        let seat_ids: Vec<usize> = lines.iter()
            .map(|line| parse_seat_id(line.to_string()))
            .collect();

        assert_eq!(seat_ids, vec![567, 119, 820]);
    }
}