use crate::gui::gui::WindowType;
use crate::gui::main::toolbar::toolbar::show_bar;
use crate::gui::main::Tab;
use crate::gui::main::TabViewer;
use crate::gui::main::Window;
use crate::gui::windows::ActiveWindows;
use crate::process::Process;
use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

pub struct DebugView {
    tree: DockState<Window<Tab>>,
    process: Process,
    windows_manager: ActiveWindows<Tab>,
    _left_index: Option<NodeIndex>,
    bottom_index: Option<NodeIndex>,
}

impl DebugView {
    /// Create a function to automatically append tabs onto the tree
    ///
    fn add_tab(&mut self, window: Window<Tab>) {
        // Since we store indexes this'll make adding tabs trivial, we need to just match the
        // window type and work from there
        match window.win_type {
            /*
            // Start with left most tabs
            WindowType::FunctionView => {
                // Check if we already have a left split
                if let Some(left_idx) = self.left_index {
                    self.tree.main_surface_mut().set_focused_node(left_idx);
                    self.tree.main_surface_mut().push_to_focused_leaf(window);
                    self.left_index = self.tree.main_surface().focused_leaf();
                }
                // If there's no existing left split we'll just need to create it
                else {
                    let [_, new_left] = self.tree.main_surface_mut().split_left(
                        NodeIndex::root(),
                        0.2,
                        vec![window],
                    );
                    self.left_index = Some(new_left);
                }
            }*/
            // Middle tabs are most trivial, just add it directly to middle
            WindowType::DisassemblyView
            | WindowType::ImportsView
            | WindowType::AllocationView
            | WindowType::ScannerView => {
                self.tree
                    .main_surface_mut()
                    .set_focused_node(NodeIndex::root());
                self.tree.main_surface_mut().push_to_focused_leaf(window);
            }
            // Anything else is yet to be decided wher it would go so we'll stick it at the bottom
            // instead
            _ => {
                if let Some(bottom_idx) = self.bottom_index {
                    self.tree.main_surface_mut().set_focused_node(bottom_idx);
                    self.tree.main_surface_mut().push_to_focused_leaf(window);
                    self.bottom_index = self.tree.main_surface().focused_leaf();
                }
                // If there's no existing right split we'll just need to create it like we did with
                // the left panel
                else {
                    let [_, new_bottom] = self.tree.main_surface_mut().split_below(
                        NodeIndex::root(),
                        0.6,
                        vec![window],
                    );
                    self.bottom_index = Some(new_bottom);
                }
            }
        }
    }
    /// Basic constructor to just setup our tree, windows manager and that's about it really.
    pub fn new(process: Process) -> Self {
        // Setup our initial view of the entire window
        //
        let windows_manager = ActiveWindows::default();
        let tree = DockState::new(vec![]);

        Self {
            tree,
            process,
            windows_manager,
            _left_index: None,
            bottom_index: None,
        }
    }
}

impl DebugView {
    pub fn show(&mut self, ctx: &egui::Context) {
        // Implement a frame so that we can grab the ui and send that over to show_bar
        egui::CentralPanel::default().show(ctx, |ui| {
            // We'll need to make it a vertical setup so that our toolbar is above the docking area
            ui.vertical(|ui| {
                // Show our tool bar
                if let Some(tab) = show_bar(ui, &mut self.process) {
                    self.add_tab(tab);
                }

                ui.add_space(3.0);
                // Show our tabs
                DockArea::new(&mut self.tree)
                    .style(Style::from_egui(ctx.style().as_ref()))
                    .show_inside(ui, &mut TabViewer {});
            });
        });
    }
}
