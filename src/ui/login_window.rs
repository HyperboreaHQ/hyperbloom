use ratatui::widgets::*;

use super::*;

pub struct LoginWindow;

#[async_trait::async_trait]
impl Window for LoginWindow {
    fn draw(&mut self, area: Rect, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("Hello, World!"), area);
    }
}
