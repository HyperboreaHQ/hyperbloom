use ratatui::widgets::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginWindow {
    current_tab: u8
}

impl LoginWindow {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            current_tab: 0
        })
    }
}

#[async_trait::async_trait]
impl Window for LoginWindow {
    fn get_title(&self) -> String {
        String::from("Login")
    }

    fn draw(&mut self, area: Rect, frame: &mut Frame) {
        // Draw the empty block to hide background.
        frame.render_widget(Block::new(), area);

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

        let [tabs_area, area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1)
        ]).areas(area);

        // Draw the tabs of the window.
        let tabs = Layout::horizontal([
           Constraint::Length(10),
           Constraint::Length(8),
           Constraint::Fill(1)
        ]).areas::<3>(tabs_area);

        let tabs = [
            ("Login", tabs[0]),
            ("New", tabs[1])
        ];

        for (i, (tab_name, tab_area)) in tabs.into_iter().enumerate() {
            let tab = Line::from(vec![
                Span::styled(format!(" F{} ", i + 1), Style::new().gray()),
                Span::raw(tab_name)
            ]);

            if i == self.current_tab as usize {
                frame.render_widget(tab.on_blue(), tab_area);
            } else {
                frame.render_widget(tab, tab_area);
            }
        }

        // Draw the tab's content.
        frame.render_widget(Block::bordered(), area);

        match self.current_tab {
            // Login
            0 => {

            }

            // New
            1 => {

            }

            _ => unreachable!()
        }
    }

    async fn handle(&mut self, event: Event) -> anyhow::Result<WindowUpdate> {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::F(1) => self.current_tab = 0,
                KeyCode::F(2) => self.current_tab = 1,

                _ => ()
            }

            return Ok(WindowUpdate::Draw);
        }

        Ok(WindowUpdate::None)
    }
}
