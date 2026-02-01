use bevy::prelude::{App, AppExit, DefaultPlugins};
use flappy_lib::plugins::*;
fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        CameraPlugin,
        ScorePlugin,
        PlayerPlugin,
        PipePlugin,
        BackgroundPlugin,
    ));
    app.run()
}
