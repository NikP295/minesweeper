use eframe::egui;
//use std::io;
use rand::prelude::*;



fn main() {
    //println!("Runs");
    //io::stdin().readline(&mut usr_input).expect("Failed to read the user input!");
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

// struct SweeperOfMines {
//     grid_size:i32,
//     bomb_num:i32;
// }
struct SweeperOfMines{
    bomb_num:i32,
    grid_size:i32,
    bomb_locations:Vec<(usize, usize)>,
    grid:Vec<Vec<i32>>
}

trait Begin {
    fn begin(size:i32) -> Self;
}

impl Begin for SweeperOfMines {
    fn begin(size: i32) -> Self {
        let bomb_num = match size {
            8 => 10,
            16 => 40,
            32 => 99,
            _ => 10
        };
        let mut grid = vec![vec![0; size as usize]; size as usize];

        let bomb_locations = bomb_locations(bomb_num, size);

        SweeperOfMines {
            grid_size: size,
            bomb_num,
            grid,
            bomb_locations
        }
    }
}

impl SweeperOfMines {
    fn adjacent_changes(&mut self, coords:(usize, usize), click_or_set:bool) {
        let mut mins_and_maxes:Vec<usize>;
        let mut r_min:usize;
        let mut c_min:usize;
        let mut r_max:usize;
        let mut c_max:usize;

        // user click handling
        if click_or_set {

        }
        // setting bombs
        else {
            for bomb_in_locations in &self.bomb_locations {
                let row = bomb_in_locations.0;
                let col = bomb_in_locations.1;
                mins_and_maxes = space_around_coord(row, col, (self.grid_size-1) as usize);
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


impl eframe::App for SweeperOfMines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size_display:i32 = self.grid_size;
            ui.heading(format!("Minesweeper: {}x{}", size_display, size_display));

            ui.vertical(|ui| {
                for row in 0..size_display {
                    ui.horizontal(|ui| {

                        for col in 0..size_display {
                            if ui.button(format!("{}", self.grid[row as usize][col as usize])).clicked() {
                                println!("Tile at {},{} clicked!", row, col);
                            }
                        }
                    });
                }
            });
            
        });
    }
}
