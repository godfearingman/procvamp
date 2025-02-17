use crate::process::Process;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub struct AttachView {
    process_list: Vec<Process>,
}

impl AttachView {
    pub unsafe fn new() -> anyhow::Result<Self> {
        Ok(Self {
            process_list: Process::get_processes()?,
        })
    }
}

/// Our attach view, this is the screen the client will initially see when it comes to selecting a
/// process to debug
impl AttachView {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Create a central panel, this view will be sort of bare bones simply because there's not
        // going to be a need for a lot to go on.
        egui::CentralPanel::default().show(ctx, |_ui| {
            // Set a background gif to be playing just to make the background look better.

            // Create an area so we can specifically set the table to be in the direct center
            let screen_rect = ctx.screen_rect();
            let center_pos = screen_rect.center();

            egui::Area::new("central_process_list".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::default())
                .fixed_pos(center_pos)
                .show(ctx, |ui| {
                    // Create our frame for our table, this is where the widget is actually
                    // affected
                    egui::Frame::default().inner_margin(10.0).show(ui, |ui| {
                        // We don't know what the button height will be for every client so we want
                        // to make sure the spacing is directly retreived from egui so we can
                        // adjust our rows accordingly
                        let button_height = ui.spacing().interact_size.y;
                        // Create our table which will be used as a sort of listbox in this case,
                        // it's prettier.
                        TableBuilder::new(ui)
                            .column(Column::exact(250.0))
                            // Create our headers which will show what the value retreived is
                            // showing.
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.heading("Processes");
                                    });
                                });
                            })
                            // Over here we're going to iterate over every single process and display
                            // the information in a pretty fashion, each process entry gets its own row
                            // in order to not mess up height shiet.
                            .body(|mut body| {
                                self.process_list.iter().for_each(|proc| {
                                    body.row(button_height, |mut row| {
                                        row.col(|ui| {
                                            let _ = ui.button(format!(
                                                "{} - {}",
                                                proc.name(),
                                                proc.pid()
                                            ));
                                        });
                                    });
                                });
                            });
                    });
                });
        });
    }
}
