use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{
    app::App,
    menu::{Menu, MenuItem},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let item_1 = MenuItem::new(0, "Foo");
    let item_2 = MenuItem::new(1, "Bar");

    let items = vec![item_1, item_2];

    // let list = Menu::new(items);

    // let items = [
    //     ListItem::new("Item 1"),
    //     ListItem::new("Item 2"),
    //     ListItem::new("Item 3"),
    // ];
    let mut state = ListState::default();
    state.select(Some(1));
    let items = vec![ListItem::new("Item 1"), ListItem::new("Item 2")];
    let list = List::new(items);
    let area = Rect::new(0, 0, 5, 5);
    f.render_stateful_widget(list, layout[0], &mut state);
}
