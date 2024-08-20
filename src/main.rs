pub mod ui;

use ui::login_window::LoginWindow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::run(Box::new(LoginWindow)).await?;

    Ok(())
}
