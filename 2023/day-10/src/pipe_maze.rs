#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Tile {
    row: usize,
    column: usize,
    tile_type: TileType,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileType {
    Vertical,
    Horizontal,
    BendPipe90NE,
    BendPipe90NW,
    BendPipe90SE,
    BendPipe90SW,
    Ground,
    Start,
}

impl From<char> for TileType {
    fn from(c: char) -> Self {
        if c == '|' {
            TileType::Vertical
        } else if c == '-' {
            TileType::Horizontal
        } else if c == 'L' {
            TileType::BendPipe90NE
        } else if c == 'J' {
            TileType::BendPipe90NW
        } else if c == '7' {
            TileType::BendPipe90SW
        } else if c == 'F' {
            TileType::BendPipe90SE
        } else if c == '.' {
            TileType::Ground
        } else {
            // char == 'S'
            TileType::Start
        }
    }
}

pub struct PipeMaze {
    tiles: Vec<Vec<Tile>>,
    pipe: Vec<Tile>,
}

impl PipeMaze {
    pub fn parse(maze_layout: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for (row, maze_line) in maze_layout.trim().split('\n').enumerate() {
            tiles.push(vec![]);
            maze_line
                .chars()
                .enumerate()
                .for_each(|(column, character)| {
                    tiles[row].push(Tile {
                        row,
                        column,
                        tile_type: TileType::from(character),
                    })
                });
        }

        let pipe = PipeMaze::find_pipe(&tiles);
        let start_tile = tiles
            .iter()
            .flatten()
            .find(|tile| tile.tile_type == TileType::Start)
            .unwrap();
        let start_tile_type = PipeMaze::get_start_tile_implicit_type(&tiles, start_tile);

        let mut start_tile = tiles
            .iter_mut()
            .flatten()
            .find(|tile| tile.tile_type == TileType::Start)
            .unwrap();
        start_tile.tile_type = start_tile_type;

        PipeMaze { tiles, pipe }
    }

    pub fn max_len_from_start(&self) -> u64 {
        (self.pipe.len() / 2) as u64
    }

    pub fn number_of_tiles_in_pipe(&self) -> u64 {
        let mut tile_inside_loop_count = 0_u64;

        for row in &self.tiles {
            tile_inside_loop_count += self.count_tiles_inside_loop_in_row(row);
        }
        tile_inside_loop_count
    }

    fn find_pipe(tiles: &[Vec<Tile>]) -> Vec<Tile> {
        let mut visited_tiles: Vec<Tile> = Vec::new();
        let mut start_tile = *tiles
            .iter()
            .flatten()
            .find(|tile| tile.tile_type == TileType::Start)
            .unwrap();

        start_tile.tile_type = PipeMaze::get_start_tile_implicit_type(tiles, &start_tile);
        visited_tiles.push(start_tile);

        let mut current_tile = start_tile;
        while let Some(tile) = PipeMaze::find_next_pipe_tile(tiles, &current_tile, &visited_tiles) {
            visited_tiles.push(tile);
            current_tile = tile;
        }

        visited_tiles
    }

    fn count_tiles_inside_loop_in_row(&self, tile_row: &[Tile]) -> u64 {
        let mut tile_counter = 0_u64;
        for index in 0..tile_row.len() {
            if self.pipe.contains(tile_row.get(index).unwrap()) {
                continue;
            }

            let mut crossed_loop = 0_u64;
            let mut remaining_row_tiles_iter = tile_row
                .iter()
                .skip(index + 1)
                .filter(|tile| self.pipe.contains(tile) && tile.tile_type != TileType::Horizontal)
                .peekable();

            while let Some(tile) = remaining_row_tiles_iter.next() {
                let next_tile_type_option = remaining_row_tiles_iter
                    .peek()
                    .map(|next_tile| next_tile.tile_type);

                if PipeMaze::is_crossing_pipe(tile.tile_type, next_tile_type_option) {
                    crossed_loop += 1;
                }
            }

            tile_counter = if crossed_loop % 2 != 0 {
                tile_counter + 1
            } else {
                tile_counter
            }
        }

        tile_counter
    }

