use std::borrow::BorrowMut;
use std::io::Stdout;
use std::time::Duration;

use ratatui::prelude::*;
use ratatui::widgets::*;

use ratatui::crossterm::ExecutableCommand;

use ratatui::crossterm::terminal::{
    EnterAlternateScreen,
    LeaveAlternateScreen,
    enable_raw_mode,
    disable_raw_mode
};

use ratatui::crossterm::event::{
    Event,
    poll as poll_event,
    read as read_event
};

pub mod login_window;

pub const EVENT_UPDATE_DURATION: Duration = Duration::from_millis(50);

pub enum WindowUpdate {
    /// (Re-)Render current window's UI.
    Draw,

    /// Open new window on top of the current one.
    New(Box<dyn Window + Send + Sync>),

    /// Close the current window.
    Close,

    /// Do nothing.
    None
}

pub enum WindowMode {
    /// Keys are handled by the application
    /// to navigate through the interface.
    Navigate,

    /// Keys are handled by the application
    /// to search available actions in a spotlight dialog.
    Search,

    /// Keys are handled by the currently active window.
    Insert
}

#[async_trait::async_trait]
pub trait Window {
    /// Draw the window's interface.
    ///
    /// This method is called when the `update` method
    /// says so.
    fn draw(&mut self, area: Rect, frame: &mut Frame);

    /// Update the window's state.
    ///
    /// This method is called in a loop without any delay.
    async fn update(&mut self) -> anyhow::Result<WindowUpdate> {
        Ok(WindowUpdate::None)
    }

    /// Handle incoming event.
    async fn handle(&mut self, _event: Event) -> anyhow::Result<WindowUpdate> {
        Ok(WindowUpdate::None)
    }
}

/// Draw window in the terminal.
fn draw_window(
    window: &mut Box<dyn Window + Send + Sync>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>
) -> anyhow::Result<()> {
    terminal.draw(|frame| {
        let [frame_rect, status_bar_rect] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3)
        ]).areas(frame.size());

        window.draw(frame_rect, frame);
    })?;

    Ok(())
}

/// Run the application starting with the given window.
pub async fn run(window: Box<dyn Window + Send + Sync>) -> anyhow::Result<()> {
    enable_raw_mode()?;

    std::io::stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    let mut windows = Vec::from([window]);
    let mut mode = WindowMode::Navigate;

    while !windows.is_empty() {
        // Update the window.
        if let Some(window) = windows.last_mut() {
            match window.update().await? {
                WindowUpdate::Draw => draw_window(window, &mut terminal)?,

                WindowUpdate::New(new_window) => windows.push(new_window),

                WindowUpdate::Close => {
                    windows.pop();

                    terminal.clear()?;
                }

                WindowUpdate::None => ()
            }
        }

        // Handle incoming event.
        if let Some(window) = windows.last_mut() {
            if poll_event(EVENT_UPDATE_DURATION)? {
                let event = read_event()?;

                match mode {
                    WindowMode::Navigate => (),
                    WindowMode::Search => (),

                    WindowMode::Insert => {
                        match window.handle(event).await? {
                            WindowUpdate::Draw => draw_window(window, &mut terminal)?,

                            WindowUpdate::New(new_window) => windows.push(new_window),

                            WindowUpdate::Close => {
                                windows.pop();

                                terminal.clear()?;
                            }

                            WindowUpdate::None => ()
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;

    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
