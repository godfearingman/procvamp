use super::ProcessEnum;
use crate::process::Process;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub struct AttachView {
    process_list: Vec<Process>,
    selected_process_enum: Option<ProcessEnum>,
    selected_process: Option<Process>,
    search_string: String,
}

impl AttachView {
    pub unsafe fn new() -> anyhow::Result<Self> {
        Ok(Self {
            process_list: Process::get_processes()?,
            selected_process_enum: None,
            selected_process: None,
            search_string: String::default(),
        })
    }
}

/// Our attach view, this is the screen the client will initially see when it comes to selecting a
/// process to debug
impl AttachView {
    pub fn show(&mut self, ctx: &egui::Context) -> Option<Process> {
        // Set our return value, this will be set if a process is double clicked
        let mut return_process = None;

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
                    egui::Frame::default()
                        .inner_margin(10.0)
                        .outer_margin(8.0)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 0.0),
                            blur: 8.0,
                            spread: 8.0,
                            color: egui::Color32::from_rgba_unmultiplied(45, 45, 45, 40),
                        })
                        .show(ui, |ui| {
                            // We don't know what the button height will be for every client so we want
                            // to make sure the spacing is directly retreived from egui so we can
                            // adjust our rows accordingly
                            let button_height = ui.spacing().interact_size.y;
                            // Create our table which will be used as a sort of listbox in this case,
                            // it's prettier.
                            TableBuilder::new(ui)
                                .column(Column::exact(300.0))
                                // We have to set a minimum simply because after searching, the
                                // table seems to reduce in size when erased back to searching for
                                // every process
                                .min_scrolled_height(
                                    self.process_list.len() as f32 * (button_height / 15.0),
                                )
                                // Create our headers which will show what the value retreived is
                                // showing.
                                .header(50.0, |mut header| {
                                    header.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.heading("Processes");
                                            ui.add_space(4.0);
                                            ui.add(
                                                egui::TextEdit::singleline(&mut self.search_string)
                                                    .frame(true)
                                                    .hint_text("Search processes..."),
                                            );
                                        });
                                    });
                                })
                                // Over here we're going to iterate over every single process and display
                                // the information in a pretty fashion, each process entry gets its own row
                                // in order to not mess up height shiet.
                                .body(|mut body| {
                                    // Create a new process list, if the string search is not blank
                                    // then we'll need a different list
                                    let new_proc_list: Vec<Process> =
                                        self.process_list
                                            .clone()
                                            .into_iter()
                                            .filter(|proc| {
                                                self.search_string.is_empty()
                                                    || proc.name().to_lowercase().contains(
                                                        &self.search_string.to_lowercase(),
                                                    )
                                            })
                                            .collect::<Vec<Process>>();
                                    new_proc_list.iter().for_each(|proc| {
                                        // Create an enum member of the current process and compare it
                                        // with the current selected one, or replace it if clicked.
                                        let process_enum = ProcessEnum::Title(format!(
                                            "{}##{}",
                                            proc.name(),
                                            proc.pid()
                                        ));
                                        let is_selected = self.selected_process_enum.as_ref()
                                            == Some(&process_enum);
                                        // Create our table entries
                                        body.row(button_height, |mut row| {
                                            row.col(|ui| {
                                                ui.centered_and_justified(|ui| {
                                                    let label = ui
                                                        .selectable_label(is_selected, proc.name());
                                                    if label.double_clicked() {
                                                        // Double click signals an attachment,
                                                        // we're going to essentially return the
                                                        // process that we're trying to attach to
                                                        // back to the main gui which will start up
                                                        // the debug screen with this process
                                                        self.selected_process_enum =
                                                            Some(process_enum);
                                                        self.selected_process = Some(proc.clone());
                                                        return_process = Some(proc.clone());
                                                    } else if label.clicked() {
                                                        // If clicked we just want to update the entry
                                                        // to be this newly clicked one as well as
                                                        // setting the process directly so that we
                                                        // don't need to try search for it again
                                                        // later
                                                        self.selected_process_enum =
                                                            Some(process_enum);
                                                        self.selected_process = Some(proc.clone());
                                                    }
                                                });
                                            });
                                        });
                                    });
                                });
                        });
                });
        });
        // Return to main gui
        return_process
    }
}
