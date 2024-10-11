use view::*;
use xilem::*;
use malta::Malta;

struct State {
    db: Malta,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let state = State {
        db: Malta::open().await?,
    };
    assert!(matches!(
        state.db.add_project("p1".to_string(), None).await,
        Ok(_)
    ));
    let app = Xilem::new(state, app_logic);
    app.run_windowed(EventLoop::with_user_event(), "Malta".to_string())?;
    Ok(())
}

fn app_logic(state: &mut State) -> impl WidgetView<State> {
    flex((prose("Hello"), label("world"))).direction(Axis::Vertical)
}
