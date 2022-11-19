pub const DIRECTIONAL_OFFSET: [i8; 2] = [7, 9];

const fn calc_const_matrix() -> [[usize; 4]; 64] {
    let mut matrix = [[0usize; 4]; 64];
    let mut row = 0usize;
    while row < 8 {
        let mut column = 0;
        while column < 8 {
            let index = column * 8 + row;
            let dist_north = 7 - column;
            let dist_south = column;
            let dist_east = 7 - row;
            let dist_west = row;

            matrix[index] = [
                if dist_north < dist_west {
                    dist_north
                } else {
                    dist_west
                },
                if dist_north < dist_east {
                    dist_north
                } else {
                    dist_east
                },
                if dist_south < dist_east {
                    dist_south
                } else {
                    dist_east
                },
                if dist_south < dist_west {
                    dist_south
                } else {
                    dist_west
                },
            ];
            column += 1;
        }
        row += 1
    }
    matrix
}

pub const NUM_SQUARES_TO_EDGE: [[usize; 4]; 64] = calc_const_matrix();
