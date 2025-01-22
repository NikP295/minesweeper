use eframe::egui;



fn main() {
    let window_options = eframe::NativeOptions::default();
    let x = 8;
    let mut game_instance = SweeperOfMines::begin(x);
    game_instance.adjacent_changes((0,0), false);
    eframe::run_native(
        "Nikov Minesweeper",
        window_options,
        Box::new(|_cc| Ok(Box::new(game_instance))),
    );
}

struct SweeperOfMines{
    bomb_num:i32,
    grid_size:i32,
    bomb_locations:Vec<(usize, usize)>,
    grid:Vec<Vec<i32>>,    //expected data is: 0-8 for nearby bombs, 10 for actual bomb, 120-130 for flagged positions, 100-110 for shown revealed positions
    playing_status:bool,
    bombs_to_cover: i32,
    victory_status:bool,
}

trait Begin {
    fn begin(size: i32) -> Self;
}

impl Begin for SweeperOfMines {
    fn begin(size: i32) -> Self {
        let bomb_num = match size {
            8 => 10,
            16 => 40,
            32 => 99,
            _ => 10
        };

        let grid = vec![vec![0; size as usize]; size as usize];
        let bomb_locations = bomb_locations(bomb_num, size);

        SweeperOfMines {
            grid_size: size,
            bomb_num,
            grid,
            bomb_locations,
            playing_status: true,
            victory_status: false,
            bombs_to_cover: bomb_num
        }
    }
}

impl SweeperOfMines {
    fn restart(&mut self, size: i32) {
        self.bomb_num = match size {
            8 => 10,
            16 => 40,
            32 => 99,
            _ => 10
        };

        self.grid_size = size;
        self.grid = vec![vec![0; size as usize]; size as usize];
        self.bomb_locations = bomb_locations(self.bomb_num, size);
        self.bombs_to_cover = self.bomb_num;
        self.playing_status = true;
        self.victory_status = false;
        self.adjacent_changes((0,0), false);
    }
}


impl SweeperOfMines {
    fn what_to_display(&mut self, coords:(usize, usize)) -> (char, (u8, u8, u8)) {
        let mut value_on_square:i32 = self.grid[coords.0][coords.1];
        let mut display_char:bool = false;
        if value_on_square >= 100 {
            value_on_square -= 100;
            if value_on_square < 20 {
                display_char = true;
            }
        }
        let mut character:char;
        

        if display_char {
            if value_on_square <= 8 {
                character = (value_on_square as u8 + b'0') as char;
            }
            else if value_on_square == 10 {
                character = 'B';
            }
            else {
                character = 'A';
            }
        }

        else if value_on_square >= 20 {
            character = 'F';
        }

        else {
            character = ' ';
        }

        let rgb:(u8, u8, u8) = match &character {
            '0' => (77, 77, 77),
            '1' => (125, 215, 208),
            '2' => (240, 125, 65),
            '3' => (230, 75, 35),
            '4' => (110, 12, 175),
            '5' => (185, 2, 15),
            '6' => (145, 2, 185),
            '7' => (185, 240, 70),
            '8' => (20, 15, 210),
            'B' => (255, 0, 25),
            'F' => (95, 95, 95),
            ' ' => (15, 20, 70),
            _ => (252, 252, 252)
        };

        return (character, rgb);
    }
}

impl SweeperOfMines {
    fn show_bombs_fail(&mut self) {
        for bomb in &self.bomb_locations {
            if self.grid[bomb.0][bomb.1] < 100 {
                self.grid[bomb.0][bomb.1] += 100;
            }
            
        }
    }
}

impl SweeperOfMines {
    fn adjacent_changes(&mut self, coords:(usize, usize), user_click:bool) {
        let mut r_min:usize;
        let mut c_min:usize;
        let mut r_max:usize;
        let mut c_max:usize;

        let mut mins_and_maxes:Vec<usize>;
        let grid_size:usize = (self.grid_size - 1) as usize;

        let mut new_x:usize;
        let mut new_y:usize;
        let mut stop_loop:bool = true;
        let mut coord_x:usize = coords.0;
        let mut coord_y:usize = coords.1;
        let mut new_x:usize;
        let mut new_y:usize;
        let mut coords_of_zeroes:Vec<(usize, usize)> = vec![(0,0)];
        let mut zero_indexes:usize = 0;

        // user click handling
        if user_click {
            loop {
                if !stop_loop {
                    if zero_indexes < coords_of_zeroes.len() - 1 {
                        zero_indexes += 1;
                        coord_x = coords_of_zeroes[zero_indexes].0;
                        coord_y = coords_of_zeroes[zero_indexes].1;
                    }
                    else {
                        stop_loop = true;
                    }
                }

                // click on unrevealed
                if self.grid[coord_x][coord_y] < 100 {
                    self.grid[coord_x][coord_y] += 100;
                    if self.grid[coord_x][coord_y] == 110 {
                        self.playing_status = false;
                        self.show_bombs_fail();
                    }
                    if self.grid[coord_x][coord_y] == 100 {
                        if !coords_of_zeroes.contains(&(coord_x, coord_y)){
                            coords_of_zeroes.push((coord_x, coord_y));
                            stop_loop = false;
                        }
                    }
                }

                // click on already revealed square to show the adjacent ones
                else if (self.grid[coord_x][coord_y] - 100) <= 10 {
                    mins_and_maxes = space_around_coord(coord_x, coord_y, grid_size);
                    r_min = mins_and_maxes[0];
                    c_min = mins_and_maxes[1];
                    r_max = mins_and_maxes[2];
                    c_max = mins_and_maxes[3];
        
                    for x in r_min..=r_max {
                        for y in c_min..=c_max {
                            let curr = self.grid[x][y];

                            if curr < 100 {
                                self.grid[x][y] += 100;

                                if curr == 10 {
                                    self.playing_status = false;
                                    self.show_bombs_fail();
                                }
                                // conintue revealing
                                else if curr == 0 {
                                    if !coords_of_zeroes.contains(&(x, y)){
                                        coords_of_zeroes.push((x, y));
                                        stop_loop = false;
                                    }
                                }
                            }
                        }
                    }
                }
                if stop_loop {
                    break;
                }
            }
        }
        // setting bombs
        else {
            for bomb_in_locations in &self.bomb_locations {
                let row = bomb_in_locations.0;
                let col = bomb_in_locations.1;
                mins_and_maxes = space_around_coord(row, col, grid_size);
                r_min = mins_and_maxes[0];
                c_min = mins_and_maxes[1];
                r_max = mins_and_maxes[2];
                c_max = mins_and_maxes[3];

                for x in r_min..=r_max {
                    for y in c_min..=c_max {
                        if x == row && y == col {
                            self.grid[x][y] = 10;
                        }
                        else {
                            if self.grid[x][y] == 10 {
                                continue;
                            }
                            self.grid[x][y] += 1; 
                        }
                    }
                }
            }
        }
    }
}

