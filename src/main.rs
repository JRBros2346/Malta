use db::Database;
use view::*;
use xilem::*;

mod db;
mod types;

struct State {
    db: Database,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = State {
        db: Database::open(),
    };
    let app = Xilem::new(state, app_logic);
    app.run_windowed(EventLoop::with_user_event(), "Malta".to_string())?;
    Ok(())
}

fn app_logic(state: &mut State) -> impl WidgetView<State> {
    flex((prose("Hello"), label("world"))).direction(Axis::Vertical)
}
