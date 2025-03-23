use malta_core::Malta;
use view::*;
use xilem::*;

struct State {
    db: Malta,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Xilem::new((), app_logic);
    app.run_windowed(EventLoop::with_user_event(), "Malta".to_string())?;
    Ok(())
}

fn app_logic(_state: &mut State) -> impl WidgetView<State> {
    flex((prose("Hello"), label("world"))).direction(Axis::Vertical)
}