impl SweeperOfMines {
    fn set_flag(&mut self, coords:(usize, usize)) {
        //check if square is shown
        let mut curr = self.grid[coords.0][coords.1];
        if curr >= 100 && curr < 120 {
            //nothing
        }
        else {
            if curr <= 10 {
                self.grid[coords.0][coords.1] += 120;
                self.bombs_to_cover -= 1;
            }
            else if curr >= 120 {
                self.grid[coords.0][coords.1] -= 120;
                self.bombs_to_cover += 1;
            }
        }
    }
}

fn space_around_coord(row:usize, col:usize, end_num:usize) -> Vec<usize> {
    let mut row_min:usize;
    let mut col_min:usize;
    let mut row_max:usize;
    let mut col_max:usize;

    if row == 0 {
        row_min = 0;
    }
    else {
        row_min = row - 1;
    }
    if col == 0 {
        col_min = 0;
    }
    else {
        col_min = col - 1;
    }

    row_max = row + 1;
    if row == end_num {
        row_max = end_num;
    }

    col_max = col + 1;
    if col == end_num {
        col_max = end_num;
    }

    return vec![row_min, col_min, row_max, col_max];
}



fn bomb_locations(number_to_place:i32, max_index:i32) -> Vec<(usize, usize)> {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut locations = Vec::new();
    let mut current_coords:(usize, usize);
    
    while locations.len() < number_to_place as usize {
        current_coords = (rng.gen_range(0..max_index) as usize, rng.gen_range(0..max_index) as usize);
        if !locations.contains(&current_coords) {
            locations.push(current_coords);
        }
    }
    return locations;
}


fn options_bar(ui: &mut egui::Ui) -> u8 {
    let mut option:u8 = 0;
    egui::TopBottomPanel::bottom("bottom_bar").show(ui.ctx(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Restart").clicked() {
                option = 1;
            }
            else if ui.button("8X8").clicked() {
                option = 2;
            }
            else if ui.button("16X16").clicked() {
                option = 3;
            }
            else if ui.button("32X32").clicked() {
                option = 4;
            }
        });
    });
    return option;
}

impl SweeperOfMines {
    fn check_if_won(&self) -> bool {
        let max_index = self.grid_size;

        for x in 0..max_index {
            for y in 0..max_index {
                if self.grid[x as usize][y as usize] < 10 {
                    return false;
                }
            }
        }
        return true;
    }
}


impl eframe::App for SweeperOfMines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let option_clicked = options_bar(ui);
            
            if option_clicked == 1 {
                self.restart(self.grid_size);
            }
            else if option_clicked == 2 {
                self.restart(8);
            }
            else if option_clicked == 3 {
                self.restart(16);
            }
            else if option_clicked == 4 {
                self.restart(32);
            }

            let size_display:i32 = self.grid_size;
            let mut text_for_heading = format!("Minesweeper: {}x{};\tBombs to cover:{};\tAlive?: {}", size_display, size_display, self.bombs_to_cover, self.playing_status);
            if self.victory_status {
                text_for_heading = format!("You won! Congratulations!");
            }
            ui.heading(text_for_heading);

            ui.vertical(|ui| {
                for row in 0..size_display {
                    ui.horizontal(|ui| {

                        for col in 0..size_display {
                            
                            let display_info = self.what_to_display((row as usize, col as usize));
                            let text_char:char = display_info.0;
                            let colors:(u8, u8, u8) = display_info.1;
                            let color_r:u8 = colors.0;
                            let color_g:u8 = colors.1;
                            let color_b:u8 = colors.2;

                            let response = ui.add(egui::Button::new(format!("            \n    {}    \n            ", text_char))
                            .fill(egui::Color32::from_rgb(color_r, color_g, color_b)));
                            
                            if self.playing_status {
                                if response.clicked(){
                                    self.adjacent_changes((row as usize, col as usize), true);
                                    self.victory_status = self.check_if_won();
                                    
                                }
                                else if response.secondary_clicked() {
                                    self.set_flag((row as usize, col as usize));
                                }
                            }
                        }
                    });
                }
            });
        });
    }
}
