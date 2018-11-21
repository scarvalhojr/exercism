pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return vec![];
    }

    let num_cols = input[0].len();
    assert!(
        input.iter().all(|row| row.len() == num_cols),
        "Invalid matrix"
    );

    let col_min: Vec<_> = (0..num_cols)
        .map(|col_num|
            input.iter().map(|row| row[col_num]).min().unwrap()
        ).collect();

    input.iter()
        .enumerate()
        .flat_map(|(row_num, row)| {
            let row_max = *row.iter().max().unwrap();
            row.iter()
                .enumerate()
                .filter(|(col_num, &val)|
                    val == col_min[*col_num] && val == row_max
                ).map(|(col_num, _)| (row_num, col_num))
                .collect::<Vec<_>>()
        }).collect()
}

pub fn find_saddle_points_iter(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = vec![];

    if input.is_empty() || input[0].is_empty() {
        return saddle_points;
    }

    let num_cols = input[0].len();
    assert!(
        input.iter().all(|row| row.len() == num_cols),
        "Invalid matrix"
    );

    let mut col_min = input[0].clone();
    let mut col_points = vec![vec![]; num_cols];

    for (row_num, row) in input.iter().enumerate() {
        let mut max_val = row[0];
        let mut max_cols = vec![];

        for (col_num, val) in row.iter().enumerate() {
            if *val < col_min[col_num] {
                col_min[col_num] = *val;
                col_points[col_num].clear();
            }

            if *val > max_val {
                max_val = *val;
                max_cols.clear();
            }

            if *val == max_val && *val == col_min[col_num] {
                max_cols.push(col_num);
            }
        }

        for col_num in max_cols {
            col_points[col_num].push(row_num);
        }
    }

    for (col_num, points) in col_points.iter().enumerate() {
        for row_num in points {
            saddle_points.push((*row_num, col_num));
        }
    }

    saddle_points
}
