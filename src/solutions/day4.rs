pub fn part1(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => 1,
                    'M' => 2,
                    'A' => 4,
                    'S' => 16,
                    _ => 0,
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("{:?}", matrix);

    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // check horizontally
            if get_at(&matrix, i, j)
                + get_at(&matrix, i, j + 1)
                + get_at(&matrix, i, j + 2)
                + get_at(&matrix, i, j + 3)
                == 23
            {
                println!("found horizontally at {}, {}", i, j);
                count += 1;
            }

            // check vertically
            if get_at(&matrix, i, j)
                + get_at(&matrix, i + 1, j)
                + get_at(&matrix, i + 2, j)
                + get_at(&matrix, i + 3, j)
                == 23
            {
                println!("found vertically at {}, {}", i, j);
                count += 1;
            }

            // check diagonally
            if get_at(&matrix, i, j)
                + get_at(&matrix, i + 1, j + 1)
                + get_at(&matrix, i + 2, j + 2)
                + get_at(&matrix, i + 3, j + 3)
                == 23
            {
                println!("found diagonally at {}, {}", i, j);
                println!(
                    "{} {} {} {}",
                    get_at(&matrix, i, j),
                    get_at(&matrix, i + 1, j + 1),
                    get_at(&matrix, i + 2, j + 2),
                    get_at(&matrix, i + 3, j + 3)
                );
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|n| n.chars().map(|c|  match c {
            'M' => 1,
            'A' => 10,
            'S' => 100,
            _ => -1
        }).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut count = 0;
    for i in 1 .. matrix.len() - 1 {
        for j in 1 .. matrix[i].len() - 1 {
            let slice = &matrix[i - 1 .. i + 2].iter().map(|l| &l[j - 1 .. j + 2]).collect::<Vec<_>>();

            if check_one(slice) {
                count += 1
            }
        }
    }

    // let i = 7; let j = 7;
    // let slice = &matrix[i - 1 .. i + 2].iter().map(|l| &l[j - 1 .. j + 2]).collect::<Vec<_>>();
    //
    // println!("{:?}", slice);
    //
    // check_one(slice);


    count
}

fn check_one(m: &Vec<&[i32]>) -> bool {
    if m[1][1] == 10 &&  m[0][0] + m[1][1] + m[2][2] == 111 && m[2][0] + m[1][1] + m[0][2] == 111 {
        return true;
    }

    false
}

fn get_at(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i < matrix.len() && j < matrix[i].len() {
        return matrix[i][j];
    }

    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            18
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            ),
            9
        );
    }
}
