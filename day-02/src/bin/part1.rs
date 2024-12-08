use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process(input: String) -> String {
    let reports = input.lines();
    let mut invalid_reports = 0;
    let mut nbr_of_reports = 0;
    println!("Number of reports: {:?}", nbr_of_reports);
    for report in reports {
        nbr_of_reports = nbr_of_reports + 1;
        let levels_iter = report.split_ascii_whitespace();
        let levels: Vec<&str> = levels_iter.collect();
        let mut level_index = 0;
        let mut level = levels[level_index].parse::<i32>().unwrap();
        let mut direction = 0;
        let mut invalid = false;
        while level_index < levels.len() - 1 {
            level_index += 1;
            let next_level = levels[level_index].parse::<i32>().unwrap();
            let diff = next_level - level;

            if diff.abs() == 0 || diff.abs() > 3 {
                invalid_reports = invalid_reports + 1;
                invalid = true;
                break;
            }

            if direction == 0 {
                if diff > 0 {
                    direction = 1;
                } else if diff < 0 {
                    direction = -1;
                }
            } else if direction == 1 {
                if diff < 0 {
                    invalid_reports = invalid_reports + 1;
                    invalid = true;
                    break;
                }
            } else if direction == -1 {
                if diff > 0 {
                    invalid_reports = invalid_reports + 1;
                    invalid = true;
                    break;
                }
            }

            level = next_level;
        }
    }

    let valid_reports = nbr_of_reports - invalid_reports;

    let res = format!(
        "Reports (valid/invalid): {:?}({:?}/{:?})",
        nbr_of_reports,
        nbr_of_reports - invalid_reports,
        invalid_reports
    )
    .to_string();
    return valid_reports.to_string();
}

fn main() {
    let input = read_to_string("./inputs/day-02.txt").unwrap();
    let res = process(input);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        assert_eq!(process(input), "2");
    }
}
