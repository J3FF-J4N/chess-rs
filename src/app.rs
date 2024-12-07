use crate::{board::Board, piece::Piece};

#[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)]
pub struct TemplateApp {
    board: Board,
    currently_moving: Option<Piece>,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            board: Board::init_board_state(),
            currently_moving: None,
        }

        // Default::default()
    }
}

const BOARD_COL: f32 = 8.0;

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:

        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //             }
        //         });
        //         ui.add_space(16.0);

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let width = ui.available_width() / BOARD_COL as f32 * 0.95;
                    let height = ui.available_height() / BOARD_COL as f32* 0.95;

                    let cell_size = egui::vec2(
                        width.min(height),
                        height.min(width), //Ensuring the aspect ratio is always maintained
                    ); //Scaling the grid to be 10% less than max to avoid clipping

                    self.board.update_board(self.currently_moving);

                    egui::Grid::new("board")
                        .num_columns(BOARD_COL as usize)
                        // .max_col_width(ui.available_width() / BOARD_COL * 0.9)
                        // .min_row_height(ui.available_height() / BOARD_COL * 0.98)
                        // .min_col_width(ui.available_width() / BOARD_COL * 0.98)
                        .show(ui, |ui| {
                            
                            
                            
                            
                            let mut moves = vec![]; //TODO Ensure there are no unnecessary memory allocations

                            

                            //TODO Filter moves that are blocked by other pieces
                            if let Some(moving_piece) = self.currently_moving {
                                // moves.append(&mut moving_piece.get_valid_moves());
                                moves.append(&mut self.board.get_valid_moves(&moving_piece));
                            }

                            self.board
                                .grid
                                .iter_mut()
                                .enumerate()
                                .for_each(|(row_idx, row)| {
                                    row.iter_mut().enumerate().for_each(|(col_idx, element)| {

                                        let highlight =
                                            moves.contains(&(col_idx as u8, row_idx as u8));

                                        if let Some(element) = element {


                                            /*When there is a piece present in the path of the moving piece the position from the current piece is removed as available movement spot.
                                            When the piece is of the opposing colour the field is highligted as possible capture point.*/
                                            // if highlight == true { //Only if the highlight is true does this code need to be executed

                                            //     // moves.iter().for_each(|to_filter|{

                                            //     //     if highlight == true { //If the highlight is already false there is no need to continue checking for equality
                                            //     //         highlight = !element.pos_x.eq(&to_filter.0) || !element.pos_y.eq(&to_filter.1); //If either of the coordinates differ the highlight is kept
                                            //     //     }
                                        
                                                    
                                            //     // });

                                            // }


                                            let image = element.get_image().sense(egui::Sense {
                                                click: true,
                                                drag: true,
                                                focusable: true,
                                            });

                                            let res = ui.add_sized(cell_size, image);

                                            if res.clicked() { //Select a piece for movement
                                                element.is_moving = !element.is_moving; //This allows the play to deselect a chosen piece

                                                //TODO Find a correct way to capture a piece

                                                if element.is_moving == true {
                                                    self.currently_moving = Some(element.to_owned())
                                                } else {
                                                    self.currently_moving = None;
                                                }
                                            }


                                            if res.secondary_clicked() {
                                                println!("{:#?}", element);
                                            }

                                            if highlight == true {//Highlight fields valid for movement
                                                ui.painter().rect_stroke(
                                                    res.rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::GOLD),
                                                );
                                            } else {
                                                ui.painter().rect_stroke(
                                                    res.rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                                                );
                                            }
                                         } else {                                            
                                            
                                            let (rect, response) = ui.allocate_exact_size(
                                                cell_size,
                                                egui::Sense {
                                                    click: true,
                                                    drag: true,
                                                    focusable: true,
                                                },
                                            );

                                            if response.clicked() { 

                                                if moves.contains(&(col_idx as u8, row_idx as u8)) && self.currently_moving.is_some() {//When a piece is moving set the destination when it is valid
                                                    self.currently_moving.as_mut().unwrap().move_piece(col_idx as u8, row_idx as u8);
                                                    println!("Moving {} to {} {}", self.currently_moving.as_ref().unwrap().get_name(), col_idx, row_idx);
                                                }

                                            }

                                            if response.secondary_clicked() {
                                                println!("{:#?}", element);
                                            }

                                            if highlight == true { //Highlight fields valid for movement
                                                ui.painter().rect_stroke(
                                                    rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::GOLD),
                                                );
                                            } else {
                                                ui.painter().rect_stroke(
                                                    rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                                                );
                                            }
                                        }
                                    });

                                    ui.end_row();

                                });

                                
                        })
                },
            );

            // if let Some(moveable) = &self.currently_moving {
            //     println!("Currently moving: {}", moveable.get_name());
            // } else {
            //     println!("Nothing moves");
            // }
        });
    }
}
