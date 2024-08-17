use ratatui::widgets::*;

use super::*;

pub struct LoginWindow;

#[async_trait::async_trait]
impl Window for LoginWindow {
    async fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<WindowResult> {
        terminal.draw(|frame| {
            frame.render_widget(Text::raw("Hello, World"), frame.size());
        })?;

        Ok(WindowResult::None)
    }
}
