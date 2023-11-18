
pub trait GameScreen {
    fn display(&mut self);
    fn handle_controls(&mut self, previous_screen: Option<Box<dyn GameScreen>>) -> Option<Box<dyn GameScreen>>;
}


pub struct ScreenManager {
    pub current_screen: Box<dyn GameScreen>,
    pub previous_screen: Option<Box<dyn GameScreen>>,
}

impl ScreenManager {
    pub fn change_screen(&mut self, new_screen: Box<dyn GameScreen>) {
        self.previous_screen = Some(std::mem::replace(&mut self.current_screen, new_screen));
        self.current_screen.display();
    }

    pub fn revert_screen(&mut self) {
        if let Some(prev_screen) = self.previous_screen.take() {
            self.current_screen = prev_screen;
            self.current_screen.display();
        }
    }
}