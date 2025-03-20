use rand::Rng;

#[derive(Debug, Clone)]
pub struct Team {
    name: String,
    seed: u32,
    strength: f64,
}

impl Team {
    pub fn new(name: String, seed: u32, strength: f64) -> Team {
        Team {
            name,
            seed,
            strength,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game<'a> {
    team_0: &'a Team,
    team_1: &'a Team,
    round: u32,
}

impl<'a> Game<'a> {
    // Constructor method to create a new Game instance
    fn new(team_0: &'a Team, team_1: &'a Team, round: u32) -> Game<'a> {
        Game {
            team_0,
            team_1,
            round,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameResult<'a> {
    game: Game<'a>,
    winner: &'a Team,
    points: u32,
}

impl<'a> GameResult<'a> {
    fn new(game: Game<'a>, winner: &'a Team, points: u32) -> GameResult<'a> {
        GameResult {
            game,
            winner,
            points,
        }
    }
}

impl<'a> GameResult<'a> {
    fn sample_from_game(game: Game<'a>) -> GameResult {
        let mut rng = rand::rng();

        let upper_bound = game.team_0.strength + game.team_1.strength;
        let random_float: f64 = rng.random_range(0.0..upper_bound);

        let round_points = 2_u32.pow(game.round as u32);
        let mut upset_points = 0;

        let winner;
        let loser;
        if random_float < game.team_0.strength {
            // team 0 wins
            winner = game.team_0;
            loser = game.team_1;
        } else {
            winner = game.team_1;
            loser = game.team_0;
        };

        if loser.seed > winner.seed {
            upset_points = loser.seed - winner.seed;
        }

        let points = round_points + upset_points;

        Self::new(game, winner, points)
    }
}

#[derive(Debug, Clone)]
pub struct Region<'a> {
    name: String,
    seed_1: &'a Team,
    seed_2: &'a Team,
    seed_3: &'a Team,
    seed_4: &'a Team,
    seed_5: &'a Team,
    seed_6: &'a Team,
    seed_7: &'a Team,
    seed_8: &'a Team,
    seed_9: &'a Team,
    seed_10: &'a Team,
    seed_11: &'a Team,
    seed_12: &'a Team,
    seed_13: &'a Team,
    seed_14: &'a Team,
    seed_15: &'a Team,
    seed_16: &'a Team,
}

impl<'a> Region<'a> {
    pub fn new(
        name: String,
        seed_1: &'a Team,
        seed_2: &'a Team,
        seed_3: &'a Team,
        seed_4: &'a Team,
        seed_5: &'a Team,
        seed_6: &'a Team,
        seed_7: &'a Team,
        seed_8: &'a Team,
        seed_9: &'a Team,
        seed_10: &'a Team,
        seed_11: &'a Team,
        seed_12: &'a Team,
        seed_13: &'a Team,
        seed_14: &'a Team,
        seed_15: &'a Team,
        seed_16: &'a Team,
    ) -> Region<'a> {
        Region {
            name,
            seed_1,
            seed_2,
            seed_3,
            seed_4,
            seed_5,
            seed_6,
            seed_7,
            seed_8,
            seed_9,
            seed_10,
            seed_11,
            seed_12,
            seed_13,
            seed_14,
            seed_15,
            seed_16,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegionResult<'a> {
    //
    // round 1
    game_1_16: GameResult<'a>,
    game_8_9: GameResult<'a>,
    game_5_12: GameResult<'a>,
    game_4_13: GameResult<'a>,
    game_6_11: GameResult<'a>,
    game_3_14: GameResult<'a>,
    game_7_10: GameResult<'a>,
    game_2_15: GameResult<'a>,
    //
    // round 2
    game_1_9: GameResult<'a>,
    game_5_13: GameResult<'a>,
    game_6_14: GameResult<'a>,
    game_7_15: GameResult<'a>,
    //
    // round 3
    game_1_13: GameResult<'a>,
    game_6_15: GameResult<'a>,
    //
    //round 4
    game_1_15: GameResult<'a>,
    //
    winner: &'a Team,
    points: u32,
}

impl<'a> RegionResult<'a> {
    fn new(
        game_1_16_result: GameResult<'a>,
        game_8_9_result: GameResult<'a>,
        game_5_12_result: GameResult<'a>,
        game_4_13_result: GameResult<'a>,
        game_6_11_result: GameResult<'a>,
        game_3_14_result: GameResult<'a>,
        game_7_10_result: GameResult<'a>,
        game_2_15_result: GameResult<'a>,
        game_1_9_result: GameResult<'a>,
        game_5_13_result: GameResult<'a>,
        game_6_14_result: GameResult<'a>,
        game_7_15_result: GameResult<'a>,
        game_1_13_result: GameResult<'a>,
        game_6_15_result: GameResult<'a>,
        game_1_15_result: GameResult<'a>,
        winner: &'a Team,
        points: u32,
    ) -> RegionResult<'a> {
        RegionResult {
            game_1_16: game_1_16_result,
            game_8_9: game_8_9_result,
            game_5_12: game_5_12_result,
            game_4_13: game_4_13_result,
            game_6_11: game_6_11_result,
            game_3_14: game_3_14_result,
            game_7_10: game_7_10_result,
            game_2_15: game_2_15_result,
            game_1_9: game_1_9_result,
            game_5_13: game_5_13_result,
            game_6_14: game_6_14_result,
            game_7_15: game_7_15_result,
            game_1_13: game_1_13_result,
            game_6_15: game_6_15_result,
            game_1_15: game_1_15_result,
            winner,
            points,
        }
    }
}

impl<'a> Region<'a> {
    fn sample_result(&self) -> RegionResult<'a> {
        // Round 0:
        let game_1_16 = Game::new(self.seed_1, self.seed_16, 0);
        let game_8_9 = Game::new(self.seed_8, self.seed_9, 0);
        let game_5_12 = Game::new(self.seed_5, self.seed_12, 0);
        let game_4_13 = Game::new(self.seed_4, self.seed_13, 0);
        let game_6_11 = Game::new(self.seed_6, self.seed_11, 0);
        let game_3_14 = Game::new(self.seed_3, self.seed_14, 0);
        let game_7_10 = Game::new(self.seed_7, self.seed_10, 0);
        let game_2_15 = Game::new(self.seed_2, self.seed_15, 0);
        //
        let game_1_16_result = GameResult::sample_from_game(game_1_16);
        let game_8_9_result = GameResult::sample_from_game(game_8_9);
        let game_5_12_result = GameResult::sample_from_game(game_5_12);
        let game_4_13_result = GameResult::sample_from_game(game_4_13);
        let game_6_11_result = GameResult::sample_from_game(game_6_11);
        let game_3_14_result = GameResult::sample_from_game(game_3_14);
        let game_7_10_result = GameResult::sample_from_game(game_7_10);
        let game_2_15_result = GameResult::sample_from_game(game_2_15);

