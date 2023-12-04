use scanf::sscanf;

pub struct Game {
    number_of_red_balls: u64,
    number_of_green_balls: u64,
    number_of_blue_balls: u64,
}

impl Game {
    pub fn new(available_balls: &str) -> Self {
        let (number_of_red_balls, number_of_green_balls, number_of_blue_balls) =
            Game::parse_ball_number(&available_balls.to_lowercase());

        Game {
            number_of_red_balls,
            number_of_green_balls,
            number_of_blue_balls,
        }
    }

    pub fn possible_games_id_sum(&self, games: &str) -> u64 {
        let mut game_id_sum: u64 = 0;

        for game in games.to_lowercase().split('\n') {
            let (game_id, pulled_balls_sets) = match Game::split_game_id_and_data(game) {
                Ok((id, balls_sets)) => (id, balls_sets),
                Err(_) => continue,
            };

            if self.is_game_possible(pulled_balls_sets) {
                game_id_sum += game_id;
            }
        }
        game_id_sum
    }

    pub fn game_power_sum(&self, games: &str) -> u64 {
        let mut pow_sum: u64 = 0;

        for game in games.to_lowercase().split('\n') {
            let (_, pulled_balls_sets) = match Game::split_game_id_and_data(game) {
                Ok((game_id, balls_sets)) => (game_id, balls_sets),
                Err(_) => continue,
            };

            pow_sum += Game::minimum_cube_set_power(pulled_balls_sets.trim());
        }

        pow_sum
    }

    fn is_game_possible(&self, ball_sets: &str) -> bool {
        let mut is_game_possible = true;
        for ball_set in ball_sets.split(';') {
            let (pulled_red_balls, pulled_green_balls, pulled_blue_balls) =
                Game::parse_ball_number(ball_set);

            if pulled_red_balls > self.number_of_red_balls
                || pulled_green_balls > self.number_of_green_balls
                || pulled_blue_balls > self.number_of_blue_balls
            {
                is_game_possible = false;
                break;
            }
        }

        is_game_possible
    }

    fn split_game_id_and_data(game_string: &str) -> Result<(u64, &str), ()> {
        let mut game_id: u64 = 0;

        let colon_index = match game_string.find(':') {
            Some(index) => index,
            None => return Err(()),
        };

        if game_string.len() <= colon_index + 1 {
            return Err(());
        }

        let (game_id_string, pulled_balls_sets) = game_string.split_at(colon_index + 1);
        _ = sscanf!(game_id_string, "game {}:", game_id);

        Ok((game_id, pulled_balls_sets))
    }

    fn minimum_cube_set_power(ball_sets: &str) -> u64 {
        let mut fewest_red_balls_in_bag: u64 = std::u64::MIN;
        let mut fewest_green_balls_in_bag: u64 = std::u64::MIN;
        let mut fewest_blue_balls_in_bag: u64 = std::u64::MIN;
        for ball_set in ball_sets.split(';') {
            let (pulled_red_balls, pulled_green_balls, pulled_blue_balls) =
                Game::parse_ball_number(ball_set);

            if fewest_red_balls_in_bag < pulled_red_balls {
                fewest_red_balls_in_bag = pulled_red_balls;
            }

            if fewest_green_balls_in_bag < pulled_green_balls {
                fewest_green_balls_in_bag = pulled_green_balls;
            }

            if fewest_blue_balls_in_bag < pulled_blue_balls {
                fewest_blue_balls_in_bag = pulled_blue_balls;
            }
        }

        fewest_red_balls_in_bag * fewest_green_balls_in_bag * fewest_blue_balls_in_bag
    }