    fn is_crossing_pipe(tile_type: TileType, next_tile_type_option: Option<TileType>) -> bool {
        if tile_type == TileType::Vertical {
            return true;
        }

        if let Some(next_tile_type) = next_tile_type_option {
            if (tile_type == TileType::BendPipe90NE && next_tile_type == TileType::BendPipe90SW)
                || (tile_type == TileType::BendPipe90SE && next_tile_type == TileType::BendPipe90NW)
            {
                return true;
            }
        }

        false
    }

    fn get_start_tile_implicit_type(tiles: &[Vec<Tile>], start_tile: &Tile) -> TileType {
        let mut tile_type = TileType::Start;

        let tile_to_the_left_opt = tiles
            .iter()
            .flatten()
            .find(|tile| tile.row == start_tile.row && tile.column + 1 == start_tile.column);

        let tile_to_the_right_opt = tiles
            .iter()
            .flatten()
            .find(|tile| tile.row == start_tile.row && tile.column == start_tile.column + 1);

        let tile_above_opt = tiles
            .iter()
            .flatten()
            .find(|tile| tile.row + 1 == start_tile.row && tile.column == start_tile.column);

        let tile_below_opt = tiles
            .iter()
            .flatten()
            .find(|tile| tile.row == start_tile.row + 1 && tile.column == start_tile.column);

        if tile_to_the_left_opt.is_some() && tile_to_the_right_opt.is_some() {
            let tile_to_the_left = tile_to_the_left_opt.unwrap();
            let tile_to_the_right = tile_to_the_right_opt.unwrap();

            if (tile_to_the_left.tile_type == TileType::BendPipe90SE
                || tile_to_the_left.tile_type == TileType::Horizontal
                || tile_to_the_left.tile_type == TileType::BendPipe90NE)
                && (tile_to_the_right.tile_type == TileType::BendPipe90NW
                    || tile_to_the_right.tile_type == TileType::Horizontal
                    || tile_to_the_right.tile_type == TileType::BendPipe90SW)
            {
                tile_type = TileType::Horizontal;
            }
        }

        if tile_above_opt.is_some() && tile_below_opt.is_some() {
            let tile_above = tile_above_opt.unwrap();
            let tile_below = tile_below_opt.unwrap();

            if (tile_above.tile_type == TileType::Vertical
                || tile_above.tile_type == TileType::BendPipe90SW
                || tile_above.tile_type == TileType::BendPipe90SE)
                && (tile_below.tile_type == TileType::Vertical
                    || tile_below.tile_type == TileType::BendPipe90NE
                    || tile_below.tile_type == TileType::BendPipe90NW)
            {
                tile_type = TileType::Vertical;
            }
        }

        if tile_above_opt.is_some() && tile_to_the_left_opt.is_some() {
            let tile_above = tile_above_opt.unwrap();
            let tile_to_the_left = tile_to_the_left_opt.unwrap();

            if (tile_above.tile_type == TileType::Vertical
                || tile_above.tile_type == TileType::BendPipe90SW
                || tile_above.tile_type == TileType::BendPipe90SE)
                && (tile_to_the_left.tile_type == TileType::Horizontal
                    || tile_to_the_left.tile_type == TileType::BendPipe90SE
                    || tile_to_the_left.tile_type == TileType::BendPipe90NE)
            {
                tile_type = TileType::BendPipe90NE;
            }
        }

        if tile_above_opt.is_some() && tile_to_the_right_opt.is_some() {
            let tile_above = tile_above_opt.unwrap();
            let tile_to_the_right = tile_to_the_right_opt.unwrap();

            if (tile_above.tile_type == TileType::Vertical
                || tile_above.tile_type == TileType::BendPipe90SW
                || tile_above.tile_type == TileType::BendPipe90SE)
                && (tile_to_the_right.tile_type == TileType::Horizontal
                    || tile_to_the_right.tile_type == TileType::BendPipe90SW
                    || tile_to_the_right.tile_type == TileType::BendPipe90NW)
            {
                tile_type = TileType::BendPipe90NW;
            }
        }

        if tile_below_opt.is_some() && tile_to_the_left_opt.is_some() {
            let tile_below = tile_below_opt.unwrap();
            let tile_to_the_left = tile_to_the_left_opt.unwrap();

            if (tile_below.tile_type == TileType::Vertical
                || tile_below.tile_type == TileType::BendPipe90NW
                || tile_below.tile_type == TileType::BendPipe90NE)
                && (tile_to_the_left.tile_type == TileType::Horizontal
                    || tile_to_the_left.tile_type == TileType::BendPipe90NE
                    || tile_to_the_left.tile_type == TileType::BendPipe90SE)
            {
                tile_type = TileType::BendPipe90SW;
            }
        }

        if tile_below_opt.is_some() && tile_to_the_right_opt.is_some() {
            let tile_below = tile_below_opt.unwrap();
            let tile_to_the_right = tile_to_the_right_opt.unwrap();

            if (tile_below.tile_type == TileType::Vertical
                || tile_below.tile_type == TileType::BendPipe90NW
                || tile_below.tile_type == TileType::BendPipe90NE)
                && (tile_to_the_right.tile_type == TileType::Horizontal
                    || tile_to_the_right.tile_type == TileType::BendPipe90NW
                    || tile_to_the_right.tile_type == TileType::BendPipe90SW)
            {
                tile_type = TileType::BendPipe90SE;
            }
        }

        tile_type
    }

