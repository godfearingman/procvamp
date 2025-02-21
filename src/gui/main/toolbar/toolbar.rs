use eframe::egui;

/// Realistically we don't need any function other than show_bar so we won't create a struct but
/// rather just a standalone function that will be called in gui/main.rs in order to display the
/// menu bar
///
pub fn show_bar(ui: &mut egui::Ui) {
    // Create the menu frame
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |_ui| {});
        ui.menu_button("Views", |_ui| {});
        ui.menu_button("Settings", |_ui| {});
    });
}
