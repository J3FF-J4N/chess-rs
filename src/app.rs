use crate::{board::Board, piece::Piece};

#[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)]
pub struct TemplateApp {
    board: Board,
    currently_moving: Option<Piece>,
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
            
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown),|ui| {
                let width = ui.available_width() / BOARD_COL as f32 * 0.9;
                let height = ui.available_height() / BOARD_COL as f32 * 0.9;

                let cell_size = egui::vec2(
                    width.min(height),
                    height.min(width), //Ensuring the aspect ratio is always maintained
                ); //Scaling the grid to be 10% less than max to avoid clipping

                egui::Grid::new("board")
                    .num_columns(BOARD_COL as usize)
                    // .max_col_width(ui.available_width() / BOARD_COL * 0.9)
                    // .min_row_height(ui.available_height() / BOARD_COL * 0.98)
                    // .min_col_width(ui.available_width() / BOARD_COL * 0.98)
                    .show(ui, |ui| {
                        let mut moves = vec![];

                        if let Some(moving_piece) = self.currently_moving {
                            moves.append(&mut moving_piece.get_valid_moves());
                        }

                        self.board
                            .grid
                            .iter_mut()
                            .enumerate()
                            .for_each(|(row_idx, row)| {
                                row.iter_mut().enumerate().for_each(|(col_idx, element)| {
                                    let highlight = moves.contains(&(col_idx as u8, row_idx as u8));

                                    if let Some(element) = element {
                                        let image = element.get_image().sense(egui::Sense {
                                            click: true,
                                            drag: true,
                                            focusable: true,
                                        });

                                        let res = ui.add_sized(cell_size, image).highlight();

                                        if res.clicked() {
                                            element.is_moving = !element.is_moving; //This allows the play to deselect a chosen piece

                                            if element.is_moving == true {
                                                self.currently_moving = Some(element.to_owned())
                                            } else {
                                                self.currently_moving = None;
                                            }
                                        }

                                        if highlight == true {
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

                                        // let dropped = response.dnd_release_payload().unwrap();

                                        if highlight == true {
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
                                // ui.label("");
                            });
                    })
            });

            if let Some(moveable) = &self.currently_moving {
                println!("Currently moving: {}", moveable.get_name());
            } else {
                println!("Nothing moves");
            }
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     let cell_size = egui::vec2(
        //         ui.available_width() / BOARD_COL as f32 * 0.9,
        //         ui.available_height() / BOARD_COL as f32 * 0.9,
        //     );

        //     egui::Grid::new("board")
        //         .num_columns(BOARD_COL as usize)
        //         .show(ui, |ui| {
        //             for row in self.board.grid.iter() {
        //                 for element in row.iter() {
        //                     // Allocate space for this cell
        //                     let (rect, _response) =
        //                         ui.allocate_exact_size(cell_size, egui::Sense::click());

        //                     // Draw the outline of the cell
        //                     ui.painter().rect_stroke(
        //                         rect,
        //                         0.0,
        //                         egui::Stroke::new(1.0, egui::Color32::BLACK),
        //                     );

        //                     // If there's a piece to draw, place it inside this allocated area
        //                     if let Some(piece) = element {
        //                         let image = piece.get_image();

        //                         // You can either add the image using add_sized to fit exactly, or just `ui.put`:
        //                         // let mut image_ui = ui.add_sized(rect.size(), image);
        //                         ui.put(rect, image);
        //                     }
        //                 }
        //                 ui.end_row();
        //             }
        //         });
        // });
    }
}
