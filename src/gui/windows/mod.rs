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
            .position(|tab| tab.name == to_find.name)
            .map(|pos| self.tabs.remove(pos))
    }
}
