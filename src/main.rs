fn main() {
    //println!("Runs");
    use eframe::egui;
    use std::io;
    //io::stdin().readline(&mut usr_input).expect("Failed to read the user input!");
    let window_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nikov Minesweeper",
        window_options,
        box::new(|_cc| Ok(box::new(sweeper_of_mines::size(x)))),
    );
}

struct sweeper_of_mines {
    grid_size:i32,
    bomb_num:i32;
}

impl size for sweeper_of_mines {
    fn size(x: i32) -> self {
        grid_size = x;
        bomb_num = match x {
            8 -> 10,
            16 -> 40,
            32 -> 99,
            _ -> 10
        }
    }
}

impl eframe::app for sweeper_of_mines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size_display:i32 = self.grid_size;
            ui.heading("Minesweeper: {}x{}", size_display, size_display);
        });
    }
}
