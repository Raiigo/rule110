const WIDTH: usize = 128;
const LINES: u32 = 128;

fn compute_next_row(current_row: &[u8; WIDTH], next_row: &mut [u8; WIDTH]) {
    for i in 0..WIDTH {
        let mut scope: (u8, u8, u8) = (0, 0, 0);
        if i == 0 {
            scope.0 = 0;
        } else {
            scope.0 = current_row[i - 1];
        }
        scope.1 = current_row[i];
        if i == WIDTH - 1 {
            scope.2 = 0;
        } else {
            scope.2 = current_row[i + 1];
        }
        next_row[i] = match scope {
            (0, 0, 0) => 0,
            (0, 0, 1) => 1,
            (0, 1, 0) => 1,
            (0, 1, 1) => 1,
            (1, 0, 0) => 0,
            (1, 0, 1) => 1,
            (1, 1, 0) => 1,
            (1, 1, 1) => 0,
            _ => 0,
        }
    }
}

fn main() {
    let mut current_row: [u8; WIDTH] = [0; WIDTH];
    current_row[WIDTH - 1] = 1;
    let mut next_row: [u8; WIDTH] = [0; WIDTH];
    compute_next_row(&current_row, &mut next_row);
    dbg!(&next_row);
    for _ in 0..LINES {
        let line_vec: Vec<char> = [' '; WIDTH].iter_mut().enumerate().map(|(i, e)| {
            match current_row[i] {
                0 => ' ',
                1 => '*',
                _ => 'E',
            }
        }).collect();
        let line: String = line_vec.iter().collect();
        println!("{}", line);
        compute_next_row(&current_row, &mut next_row);
        current_row = next_row.clone();
    }
}
