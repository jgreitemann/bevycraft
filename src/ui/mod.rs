mod hud;
mod menu;

use hud::HudPlugin;
use menu::MenuPlugin;

use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(UiBasePlugin).add(HudPlugin).add(MenuPlugin);
    }
}

struct UiBasePlugin;

impl Plugin for UiBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(UiState::Hud)
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_ui_camera)
            .add_system_to_stage(CoreStage::Update, sync_ui_state);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum UiState {
    Hud,
    Menu,
}

fn sync_ui_state(
    mut commands: Commands,
    ui_state: Res<CurrentState<UiState>>,
    turn_state: Res<CurrentState<TurnState>>,
) {
    let CurrentState(current_turn_state) = *turn_state;

    use TurnState::*;
    let supposed_ui_state = match current_turn_state {
        AwaitingInput | PlayerTurn | MonsterTurn | Reset => UiState::Hud,
        Victory | Defeat | Pause => UiState::Menu,
    };

    if *ui_state != CurrentState(supposed_ui_state) {
        commands.insert_resource(NextState(supposed_ui_state));
    }
}

fn setup_ui_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiStyles::new(asset_server.as_ref()));
    commands.spawn_bundle(UiCameraBundle::default());
}

struct UiStyles {
    default_text_style: TextStyle,
}

impl UiStyles {
    fn new(asset_server: &AssetServer) -> Self {
        UiStyles {
            default_text_style: TextStyle {
                font: asset_server.load("PixeloidSans.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        }
    }

    fn text(&self) -> TextStyle {
        self.default_text_style.clone()
    }

    fn heading(&self) -> TextStyle {
        let mut style = self.default_text_style.clone();
        style.font_size = 40.0;
        style
    }
}
