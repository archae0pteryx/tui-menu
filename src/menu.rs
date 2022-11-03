use tui::{
    style::Color,
    widgets::{BorderType, Borders, ListState, ListItem},
};

pub struct MenuTheme {
    pub primary: Color,
    pub secondary: Color,
    pub border_color: Color,
    pub border_type: BorderType,
    pub borders: Borders,
    pub item_fg: Color,
    pub item_bg: Color
}

impl Default for MenuTheme {
    fn default() -> Self {
        MenuTheme {
            primary: Color::White,
            secondary: Color::Green,
            border_color: Color::White,
            border_type: BorderType::Rounded,
            borders: Borders::ALL,
            item_fg: Color::White,
            item_bg: Color::Black
        }
    }
}

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub state: ListState,
    pub theme: MenuTheme,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            items: Vec::new(),
            state: ListState::default(),
            theme: MenuTheme::default(),
        }
    }
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            ..Menu::default()
        }
    }

    fn make_list(&self) -> Vec<ListItem> {
        self.items
            .iter()
            .map(|item| ListItem::new(item.text.clone()).style(style))
            .collect()
    }

    pub fn set_items(&mut self, items: Vec<MenuItem>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn build(&self) -> &Self {
      let constructed = self.clone();

      self
    }
}

pub struct MenuItem {
    pub id: usize,
    pub text: String,
    pub visible: bool,
    pub selected: bool,
}

impl Default for MenuItem {
    fn default() -> Self {
        MenuItem {
            id: 0,
            text: String::new(),
            visible: true,
            selected: false,
        }
    }
}

impl MenuItem {
    pub fn new(id: usize, text: &str) -> Self {
        MenuItem {
            id,
            text: text.to_owned(),
            ..MenuItem::default()
        }
    }

    pub fn set_id(&mut self, id: usize) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }

    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }

    pub fn toggle(&mut self) -> &mut Self {
        self.selected = !self.selected;
        self
    }
}

// fn new(items: Vec<String>) -> Events {
//         Events {
//             items,
//             state: ListState::default(),
//         }
//     }

//     // Select the next item. This will not be reflected until the widget is drawn in the
//     // `Terminal::draw` callback using `Frame::render_stateful_widget`.

//     // Unselect the currently selected item if any. The implementation of `ListState` makes
//     // sure that the stored offset is also reset.
//     pub fn unselect(&mut self) {
//         self.state.select(None);
//     }
