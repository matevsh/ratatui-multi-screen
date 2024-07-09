use crate::views::menu::MenuView;
use crate::views::product::ProductView;
use crate::views::Screen;

pub enum AppView {
    Menu,
    Product,
}

dyn_clone::clone_trait_object!(Screen);

#[derive(Clone)]
pub struct App {
    pub view: Box<dyn Screen>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            view: Box::new(MenuView::new()),
        }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn set_screen(&mut self, state: AppView) {
        self.view = match state {
            AppView::Menu => Box::new(MenuView::new()),
            AppView::Product => Box::new(ProductView::new()),
        };
    }
}
