use super::ModuleEnum;
use crate::gui::gui::TabContent;
use crate::gui::main::DARK_THEME;
use crate::to_rstr;
use egui::Ui;
use windows::Win32::System::Diagnostics::ToolHelp::MODULEENTRY32;

// Create out custom TabContent object for this specific tab, in this case it will be for our
// module view
//
#[derive(Clone)]
pub struct ModuleView {
    pub modules: Vec<MODULEENTRY32>,
    pub selected_module_enum: Option<ModuleEnum>,
    pub selected_module: Option<MODULEENTRY32>,
}

// Form abstract link to TabContent
//
impl TabContent for ModuleView {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(DARK_THEME.background_dark)
            .inner_margin(10.0)
            .show(ui, |ui| {
                // Create a side panel between the left and right side of the view, the left side
                // will be all the modules and the right side will show details about the specific
                // module selected
                egui::SidePanel::left("modules_panel")
                    .resizable(false)
                    .default_width(ui.available_width() * 0.5)
                    .show_inside(ui, |ui| {
                        // Create a list of all modules, once clicked it will store the selected
                        // module for the right panel to display information about it
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                for module in &self.modules {
                                    // Create an enum member of all modules
                                    let module_enum = ModuleEnum::Title(to_rstr!(module.szModule));
                                    // Check if it was selected
                                    let is_selected =
                                        self.selected_module_enum.as_ref() == Some(&module_enum);

                                    // The actual selectable button in question...
                                    let button = ui.add_sized(
                                        [ui.available_width(), 20.0],
                                        egui::SelectableLabel::new(
                                            is_selected,
                                            to_rstr!(module.szModule),
                                        ),
                                    );

                                    // If it was clicked then store it
                                    if button.clicked() {
                                        self.selected_module_enum = Some(module_enum);
                                        self.selected_module = Some(*module);
                                    }
                                }
                            });
                        });
                    });
                // Create the right panel now which will show information about the currently
                // selected module
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().inner_margin(egui::Margin {
                        left: ui.available_width() * 0.1, // Add left margin for spacing
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    }))
                    .show_inside(ui, |ui| {
                        // Create a list of all modules, once clicked it will store the selected
                        // module for the right panel to display information about it
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                if let Some(module) = self.selected_module {
                                    ui.label(format!("Path : {}", to_rstr!(module.szExePath)));
                                    ui.label(format!("Size : {:X}", module.modBaseSize));
                                    ui.label(format!("Base : {:X}", module.modBaseAddr as usize));
                                }
                            });
                        });
                    });
                //egui::ScrollArea::vertical().show(ui, |ui| {});
            });
    }

    // Handle our name of the tab
    //
    fn title(&self) -> String {
        return "[>] Modules".to_string();
    }
}
