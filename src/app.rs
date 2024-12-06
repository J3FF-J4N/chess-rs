use crate::{board::Board, piece::Piece};

#[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] 
pub struct TemplateApp {
    
    board: Board,
    pieces: Vec<Piece>,

}

// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//             // Example stuff:
//             board: Board::init_board_state(),
//             value: 2.7,
//         }
//     }
// }

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.


        Self { board: Board::init_board_state(), pieces: Vec::new() }
        
        // Default::default()
    }

    pub fn row (&mut self, ui: &mut egui::Ui) {
        
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
        ui.label("Test");
    }
}

const BOARD_COL: f32 = 8.0;

impl eframe::App for TemplateApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add(
            //     egui::Image::new(egui::include_image!("../assets/Casual/Pieces/Chess_white_casual/Pawn.png"))
            //         .rounding(5.0)
            // );

            egui::Grid::new("board").num_columns(BOARD_COL as usize).max_col_width(ui.available_width() / BOARD_COL * 0.9).min_row_height(ui.available_height() / BOARD_COL * 0.9).min_col_width(ui.available_width() / BOARD_COL * 0.9).show(ui, |ui| {//Scaling the grid to be 10% less than max to avoid clipping

                self.board.grid.iter().for_each(|&row| {

                    

                    row.iter().for_each(|element|{

                        let rect = egui::Rect { min: egui::Pos2::new(ui.available_height() * 0.98, ui.available_width() * 0.98), max: egui::Pos2::new(ui.available_height() * 0.98, ui.available_width() * 0.98) }; //Scaling the grid to be 5% less than max to avoid clipping
                        //ui.allocate_rect(egui::Rect { min: egui::Pos2::new(ui.available_height() / BOARD_COL, ui.available_width() / BOARD_COL), max: egui::Pos2::new(ui.available_height() / BOARD_COL, ui.available_width() / BOARD_COL) }, egui::Sense { click: true, drag: true, focusable: true });
                        
                        //ui.dnd_drag_source(id, payload, add_contents)

                        // ui.dnd_drop_zone(frame, add_contents)

                        // ui.painter().hline(rect.x_range(), rect.center().y, egui::Stroke::new(1.0, egui::Color32::RED));

                        // ui.painter().vline(rect.center().x, rect.y_range(), egui::Stroke::new(1.0, egui::Color32::RED));

                        

                        if element.is_some() {
                            let image = element.as_ref().unwrap().get_image();

                            image.paint_at(ui, rect);

                            // ui.painter().hline(x, y, stroke);

                            let response = ui.add(image);

                            //TODO Do stuff when the image is clicked
                        } else {
                            // ui.add(rect);
                            ui.allocate_rect(rect, egui::Sense { click: false, drag: false, focusable: false });
                            // ui.label("");
                        }
                    });

                    ui.end_row();
                    // ui.label("");
                });

                // ui.add(egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Pawn.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Pawn.png")));

                // self.board.grid.iter().for_each(|&row| {
                //     row.iter().for_each(|element|{
                //         print!("{:#?}", element);
                //     });
                //     println!("");
                // });

                // println!("");
                
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
                // self.row(ui);
                // ui.end_row();
            })

        });
    }
}