use std::num::ParseIntError;
pub fn is_right_visible(
    matrix: &Vec<Vec<i32>>,
    value: i32,
    mut index_row: usize,
    index_column: usize,
    row_len: usize,
) -> bool {
    if index_row == row_len - 1 && value > matrix[index_column][index_row] {
        true
    } else if value <= matrix[index_column][index_row] {
        false
    } else {
        is_right_visible(matrix, value, index_row + 1, index_column, row_len)
    }
}
pub fn is_down_visible(
    matrix: &Vec<Vec<i32>>,
    value: i32,
    index_row: usize,
    mut index_column: usize,
    col_len: usize,
) -> bool {
    if index_row == col_len - 1 && value > matrix[index_column][index_row] {
        true
    } else if value <= matrix[index_column][index_row] {
        false
    } else {
        is_right_visible(matrix, value, index_row, index_column + 1, col_len)
    }
}
pub fn populate_visible_trees_and_matrix(input: &str)-> (Vec<Vec<i32>>,Vec<Vec<bool>>) {
    let mut matrix: Vec<Vec<i32>> = vec![];
    let mut visible_trees: Vec<Vec<bool>> = vec![];
    input
        .lines()
        .enumerate()
        .map(|(j,line)| {
            let vector: Vec<i32> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();
            (j,vector)
        })
        .for_each(|(j,vector)| {
            let visible_trees_row:Vec<bool> = vector.clone().iter()
                .enumerate()
                .map(|(i,_)| {
                    if i == 0 || i == vector.len() || j == 0 {
                        true
                    }
                    else {
                        false
                    }
                } ).collect();
            matrix.push(vector);
            visible_trees.push(visible_trees_row);

        });
    (matrix,visible_trees)
}
pub fn how_many_visible_tree(input: &str) -> i32 {
    let (matrix,mut visible_trees) = populate_visible_trees_and_matrix(input);
    println!("{:?}", matrix);
    println!("{:?}", visible_trees);
    let (mut i, mut j) = (1, 1);
    let (col_len, row_len) = (matrix.len(), matrix[0].len());
    let mut number_visible_trees = 2 * (col_len + row_len) - 4;
    while true {
        let current_tree = matrix[i][j];
        if visible_trees[j][i - 1] && current_tree > matrix[j][i - 1]
            || visible_trees[j - 1][i] && current_tree > matrix[j - 1][i]
            || is_right_visible(&matrix, current_tree, i, j, row_len)
            || is_down_visible(&matrix, current_tree, i, j, col_len)
        {
            visible_trees[j][i] = true;
            number_visible_trees += 1
        }
        i += 1;
        if i == row_len - 1 as usize {
            i = 1;
            j += 1;
        }
        if j == col_len - 1 as usize {
            break;
        }
    }
    number_visible_trees.try_into().unwrap()
}
