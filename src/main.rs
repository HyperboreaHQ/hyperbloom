pub mod ui;

use ui::draw;
use ui::login_window::LoginWindow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    draw(Box::new(LoginWindow)).await?;

    Ok(())
}
