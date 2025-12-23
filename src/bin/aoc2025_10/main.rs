use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 10;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Debug, Clone)]
struct LightPanel {
    lights: u32,
    buttons: Vec<u32>,
}

impl LightPanel {
    fn new(light_diagram: Vec<bool>, wiring_diagram: Vec<Vec<u32>>) -> LightPanel {
        let lights = light_diagram
            .iter()
            .enumerate()
            .fold(0, |acc, (n, c)| acc | if *c { 1 << n } else { 0 });
        let buttons = wiring_diagram
            .iter()
            .map(|y| y.iter().fold(0, |acc, x| acc | (1 << x)))
            .collect_vec();
        LightPanel {
            lights: lights,
            buttons: buttons,
        }
    }

    fn least_presses(
        &self,
        target: u32,
        buttons: &[u32],
        state: u32,
        index: usize,
        count: u32,
        best: &mut u32,
    ) {
        if count >= *best {
            // continue the loop, no reason to check more buttons
        } else if state == target {
            *best = count;
        } else if index < buttons.len() {
            self.least_presses(
                target,
                buttons,
                state ^ buttons[index],
                index + 1,
                count + 1,
                best,
            );
            self.least_presses(target, buttons, state, index + 1, count, best);
        }
    }

    fn solve(&mut self) -> u32 {
        let mut best = u32::MAX;
        self.least_presses(self.lights, &self.buttons, 0, 0, 0, &mut best);

        best
    }
}

#[derive(Debug, Clone)]
struct JoltagePanel {
    joltages: Vec<u32>,
    buttons: Vec<Vec<u32>>,
}

impl JoltagePanel {
    fn new(joltages: Vec<u32>, buttons: Vec<Vec<u32>>) -> JoltagePanel {
        JoltagePanel { joltages, buttons }
    }

    fn solve(&self) -> Option<u64> {
        // Build the augmented matrix
        // Rows = equations (counters)
        // Cols = variables (buttons) + 1 (target)
        let num_eq = self.joltages.len();
        let num_vars = self.buttons.len();
        let mut matrix = vec![vec![0.0; num_vars + 1]; num_eq];

        for (j, button) in self.buttons.iter().enumerate() {
            // button is a list of counter indices it affects
            // so for each counter i in button, coeff of button_j in eq_i is 1
            for &counter_idx in button {
                if (counter_idx as usize) < num_eq {
                    matrix[counter_idx as usize][j] = 1.0;
                }
            }
        }

        // Fill target column
        for i in 0..num_eq {
            matrix[i][num_vars] = self.joltages[i] as f64;
        }

        // Gaussian Elimination
        let mut pivot_row = 0;
        let mut pivot_cols = vec![];
        let mut free_vars = vec![];

        for col in 0..num_vars {
            if pivot_row >= num_eq {
                free_vars.push(col);
                continue;
            }

            // Find pivot
            let mut max_row = pivot_row;
            for r in pivot_row + 1..num_eq {
                if matrix[r][col].abs() > matrix[max_row][col].abs() {
                    max_row = r;
                }
            }

            if matrix[max_row][col].abs() < 1e-9 {
                free_vars.push(col);
                continue;
            }

            // Swap rows
            matrix.swap(pivot_row, max_row);
            pivot_cols.push(col);

            // Notify pivot
            // let pivot_val = matrix[pivot_row][col]; // Keep it 1.0 logic below

            // Normalize row
            let pivot_val = matrix[pivot_row][col];
            for c in col..=num_vars {
                matrix[pivot_row][c] /= pivot_val;
            }

            // Eliminate other rows
            for r in 0..num_eq {
                if r != pivot_row {
                    let factor = matrix[r][col];
                    if factor.abs() > 1e-9 {
                        for c in col..=num_vars {
                            matrix[r][c] -= factor * matrix[pivot_row][c];
                        }
                    }
                }
            }

            pivot_row += 1;
        }

        // After elimination, check for consistency in zero rows
        for r in pivot_row..num_eq {
            if matrix[r][num_vars].abs() > 1e-9 {
                return None;
            }
        }

        // Search for minimal integer solution
        // We have `pivot_vars` expressed as:
        // x_pivot = target - sum(coeff * x_free)
        // We need x_i >= 0 and integer.

        // If no free vars, it's just the constants in the pivot rows
        if free_vars.is_empty() {
            let mut total_presses = 0.0;
            for r in 0..num_vars {
                let val = matrix[r][num_vars];
                if val < -1e-9 || (val.round() - val).abs() > 1e-9 {
                    return None; // Invalid
                }
                total_presses += val.round();
            }
            return Some(total_presses as u64);
        }

        // If there are free vars, we iterate.
        // Problem implies small numbers, so simple brute force on free vars might work.
        // Or Recurse.
        let mut min_total = u64::MAX;

        // LIMIT search depth? The inputs are small-ish integers usually.
        // Let's recursive search assignments for free vars.

        self.search_free_vars(
            &matrix,
            &free_vars,
            &pivot_cols,
            0,
            &mut vec![0.0; num_vars],
            &mut min_total,
        );

        if min_total == u64::MAX {
            None
        } else {
            Some(min_total)
        }
    }

