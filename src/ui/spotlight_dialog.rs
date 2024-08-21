use ratatui::widgets::*;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct SpotlightDialog {
    search_input: String
}

impl SpotlightDialog {
    pub fn new() -> Self {
        Self {
            search_input: String::new()
        }
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
            .block(Block::bordered());

        frame.render_widget(search_input, input_area);

        frame.render_widget(Paragraph::new("Hello, World!"), items_area);
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
