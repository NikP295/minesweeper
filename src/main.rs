use eframe::egui;
//use std::io;


fn main() {
    //println!("Runs");
    //io::stdin().readline(&mut usr_input).expect("Failed to read the user input!");
    let window_options = eframe::NativeOptions::default();
    let x = 8;
    eframe::run_native(
        "Nikov Minesweeper",
        window_options,
        Box::new(|_cc| Ok(Box::new(SweeperOfMines::size(x)))),
    );
}

// struct SweeperOfMines {
//     grid_size:i32,
//     bomb_num:i32;
// }
struct SweeperOfMines{
    bomb_num:i32,
    grid_size:i32
}

trait Size {
    fn size(x:i32) -> Self;
}

impl Size for SweeperOfMines {
    fn size(x: i32) -> Self {
        let bomb_num = match x {
            8 => 10,
            16 => 40,
            32 => 99,
            _ => 10
        };
        
        SweeperOfMines {
            grid_size: x,
            bomb_num
        }
    }
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
                            if ui.button(format!("{}x{}", row, col)).clicked() {
                                println!("Tile at {},{} clicked!", row, col);
                            }
                        }
                    });
                }
            });
            
        });
    }
}
