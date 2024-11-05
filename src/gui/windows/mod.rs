use super::gui::Window;

// Define structure for how we manage all currently opened windows/tabs
//
pub struct ActiveWindows {
    // Anytime we open a tab through a dropdown, it'll be pushed onto this array
    //
    pub tabs: Vec<Window>,
}

// Define any functions we need
//
impl ActiveWindows {
    // our constructor, don't need to do anything here
    //
    pub fn default() -> Self {
        Self { tabs: vec![] }
    }
    // Push any newly opened tabs onto the array
    //
    pub fn add_tab(&mut self, new_tab: Window) {
        self.tabs.push(new_tab)
    }
    // Remove tab on close
    //
    pub fn remove_tab(&mut self, to_find: Window) -> Option<Window> {
        self.tabs
            .iter()
            .position(|tab| tab.win_content.title() == to_find.win_content.title())
            .map(|pos| self.tabs.remove(pos))
    }
    // Get latest element, when a new tab is opened we'll focus that
    //
    pub fn get_latest_tab(&self) -> Option<&Window> {
        self.tabs.last()
    }
}

pub mod disassembly_view;
pub mod function_view;
pub mod graph_view;
pub mod scanner_results;
pub mod scanner_view;
