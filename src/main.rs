mod views;

use malta_core::Malta;
use views::*;

struct State {
    db: Malta,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let app = Xilem::new((), app_logic);
    // app.run_windowed(EventLoop::with_user_event(), "Malta".to_string())?;
    Ok(())
}
