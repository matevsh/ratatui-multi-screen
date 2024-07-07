use crate::views::menu::MenuView;
use crate::views::product::ProductView;
use crate::views::Screen;

pub enum AppView {
    Menu,
    Product,
}

pub struct App {
    pub view: Box<dyn Screen>,
}

impl App {
    pub fn new() -> App {
        App {
            view: Box::new(MenuView::new()),
        }
    }

    pub fn set_screen(&mut self, state: AppView) {
        self.view = match state {
            AppView::Menu => Box::new(MenuView::new()),
            AppView::Product => Box::new(ProductView::new())
        };
    }
}