    fn parse_ball_number(ball_number: &str) -> (u64, u64, u64) {
        let mut red_balls: u64 = 0;
        let mut green_balls: u64 = 0;
        let mut blue_balls: u64 = 0;

        for balls in ball_number.split(',') {
            if balls.contains("red") {
                _ = sscanf!(balls, "{} red", red_balls);
            }

            if balls.contains("green") {
                _ = sscanf!(balls, "{} green", green_balls);
            }

            if balls.contains("blue") {
                _ = sscanf!(balls, "{} blue", blue_balls);
            }
        }

        (red_balls, green_balls, blue_balls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AVAILABLE_BALLS: &str = "12 red, 13 green, 14 blue";

    fn assert_cube_games_id_sum(possible_games_id_sum: u64, games_string: &str) {
        assert_eq!(
            possible_games_id_sum,
            Game::new(AVAILABLE_BALLS).possible_games_id_sum(games_string)
        );
    }

    fn assert_cube_games_power_sum(possible_games_pow_sum: u64, games_string: &str) {
        assert_eq!(
            possible_games_pow_sum,
            Game::new(AVAILABLE_BALLS).game_power_sum(games_string)
        );
    }

    #[test]
    fn possible_game_only_red() {
        assert_cube_games_id_sum(1, "Game 1: 0 red");
        assert_cube_games_id_sum(2, "Game 2: 5 red; 3 red");
        assert_cube_games_id_sum(3, "Game 3: 11 red; 10 red, 9 red");
    }

    #[test]
    fn possible_game_only_green() {
        assert_cube_games_id_sum(1, "Game 1: 0 green");
        assert_cube_games_id_sum(2, "Game 2: 5 green; 7 green");
        assert_cube_games_id_sum(3, "Game 3: 12 green; 11 green; 10 green");
    }

    #[test]
    fn possible_game_only_blue() {
        assert_cube_games_id_sum(1, "Game 1: 0 blue");
        assert_cube_games_id_sum(2, "Game 2: 7 blue; 9 blue");
        assert_cube_games_id_sum(3, "Game 3: 14 blue; 13 blue; 12 blue");
    }

    #[test]
    fn possible_game_only_read_and_green() {
        assert_cube_games_id_sum(1, "Game 1: 0 red, 0 green");
        assert_cube_games_id_sum(2, "Game 2: 3 red, 5 green; 9 red");
        assert_cube_games_id_sum(2, "Game 2: 3 red, 5 green; 6 green");
        assert_cube_games_id_sum(3, "Game 3: 12 red, 13 green; 11 red, 12 green");
    }

    #[test]
    fn possible_game_only_read_and_blue() {
        assert_cube_games_id_sum(1, "Game 1: 0 red, 0 blue");
        assert_cube_games_id_sum(2, "Game 2: 3 red, 5 blue; 1 red");
        assert_cube_games_id_sum(2, "Game 2: 3 red, 5 blue; 10 blue");
        assert_cube_games_id_sum(3, "Game 3: 12 red, 14 blue; 11 red, 13 blue");
    }

    #[test]
    fn possible_game_only_green_and_blue() {
        assert_cube_games_id_sum(1, "Game 1: 0 green, 0 blue");
        assert_cube_games_id_sum(2, "Game 2: 3 green, 5 blue; 5 green");
        assert_cube_games_id_sum(2, "Game 2: 3 green, 5 blue; 8 blue");
        assert_cube_games_id_sum(3, "Game 3: 13 green, 14 blue; 13 blue, 12 green");
    }

    #[test]
    fn possible_game_red_green_and_blue() {
        assert_cube_games_id_sum(1, "Game 1: 0 red, 0 green, 0 blue");
        assert_cube_games_id_sum(2, "Game 2: 7 red, 3 green, 5 blue; 6 red");
        assert_cube_games_id_sum(2, "Game 2: 7 red, 3 green, 5 blue; 6 red, 2 green");
        assert_cube_games_id_sum(2, "Game 2: 7 red, 3 green, 5 blue; 6 red, 2 green, 9 blue");
        assert_cube_games_id_sum(3, "Game 3: 12 red, 13 green, 14 blue");
    }

    #[test]
    fn possbile_games_only_red() {
        assert_cube_games_id_sum(3, "Game 1: 11 red\nGame 2: 3 red");
        assert_cube_games_id_sum(10, "Game 1: 11 red\nGame 2: 3 red\nGame 7: 13");
        assert_cube_games_id_sum(6, "Game 4: 11 red; 9 red; 3 red\nGame 2: 3 red, 10 red");
    }

    #[test]
    fn possbile_games_only_blue() {
        assert_cube_games_id_sum(3, "Game 1: 11 blue\nGame 2: 3 blue");
        assert_cube_games_id_sum(10, "Game 1: 11 blue\nGame 2: 3 blue\nGame 7: 13");
        assert_cube_games_id_sum(
            6,
            "Game 4: 11 blue; 9 blue; 3 blue\nGame 2: 3 blue, 10 blue",
        );
    }

    #[test]
    fn possbile_games_only_green() {
        assert_cube_games_id_sum(3, "Game 1: 11 green\nGame 2: 3 green");
        assert_cube_games_id_sum(10, "Game 1: 11 green\nGame 2: 3 green\nGame 7: 13");
        assert_cube_games_id_sum(
            6,
            "Game 4: 11 green; 9 green; 3 green\nGame 2: 3 green, 10 green",
        );
    }

    #[test]
    fn impossible_game_only_red() {
        assert_cube_games_id_sum(0, "Game 1: 13 red");
        assert_cube_games_id_sum(0, "Game 2: 14 red; 1 red");
        assert_cube_games_id_sum(0, "Game 3: 20 red; 21 red");
    }

    #[test]
    fn impossible_game_only_green() {
        assert_cube_games_id_sum(0, "Game 1: 14 green");
        assert_cube_games_id_sum(0, "Game 2: 34 green; 5 green");
        assert_cube_games_id_sum(0, "Game 3: 74 green; 63 green");
    }

    #[test]
    fn impossible_game_only_blue() {
        assert_cube_games_id_sum(0, "Game 1: 15 blue");
        assert_cube_games_id_sum(0, "Game 2: 42 blue; 3 blue");
        assert_cube_games_id_sum(0, "Game 3: 71 blue; 88 blue");
    }

    #[test]
    fn impossible_game_only_read_and_green() {
        assert_cube_games_id_sum(0, "Game 1: 13 red, 14 green");
        assert_cube_games_id_sum(0, "Game 2: 3 red, 14 green; 10 red");
        assert_cube_games_id_sum(0, "Game 2: 3 red, 14 green; 9 green");
        assert_cube_games_id_sum(0, "Game 3: 13 red, 1 green");
        assert_cube_games_id_sum(0, "Game 3: 13 red, 1 green; 12 red, 69 green");
    }

    #[test]
    fn impossible_game_only_read_and_blue() {
        assert_cube_games_id_sum(0, "Game 1: 13 red, 15 blue");
        assert_cube_games_id_sum(0, "Game 2: 3 red, 15 blue; 1 red");
        assert_cube_games_id_sum(0, "Game 2: 3 red, 15 blue; 8 blue");
        assert_cube_games_id_sum(0, "Game 3: 14 red, 9 blue");
        assert_cube_games_id_sum(0, "Game 3: 14 red, 9 blue; 8 red, 19 blue");
    }

    #[test]
    fn impossible_game_only_green_and_blue() {
        assert_cube_games_id_sum(0, "Game 1: 14 green, 15 blue");
        assert_cube_games_id_sum(0, "Game 2: 3 green, 15 blue");
        assert_cube_games_id_sum(0, "Game 2: 3 green, 15 blue; 9 green");
        assert_cube_games_id_sum(0, "Game 2: 3 green, 15 blue; 3 blue");
        assert_cube_games_id_sum(0, "Game 3: 14 green, 4 blue");
        assert_cube_games_id_sum(0, "Game 3: 14 green, 4 blue; 7 blue, 13 green");
    }

    #[test]
    fn impossible_game_red_green_and_blue() {
        assert_cube_games_id_sum(0, "Game 1: 13 red, 14 green, 15 blue");
        assert_cube_games_id_sum(0, "Game 1: 0 red, 14 green, 15 blue");
        assert_cube_games_id_sum(0, "Game 1: 0 red, 1 green, 15 blue");
        assert_cube_games_id_sum(0, "Game 1: 0 red, 14 green, 2 blue");
        assert_cube_games_id_sum(0, "Game 1: 13 red, 14 green, 2 blue");
        assert_cube_games_id_sum(0, "Game 1: 13 red, 1 green, 2 blue");
        assert_cube_games_id_sum(0, "Game 1: 3 red, 1 green, 2 blue; 19 green");
        assert_cube_games_id_sum(0, "Game 1: 3 red, 1 green, 2 blue; 19 blue");
        assert_cube_games_id_sum(0, "Game 1: 3 red, 1 green, 2 blue; 19 red");
        assert_cube_games_id_sum(
            0,
            "Game 1: 3 red, 1 green, 2 blue; 19 red, 47 blue, 55 green",
        );
    }

    #[test]
    fn fewest_ball_power_sum() {
        assert_cube_games_power_sum(48, "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_cube_games_power_sum(
            12,
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        );
        assert_cube_games_power_sum(
            1560,
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_cube_games_power_sum(
            630,
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );
        assert_cube_games_power_sum(36, "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_cube_games_power_sum(
            2286,
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
    }
}