    fn find_next_pipe_tile(
        tiles: &[Vec<Tile>],
        current_tile: &Tile,
        visited_tiles: &[Tile],
    ) -> Option<Tile> {
        let mut next_tile: Option<Tile> = None;

        if current_tile.tile_type == TileType::Vertical {
            if current_tile.row > 0 {
                if let Some(tile_row) = tiles.get(current_tile.row - 1) {
                    if let Some(tile_cell) = tile_row.get(current_tile.column) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Vertical
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90SW)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }
            }

            if let Some(tile_row) = tiles.get(current_tile.row + 1) {
                if let Some(tile_cell) = tile_row.get(current_tile.column) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Vertical
                            || tile_cell.tile_type == TileType::BendPipe90NE
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }
        }

        if current_tile.tile_type == TileType::Horizontal {
            if let Some(tile_row) = tiles.get(current_tile.row) {
                if current_tile.column > 0 {
                    if let Some(tile_cell) = tile_row.get(current_tile.column - 1) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Horizontal
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90NE)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }

                if let Some(tile_cell) = tile_row.get(current_tile.column + 1) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Horizontal
                            || tile_cell.tile_type == TileType::BendPipe90SW
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }
        }

        if current_tile.tile_type == TileType::BendPipe90NE {
            // L
            if current_tile.row > 0 {
                if let Some(tile_row) = tiles.get(current_tile.row - 1) {
                    if let Some(tile_cell) = tile_row.get(current_tile.column) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Vertical
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90SW)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }
            }

            if let Some(tile_row) = tiles.get(current_tile.row) {
                if let Some(tile_cell) = tile_row.get(current_tile.column + 1) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Horizontal
                            || tile_cell.tile_type == TileType::BendPipe90SW
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }
        }

        if current_tile.tile_type == TileType::BendPipe90NW {
            // J
            if current_tile.row > 0 {
                if let Some(tile_row) = tiles.get(current_tile.row - 1) {
                    if let Some(tile_cell) = tile_row.get(current_tile.column) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Vertical
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90SW)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }
            }

            if let Some(tile_row) = tiles.get(current_tile.row) {
                if current_tile.column > 0 {
                    if let Some(tile_cell) = tile_row.get(current_tile.column - 1) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Horizontal
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90NE)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }
            }
        }

        if current_tile.tile_type == TileType::BendPipe90SE {
            // F
            if let Some(tile_row) = tiles.get(current_tile.row + 1) {
                if let Some(tile_cell) = tile_row.get(current_tile.column) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Vertical
                            || tile_cell.tile_type == TileType::BendPipe90NE
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }

            if let Some(tile_row) = tiles.get(current_tile.row) {
                if let Some(tile_cell) = tile_row.get(current_tile.column + 1) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Horizontal
                            || tile_cell.tile_type == TileType::BendPipe90SW
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }
        }
        if current_tile.tile_type == TileType::BendPipe90SW {
            // 7
            if let Some(tile_row) = tiles.get(current_tile.row + 1) {
                if let Some(tile_cell) = tile_row.get(current_tile.column) {
                    if !visited_tiles.contains(tile_cell)
                        && (tile_cell.tile_type == TileType::Vertical
                            || tile_cell.tile_type == TileType::BendPipe90NE
                            || tile_cell.tile_type == TileType::BendPipe90NW)
                    {
                        next_tile = Some(*tile_cell);
                    }
                }
            }

            if let Some(tile_row) = tiles.get(current_tile.row) {
                if current_tile.column > 0 {
                    if let Some(tile_cell) = tile_row.get(current_tile.column - 1) {
                        if !visited_tiles.contains(tile_cell)
                            && (tile_cell.tile_type == TileType::Horizontal
                                || tile_cell.tile_type == TileType::BendPipe90SE
                                || tile_cell.tile_type == TileType::BendPipe90NE)
                        {
                            next_tile = Some(*tile_cell);
                        }
                    }
                }
            }
        }

        next_tile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_pipe_maze_max_distance_from_start(
        expected_max_distance_from_start: u64,
        maze_layout: &str,
    ) {
        assert_eq!(
            expected_max_distance_from_start,
            PipeMaze::parse(maze_layout).max_len_from_start()
        );
    }

    fn assert_pipe_maze_tiles_inside_pipe_loop_count(
        expected_tiles_inside_loop_count: u64,
        maze_layout: &str,
    ) {
        assert_eq!(
            expected_tiles_inside_loop_count,
            PipeMaze::parse(maze_layout).number_of_tiles_in_pipe()
        );
    }

    #[test]
    fn pipe_maze_only_tiles_that_make_the_pipe() {
        assert_pipe_maze_max_distance_from_start(4, ".....\n.S-7.\n.|.|.\n.L-J.\n.....");
        assert_pipe_maze_max_distance_from_start(8, "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
    }

    #[test]
    fn pipe_maze_pipe_tiles_and_surrounding_pipes() {
        assert_pipe_maze_max_distance_from_start(4, "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF\n");
        assert_pipe_maze_max_distance_from_start(8, "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ\n");
    }

    #[test]
    fn pipe_maze_tile_count_only_tiles_that_make_the_pipe() {
        assert_pipe_maze_tiles_inside_pipe_loop_count(1, ".....\n.S-7.\n.|.|.\n.L-J.\n.....");
        assert_pipe_maze_tiles_inside_pipe_loop_count(1, "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
        assert_pipe_maze_tiles_inside_pipe_loop_count(4, "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........");
    }

    #[test]
    fn pipe_maze_tile_count_with_random_tiles() {
        assert_pipe_maze_tiles_inside_pipe_loop_count(1, "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF");
        assert_pipe_maze_tiles_inside_pipe_loop_count(1, "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ\n");
        assert_pipe_maze_tiles_inside_pipe_loop_count(8, ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...");
        assert_pipe_maze_tiles_inside_pipe_loop_count(10, "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L");
    }
}
