use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated = Vec::new();
    let mines = (0..)
        .zip(minefield.iter())
        .flat_map(|(row_num, row)| {
            (0..)
                .zip(row.chars())
                .filter(|(_, cell)| *cell == '*')
                .map(move |(col_num, _)| (row_num, col_num))
        })
        .collect::<HashSet<(isize, isize)>>();

    for (row_num, row) in (0_isize..).zip(minefield.iter()) {
        let mut line = String::new();
        for (col_num, cell) in (0_isize..).zip(row.chars()) {
            if cell == '*' {
                line.push('*');
                continue;
            }

            let mut count = 0;
            for r in row_num - 1..=row_num + 1 {
                for c in col_num - 1..=col_num + 1 {
                    if mines.contains(&(r, c)) {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                line.push(' ');
            } else {
                line.push_str(&count.to_string());
            }
        }
        annotated.push(line);
    }
    annotated
}
