use game::{Dir, RenderInfo};
use gfx::{Commands};
use platform_types::{command, sprite, unscaled, Button, Input, Speaker, SFX};
pub use platform_types::StateParams;

pub struct State {
    pub game_state: game::State,
    pub commands: Commands,
    pub input: Input,
    pub speaker: Speaker,
}

impl State {
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        unsafe {
            features::GLOBAL_LOGGER = logger;
            features::GLOBAL_ERROR_LOGGER = error_logger;
        }

        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        features::log(&format!("{:?}", seed));

        let game_state = game::State::new(seed);

        Self {
            game_state,
            commands: Commands::default(),
            input: Input::default(),
            speaker: Speaker::default(),
        }
    }
}

impl platform_types::State for State {
    fn frame(&mut self) -> (&[platform_types::Command], &[SFX]) {
        self.commands.clear();
        self.speaker.clear();
        update_and_render(
            &mut self.commands,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        (self.commands.slice(), self.speaker.slice())
    }

    fn press(&mut self, button: Button) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button) {
        self.input.gamepad.remove(button);
    }
}

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    let sfx_opt = if input.pressed_this_frame(Button::UP) {
        state.move_player(Dir::Up)
    } else if input.pressed_this_frame(Button::DOWN) {
        state.move_player(Dir::Down)
    } else if input.pressed_this_frame(Button::LEFT) {
        state.move_player(Dir::Left)
    } else if input.pressed_this_frame(Button::RIGHT) {
        state.move_player(Dir::Right)
    } else {
        None
    };

    if input.pressed_this_frame(Button::A) {
        if input.gamepad.contains(Button::UP) {
            state.interact(Dir::Up)
        } else if input.gamepad.contains(Button::DOWN) {
            state.interact(Dir::Down)
        } else if input.gamepad.contains(Button::LEFT) {
            state.interact(Dir::Left)
        } else if input.gamepad.contains(Button::RIGHT) {
            state.interact(Dir::Right)
        }
    }

    if let Some(sfx) = sfx_opt {
        speaker.request_sfx(sfx);
    }
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    const X_OFFSET: unscaled::X = unscaled::X((command::WIDTH - (game::xy::MAX_W_INNER as unscaled::Inner)) / 2);
    const Y_OFFSET: unscaled::Y = unscaled::Y((command::HEIGHT - (game::xy::MAX_H_INNER as unscaled::Inner)) / 2);

    fn to_x(x: game::X) -> unscaled::X {
        X_OFFSET + x.get().get() * gfx::tile::WIDTH
    }

    fn to_y(y: game::Y) -> unscaled::Y {
        Y_OFFSET + y.get().get() * gfx::tile::HEIGHT
    }

    let RenderInfo {
        tiles,
        text_boxes,
        message_segments,
    } = state.render_info();

    for tile in tiles {
        commands.draw_tile(
            tile.kind,
            to_x(tile.x),
            to_y(tile.y),
        );
    }

    for text_box in text_boxes {
        commands.draw_text_box(
            to_x(text_box.min_x),
            to_y(text_box.min_y),
            to_x(text_box.max_x),
            to_y(text_box.max_y),
        );
    }

    for segment in message_segments {
        commands.print(
            &segment.text[segment.start..segment.end],
            to_x(segment.x),
            to_y(segment.y),
            6
        );
    }
}

#[inline]
fn update_and_render(
    commands: &mut Commands,
    state: &mut game::State,
    input: Input,
    speaker: &mut Speaker,
) {
    update(state, input, speaker);
    render(commands, state);
}
