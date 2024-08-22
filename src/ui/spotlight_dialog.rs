use ratatui::widgets::*;

use super::*;

pub struct SpotlightEntry {
    pub category: String,
    pub title: String,
    pub action: Box<dyn FnOnce() + Send + Sync>
}

impl SpotlightEntry {
    pub fn matches(&self, input: impl AsRef<str>) -> u16 {
        let category = self.category.to_lowercase();
        let title = self.title.to_lowercase();

        input.as_ref()
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                if category.contains(word) || title.contains(word) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

#[derive(Default)]
pub struct SpotlightDialog {
    search_input: String,
    entries: Vec<SpotlightEntry>
}

impl SpotlightDialog {
    pub fn new(entries: impl Into<Vec<SpotlightEntry>>) -> Box<Self> {
        Box::new(Self {
            search_input: String::new(),
            entries: entries.into()
        })
    }
}

#[async_trait::async_trait]
impl Window for SpotlightDialog {
    fn get_title(&self) -> String {
        String::from("Spotlight")
    }

    fn draw(&mut self, area: Rect, frame: &mut Frame) {
        let [_, area, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(65),
            Constraint::Fill(1)
        ]).areas(area);

        let [_, area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(85),
            Constraint::Fill(1)
        ]).areas(area);

        let [input_area, items_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1)
        ]).areas(area);

        let search_input = Paragraph::new(self.search_input.as_str())
            .left_aligned()
            .block({
                Block::bordered()
                    .title("Search input")
            });

        // Render search input.
        frame.render_widget(search_input, input_area);

        // Search through entries.
        let mut sorted_entries = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            let matches = if !self.search_input.is_empty() {
                entry.matches(&self.search_input)
            } else {
                1
            };

            if matches > 0 {
                sorted_entries.push((entry, matches));
            }
        }

        // Sort them by the matches count.
        sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));

        // Render matched entries.
        let entries_widget = List::new({
            sorted_entries.iter()
                .map(|(entry, _)| {
                    ListItem::new(format!("{} > {}", entry.category, entry.title))
                })
        }).block({
            Block::bordered()
                .title("Entries")
        });

        frame.render_widget(entries_widget, items_area);
    }

    async fn handle(&mut self, event: Event) -> anyhow::Result<WindowUpdate> {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Backspace => {
                    self.search_input.pop();
                }

                KeyCode::Char(char) => {
                    self.search_input.push(char);
                }

                _ => ()
            }

            return Ok(WindowUpdate::Draw);
        }

        Ok(WindowUpdate::None)
    }
}