        // Round 1:
        let game_1_9 = Game::new(game_1_16_result.winner, game_8_9_result.winner, 1);
        let game_5_13 = Game::new(game_5_12_result.winner, game_4_13_result.winner, 1);
        let game_6_14 = Game::new(game_6_11_result.winner, game_3_14_result.winner, 1);
        let game_7_15 = Game::new(game_7_10_result.winner, game_2_15_result.winner, 1);
        //
        let game_1_9_result = GameResult::sample_from_game(game_1_9);
        let game_5_13_result = GameResult::sample_from_game(game_5_13);
        let game_6_14_result = GameResult::sample_from_game(game_6_14);
        let game_7_15_result = GameResult::sample_from_game(game_7_15);

        // Round 2:
        let game_1_13 = Game::new(game_1_9_result.winner, game_5_13_result.winner, 2);
        let game_6_15 = Game::new(game_6_14_result.winner, game_7_15_result.winner, 2);
        //
        let game_1_13_result = GameResult::sample_from_game(game_1_13);
        let game_6_15_result = GameResult::sample_from_game(game_6_15);

        // Round 3:
        let game_1_15 = Game::new(game_1_13_result.winner, game_6_15_result.winner, 3);
        //
        let game_1_15_result = GameResult::sample_from_game(game_1_15);

        let winner = game_1_15_result.winner;

        // Round 1
        let points = game_1_16_result.points
            + game_8_9_result.points
            + game_5_12_result.points
            + game_4_13_result.points
            + game_6_11_result.points
            + game_3_14_result.points
            + game_7_10_result.points
            + game_2_15_result.points

        // Round 2
            + game_1_9_result.points
            + game_5_13_result.points
            + game_6_14_result.points
            + game_7_15_result.points

        // Round 3
            + game_1_13_result.points
            + game_6_15_result.points

        // Round 4
            + game_1_15_result.points;

