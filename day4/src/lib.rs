use rayon::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &str) {
        let grid: Vec<&[u8]> = file.lines().map(|line| line.as_bytes()).collect();
        let mut check = 0usize;
        let (row, col) = (grid.len(), grid[0].len());
        let dir = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == b'.' {
                    continue;
                }
                let mut ok = 0_u8;
                for &(dx, dy) in &dir {
                    let (nr, nc) = (i as isize + dx, dy + j as isize);
                    if nr < 0 || nc < 0 || nr >= row as isize || nc >= col as isize {
                        continue;
                    }

                    if grid[nr as usize][nc as usize] == b'@' {
                        ok += 1;
                    }
                }
                if ok < 4 {
                    check += 1;
                }
            }
        }

        dbg!(&check);
    }

    pub fn second_part(file: &str) {
        let mut grid: Vec<Vec<u8>> = file.lines().map(|line| line.as_bytes().to_vec()).collect();
        let mut f_val = 0;

        let (row, col) = (grid.len(), grid[0].len());
        let dir = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        loop {
            let results: Vec<(usize, usize)> = (0..row)
                .into_par_iter()
                .flat_map(|i| {
                    let mut local = Vec::new();
                    for j in 0..col {
                        if grid[i][j] == b'.' {
                            continue;
                        }
                        let mut ok = 0_u8;
                        for (x, y) in dir {
                            let (nr, nc) = (i as i32 + x, y + j as i32);
                            if nr as usize >= row || nc as usize >= col || nr < 0 || nc < 0 {
                                continue;
                            }
                            if grid[nr as usize][nc as usize] == b'@' {
                                ok += 1;
                            }
                        }
                        if ok < 4 {
                            local.push((i, j));
                        }
                    }
                    local
                })
                .collect();
            let check = results.len();
            f_val += check;
            if check == 0 {
                break;
            }

            for &(i, j) in &results {
                grid[i][j] = b'.';
            }
        }
        dbg!(&f_val);
    }
}
