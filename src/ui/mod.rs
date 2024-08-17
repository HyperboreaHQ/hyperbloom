use std::borrow::BorrowMut;
use std::io::Stdout;
use std::time::Duration;

use ratatui::prelude::*;

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

pub enum WindowResult {
    New(Box<dyn Window + Send + Sync>),
    Close,
    None
}

#[async_trait::async_trait]
pub trait Window {
    async fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<WindowResult>;

    async fn handle_event(&mut self, _event: Event) -> anyhow::Result<WindowResult> {
        Ok(WindowResult::None)
    }
}

fn handle_result(
    windows: &mut Vec<Box<dyn Window + Send + Sync>>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    result: WindowResult
) -> anyhow::Result<()> {
    match result {
        WindowResult::New(new_window) => {
            windows.push(new_window);
        }

        WindowResult::Close => {
            windows.pop();

            terminal.clear()?;
        }

        WindowResult::None => ()
    }

    Ok(())
}

pub async fn draw(window: Box<dyn Window + Send + Sync>) -> anyhow::Result<()> {
    enable_raw_mode()?;

    std::io::stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    let mut windows = Vec::from([window]);

    while !windows.is_empty() {
        let mut result = None;

        // Try to handle environment event.
        if let Some(window) = windows.last_mut() {
            if poll_event(EVENT_UPDATE_DURATION)? {
                result = Some(window.handle_event(read_event()?).await?);
            }
        }

        // Update the state using the window's result.
        if let Some(result) = result.take() {
            handle_result(
                windows.borrow_mut(),
                terminal.borrow_mut(),
                result
            )?;
        }

        // Try to draw the window.
        if let Some(window) = windows.last_mut() {
            result = Some(window.draw(terminal.borrow_mut()).await?);
        }

        // Update the state using the window's result.
        if let Some(result) = result.take() {
            handle_result(
                windows.borrow_mut(),
                terminal.borrow_mut(),
                result
            )?;
        }
    }

    disable_raw_mode()?;

    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