        let region_result = RegionResult::new(
            game_1_16_result,
            game_8_9_result,
            game_5_12_result,
            game_4_13_result,
            game_6_11_result,
            game_3_14_result,
            game_7_10_result,
            game_2_15_result,
            //
            game_1_9_result,
            game_5_13_result,
            game_6_14_result,
            game_7_15_result,
            //
            game_1_13_result,
            game_6_15_result,
            //
            game_1_15_result,
            //
            winner,
            points,
        );

        region_result
    }
}

#[derive(Debug, Clone)]
pub struct Bracket<'a> {
    south: Region<'a>,
    east: Region<'a>,
    west: Region<'a>,
    midwest: Region<'a>,
}

impl<'a> Bracket<'a> {
    pub fn new(
        south: Region<'a>,
        east: Region<'a>,
        west: Region<'a>,
        midwest: Region<'a>,
    ) -> Bracket<'a> {
        Bracket {
            south,
            east,
            west,
            midwest,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BracketResult<'a> {
    south_result: RegionResult<'a>,
    east_result: RegionResult<'a>,
    west_result: RegionResult<'a>,
    midwest_result: RegionResult<'a>,
    south_west_result: GameResult<'a>,
    east_midwest_result: GameResult<'a>,
    championship_result: GameResult<'a>,
    points: u32,
}

impl<'a> BracketResult<'a> {
    fn new(
        south_result: RegionResult<'a>,
        east_result: RegionResult<'a>,
        west_result: RegionResult<'a>,
        midwest_result: RegionResult<'a>,
        south_west_result: GameResult<'a>,
        east_midwest_result: GameResult<'a>,
        championship_result: GameResult<'a>,
        points: u32,
    ) -> BracketResult<'a> {
        BracketResult {
            south_result,
            east_result,
            west_result,
            midwest_result,
            south_west_result,
            east_midwest_result,
            championship_result,
            points,
        }
    }
}

impl<'a> Bracket<'a> {
    pub fn sample_result(&self) -> BracketResult<'a> {
        let south_result: RegionResult<'a> = self.south.sample_result(); // LIFETIME MAY NOT LIVE LONG ENOUGH
        let west_result: RegionResult<'a> = self.west.sample_result();
        let east_result: RegionResult<'a> = self.east.sample_result();
        let midwest_result: RegionResult<'a> = self.midwest.sample_result();

        // Round 4:
        let south_west_game: Game<'a> = Game::new(south_result.winner, west_result.winner, 4);
        let east_midwest_game: Game<'a> = Game::new(east_result.winner, midwest_result.winner, 4);
        //
        let south_west_result = GameResult::sample_from_game(south_west_game);
        let east_midwest_result = GameResult::sample_from_game(east_midwest_game);

        // Round 5 (Championship)
        let championsip_game = Game::new(south_west_result.winner, east_midwest_result.winner, 5);
        //
        let championship_result = GameResult::sample_from_game(championsip_game);

        let points = south_result.points
            + west_result.points
            + east_result.points
            + midwest_result.points
            + south_west_result.points
            + east_midwest_result.points
            + championship_result.points;

        BracketResult::new(
            south_result,
            east_result,
            west_result,
            midwest_result,
            south_west_result,
            east_midwest_result,
            championship_result,
            points,
        )
    }
}

pub fn generate_random_u64() -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}

