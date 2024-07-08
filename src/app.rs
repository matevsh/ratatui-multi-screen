use std::rc::Rc;

use crate::views::menu::MenuView;
use crate::views::product::ProductView;
use crate::views::Screen;

pub enum AppView {
    Menu,
    Product,
}

#[derive(Clone)]
pub struct App {
    pub view: Rc<dyn Screen>,
}

impl App {
    pub fn new() -> App {
        App {
            view: Rc::new(MenuView::new()),
        }
    }

    pub fn set_screen(&mut self, state: AppView) {
        self.view = match state {
            AppView::Menu => Rc::new(MenuView::new()),
            AppView::Product => Rc::new(ProductView::new()),
        };
    }
}