    fn search_free_vars(
        &self,
        matrix: &Vec<Vec<f64>>,
        free_vars: &[usize],
        pivot_cols: &[usize], // map row_idx -> variable index for pivot
        fv_idx: usize,
        current_sol: &mut Vec<f64>,
        best_total: &mut u64,
    ) {
        if fv_idx == free_vars.len() {
            // Calculate dependent variables (pivots)
            let mut round_sum = 0;
            // Reconstruct pivots from RREF matrix
            // matrix[r][pivot_col] is 1. We solved for this.
            // x_pivot + sum(matrix[r][free] * x_free) = matrix[r][target]
            // x_pivot = matrix[r][target] - sum(matrix[r][free] * x_free)

            // Map pivot row to pivot col
            // pivot_cols[r] gives the variable index for the pivot at row r
            for (r, &p_col) in pivot_cols.iter().enumerate() {
                let mut val = matrix[r][matrix[0].len() - 1]; // constant
                for &fv in free_vars {
                    val -= matrix[r][fv] * current_sol[fv];
                }

                // Check if int and >= 0
                if val < -1e-4 {
                    return;
                } // Negative
                let rounded = val.round();
                if (val - rounded).abs() > 1e-4 {
                    return;
                } // Not integer

                current_sol[p_col] = rounded;
                round_sum += rounded as u64;
            }

            // Add free vars to sum
            for &fv in free_vars {
                round_sum += current_sol[fv] as u64;
            }

            if round_sum < *best_total {
                *best_total = round_sum;
            }
            return;
        }

        let fv = free_vars[fv_idx];
        // Heuristic: Try a range of values for free var.
        // Since we want MINIMUM presses, start small.
        for val in 0..2000 {
            // Limit to 2000 presses per free button?
            current_sol[fv] = val as f64;
            // Optimization: check partial sum?
            let mut current_sum = 0.0;
            for i in 0..=fv_idx {
                current_sum += current_sol[free_vars[i]];
            }
            if current_sum as u64 >= *best_total {
                break;
            }

            self.search_free_vars(
                matrix,
                free_vars,
                pivot_cols,
                fv_idx + 1,
                current_sol,
                best_total,
            );
        }
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<Vec<u32>>, Vec<u32>) {
    let (lights, remainder) = input.split_once(" ").unwrap();
    let lvs = &lights.as_bytes()[1..lights.len() - 1]
        .iter()
        .map(|c| *c == '#' as u8)
        .collect_vec();
    let (wiring, joltage) = remainder.split_once("{").unwrap();
    let w = wiring
        .trim()
        .split(" ")
        .map(|p| {
            p[1..p.len() - 1]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let j = joltage[..joltage.len() - 1]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    (lvs.clone(), w.clone(), j.clone())
}

fn part1(data: &String) -> u64 {
    let panels = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|l| parse_input(l))
        .map(|(l, w, _)| LightPanel::new(l, w))
        .collect_vec();
    println!("{:?}", panels);

    let mut acc = 0;
    for mut p in panels {
        acc += p.solve();
    }

    acc as u64
}

fn part2(data: &String) -> u64 {
    let panels = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|l| parse_input(l))
        .map(|(_, w, j)| JoltagePanel::new(j, w))
        .collect_vec();

    let mut total = 0;
    for (i, p) in panels.iter().enumerate() {
        if let Some(sol) = p.solve() {
            total += sol;
        } else {
            panic!("Failed to solve panel index {}", i);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_10_ep1() {
        assert_eq!(7, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_10_ep2() {
        assert_eq!(33, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_10_rp1() {
        assert_eq!(459, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_10_rp2() {
        assert_eq!(18687, part2(&get_input(YEAR, DAY)));
    }
}