impl<'a> BracketResult<'a> {
    pub fn write_to_csv(&self, file_path: &str) -> std::io::Result<()> {
        use std::fs::{File, OpenOptions};
        use std::io::{BufWriter, Write};
        use std::path::Path;

        let path = Path::new(file_path);
        let file_exists = path.exists();

        let file = if file_exists {
            OpenOptions::new().append(true).open(file_path)?
        } else {
            File::create(file_path)?
        };

        let mut writer = BufWriter::new(file);

        // Write header if the file is new
        if !file_exists {
            writeln!(
                writer,
                "sim_id, bracket_hash, south_1_16,south_8_9,south_5_12,south_4_13,south_6_11,south_3_14,south_7_10,south_2_15,\
                 south_1_9,south_5_13,south_6_14,south_7_15,\
                 south_1_13,south_6_15,\
                 south_winner,\
                 east_1_16,east_8_9,east_5_12,east_4_13,east_6_11,east_3_14,east_7_10,east_2_15,\
                 east_1_9,east_5_13,east_6_14,east_7_15,\
                 east_1_13,east_6_15,\
                 east_winner,\
                 west_1_16,west_8_9,west_5_12,west_4_13,west_6_11,west_3_14,west_7_10,west_2_15,\
                 west_1_9,west_5_13,west_6_14,west_7_15,\
                 west_1_13,west_6_15,\
                 west_winner,\
                 midwest_1_16,midwest_8_9,midwest_5_12,midwest_4_13,midwest_6_11,midwest_3_14,midwest_7_10,midwest_2_15,\
                 midwest_1_9,midwest_5_13,midwest_6_14,midwest_7_15,\
                 midwest_1_13,midwest_6_15,\
                 midwest_winner,\
                 south_west_winner,east_midwest_winner,champion,total_points"
            )?;
        }

        // Write data row
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\
             {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\
             {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\
             {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\
             {},{},{},{}",
            generate_random_u64(),
            self.get_hash(),
            // South region round 1
            self.south_result.game_1_16.winner.name,
            self.south_result.game_8_9.winner.name,
            self.south_result.game_5_12.winner.name,
            self.south_result.game_4_13.winner.name,
            self.south_result.game_6_11.winner.name,
            self.south_result.game_3_14.winner.name,
            self.south_result.game_7_10.winner.name,
            self.south_result.game_2_15.winner.name,
            // South region round 2
            self.south_result.game_1_9.winner.name,
            self.south_result.game_5_13.winner.name,
            self.south_result.game_6_14.winner.name,
            self.south_result.game_7_15.winner.name,
            // South region round 3
            self.south_result.game_1_13.winner.name,
            self.south_result.game_6_15.winner.name,
            // South region winner
            self.south_result.winner.name,
            // East region round 1
            self.east_result.game_1_16.winner.name,
            self.east_result.game_8_9.winner.name,
            self.east_result.game_5_12.winner.name,
            self.east_result.game_4_13.winner.name,
            self.east_result.game_6_11.winner.name,
            self.east_result.game_3_14.winner.name,
            self.east_result.game_7_10.winner.name,
            self.east_result.game_2_15.winner.name,
            // East region round 2
            self.east_result.game_1_9.winner.name,
            self.east_result.game_5_13.winner.name,
            self.east_result.game_6_14.winner.name,
            self.east_result.game_7_15.winner.name,
            // East region round 3
            self.east_result.game_1_13.winner.name,
            self.east_result.game_6_15.winner.name,
            // East region winner
            self.east_result.winner.name,
            // West region round 1
            self.west_result.game_1_16.winner.name,
            self.west_result.game_8_9.winner.name,
            self.west_result.game_5_12.winner.name,
            self.west_result.game_4_13.winner.name,
            self.west_result.game_6_11.winner.name,
            self.west_result.game_3_14.winner.name,
            self.west_result.game_7_10.winner.name,
            self.west_result.game_2_15.winner.name,
            // West region round 2
            self.west_result.game_1_9.winner.name,
            self.west_result.game_5_13.winner.name,
            self.west_result.game_6_14.winner.name,
            self.west_result.game_7_15.winner.name,
            // West region round 3
            self.west_result.game_1_13.winner.name,
            self.west_result.game_6_15.winner.name,
            // West region winner
            self.west_result.winner.name,
            // Midwest region round 1
            self.midwest_result.game_1_16.winner.name,
            self.midwest_result.game_8_9.winner.name,
            self.midwest_result.game_5_12.winner.name,
            self.midwest_result.game_4_13.winner.name,
            self.midwest_result.game_6_11.winner.name,
            self.midwest_result.game_3_14.winner.name,
            self.midwest_result.game_7_10.winner.name,
            self.midwest_result.game_2_15.winner.name,
            // Midwest region round 2
            self.midwest_result.game_1_9.winner.name,
            self.midwest_result.game_5_13.winner.name,
            self.midwest_result.game_6_14.winner.name,
            self.midwest_result.game_7_15.winner.name,
            // Midwest region round 3
            self.midwest_result.game_1_13.winner.name,
            self.midwest_result.game_6_15.winner.name,
            // Midwest region winner
            self.midwest_result.winner.name,
            // Final Four
            self.south_west_result.winner.name,
            self.east_midwest_result.winner.name,
            // Champion
            self.championship_result.winner.name,
            // Total points
            self.points
        )?;

        writer.flush()?;
        Ok(())
    }
}

use std::hash::{Hash, Hasher};

// Add this derive to BracketResult
// #[derive(Debug, Clone)]
// pub struct BracketResult<'a> { ... }

impl<'a> Hash for BracketResult<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash South region results
        self.south_result.game_1_16.winner.name.hash(state);
        self.south_result.game_8_9.winner.name.hash(state);
        self.south_result.game_5_12.winner.name.hash(state);
        self.south_result.game_4_13.winner.name.hash(state);
        self.south_result.game_6_11.winner.name.hash(state);
        self.south_result.game_3_14.winner.name.hash(state);
        self.south_result.game_7_10.winner.name.hash(state);
        self.south_result.game_2_15.winner.name.hash(state);

        self.south_result.game_1_9.winner.name.hash(state);
        self.south_result.game_5_13.winner.name.hash(state);
        self.south_result.game_6_14.winner.name.hash(state);
        self.south_result.game_7_15.winner.name.hash(state);

        self.south_result.game_1_13.winner.name.hash(state);
        self.south_result.game_6_15.winner.name.hash(state);

        self.south_result.game_1_15.winner.name.hash(state);

        // Hash East region results
        self.east_result.game_1_16.winner.name.hash(state);
        self.east_result.game_8_9.winner.name.hash(state);
        self.east_result.game_5_12.winner.name.hash(state);
        self.east_result.game_4_13.winner.name.hash(state);
        self.east_result.game_6_11.winner.name.hash(state);
        self.east_result.game_3_14.winner.name.hash(state);
        self.east_result.game_7_10.winner.name.hash(state);
        self.east_result.game_2_15.winner.name.hash(state);

        self.east_result.game_1_9.winner.name.hash(state);
        self.east_result.game_5_13.winner.name.hash(state);
        self.east_result.game_6_14.winner.name.hash(state);
        self.east_result.game_7_15.winner.name.hash(state);

        self.east_result.game_1_13.winner.name.hash(state);
        self.east_result.game_6_15.winner.name.hash(state);

        self.east_result.game_1_15.winner.name.hash(state);

        // Hash West region results
        self.west_result.game_1_16.winner.name.hash(state);
        self.west_result.game_8_9.winner.name.hash(state);
        self.west_result.game_5_12.winner.name.hash(state);
        self.west_result.game_4_13.winner.name.hash(state);
        self.west_result.game_6_11.winner.name.hash(state);
        self.west_result.game_3_14.winner.name.hash(state);
        self.west_result.game_7_10.winner.name.hash(state);
        self.west_result.game_2_15.winner.name.hash(state);

        self.west_result.game_1_9.winner.name.hash(state);
        self.west_result.game_5_13.winner.name.hash(state);
        self.west_result.game_6_14.winner.name.hash(state);
        self.west_result.game_7_15.winner.name.hash(state);

        self.west_result.game_1_13.winner.name.hash(state);
        self.west_result.game_6_15.winner.name.hash(state);

        self.west_result.game_1_15.winner.name.hash(state);

        // Hash Midwest region results
        self.midwest_result.game_1_16.winner.name.hash(state);
        self.midwest_result.game_8_9.winner.name.hash(state);
        self.midwest_result.game_5_12.winner.name.hash(state);
        self.midwest_result.game_4_13.winner.name.hash(state);
        self.midwest_result.game_6_11.winner.name.hash(state);
        self.midwest_result.game_3_14.winner.name.hash(state);
        self.midwest_result.game_7_10.winner.name.hash(state);
        self.midwest_result.game_2_15.winner.name.hash(state);

        self.midwest_result.game_1_9.winner.name.hash(state);
        self.midwest_result.game_5_13.winner.name.hash(state);
        self.midwest_result.game_6_14.winner.name.hash(state);
        self.midwest_result.game_7_15.winner.name.hash(state);

        self.midwest_result.game_1_13.winner.name.hash(state);
        self.midwest_result.game_6_15.winner.name.hash(state);

        self.midwest_result.game_1_15.winner.name.hash(state);

        // Hash Final Four results
        self.south_west_result.winner.name.hash(state);
        self.east_midwest_result.winner.name.hash(state);

        // Hash Championship result
        self.championship_result.winner.name.hash(state);
    }
}

// Add this function to BracketResult to get a hash value
impl<'a> BracketResult<'a> {
    pub fn get_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    // Optional: Method to compare if two brackets have the same structure
    pub fn has_same_structure(&self, other: &BracketResult) -> bool {
        self.get_hash() == other.get_hash()
    }
}

// Add Eq trait implementation
impl<'a> PartialEq for BracketResult<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_hash() == other.get_hash()
    }
}

impl<'a> Eq for BracketResult<'a> {}
