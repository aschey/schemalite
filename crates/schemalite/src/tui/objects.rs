use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget},
};

#[derive(Debug, Clone, Default)]
pub struct Objects {
    focused: bool,
}

impl Objects {
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl StatefulWidget for Objects {
    type State = ObjectsState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let items: Vec<ListItem> = state.objects.iter().map(|i| i.clone().into()).collect();

        tui::widgets::StatefulWidget::render(
            List::new(items)
                .highlight_style(Style::default().fg(Color::Green).bg(Color::Black))
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Objects",
                            Style::default().add_modifier(if self.focused {
                                Modifier::BOLD
                            } else {
                                Modifier::empty()
                            }),
                        ))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(if self.focused {
                            Color::Green
                        } else {
                            Color::White
                        })),
                ),
            area,
            buf,
            &mut state.state,
        );
    }
}

#[derive(Debug, Clone)]
pub enum ListItemType {
    Entry(String),
    Header(String),
}

impl From<ListItemType> for ListItem<'static> {
    fn from(val: ListItemType) -> Self {
        match val {
            ListItemType::Entry(title) => {
                ListItem::new(Text::styled("  ".to_owned() + &title, Style::default()))
            }
            ListItemType::Header(title) => ListItem::new(Text::styled(
                title,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectsState {
    state: ListState,
    object_view_width: usize,
    objects: Vec<ListItemType>,
    has_items: bool,
    adjusted_index: i32,
    adjusted_size: i32,
}

impl ObjectsState {
    pub fn new(tables: Vec<String>, indexes: Vec<String>) -> ObjectsState {
        let mut list_items = vec![];
        let mut has_items = false;

        has_items |= !tables.is_empty();
        list_items.push(ListItemType::Header("Tables".to_owned()));

        list_items.extend(tables.into_iter().map(ListItemType::Entry));

        has_items |= !indexes.is_empty();
        list_items.push(ListItemType::Header("Indexes".to_owned()));

        list_items.extend(indexes.into_iter().map(ListItemType::Entry));

        let max_length = list_items
            .iter()
            .map(|o| match o {
                ListItemType::Header(header) => header.len(),
                ListItemType::Entry(title) => title.len()
            }+5)
            .max()
            .unwrap_or_default();

        let mut state = ListState::default();
        if has_items {
            state.select(Some(1));
        }
        ObjectsState {
            state,
            adjusted_size: list_items.len() as i32 - 2,
            objects: list_items,
            object_view_width: max_length,
            has_items,
            adjusted_index: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.has_items {
            return;
        }
        self.adjusted_index = (self.adjusted_index + 1).rem_euclid(self.adjusted_size);

        let mut next_index = (self.state.selected().expect("Item not selected") as i32 + 1)
            .rem_euclid(self.objects.len() as i32);
        let real_index = loop {
            match self.objects.get(next_index as usize) {
                Some(ListItemType::Entry { .. }) => {
                    break next_index;
                }
                Some(ListItemType::Header(_)) => {
                    next_index = (next_index + 1).rem_euclid(self.objects.len() as i32);
                }
                None => unreachable!(),
            }
        };

        self.state.select(Some(real_index as usize));
    }

    pub fn previous(&mut self) {
        if !self.has_items {
            return;
        }
        self.adjusted_index = (self.adjusted_index - 1).rem_euclid(self.adjusted_size);

        let mut next_index = (self.state.selected().expect("Item not selected") as i32 - 1)
            .rem_euclid(self.objects.len() as i32);
        let real_index = loop {
            match self.objects.get(next_index as usize) {
                Some(ListItemType::Entry { .. }) => {
                    break next_index;
                }
                Some(ListItemType::Header(_)) => {
                    next_index = (next_index - 1).rem_euclid(self.objects.len() as i32);
                }
                None => unreachable!(),
            }
        };

        self.state.select(Some(real_index as usize));
    }

    pub fn selected(&self) -> usize {
        self.adjusted_index as usize
    }

    pub fn view_width(&self) -> usize {
        self.object_view_width
    }
}