use crate::color::Color;
use crate::common::MapPoint;
use crate::common::ScreenPoint;
use crate::screen::DefaultScreen;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::state::State;

pub struct Renderer {
    screen: DefaultScreen,
}

impl Renderer {
    pub fn new() -> Self {
        let screen = DefaultScreen::new();

        Self { screen }
    }

    pub fn resize(&mut self) -> ScreenPoint {
        self.screen.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.screen.clear();

        self.draw_floor(state);
        self.draw_map(state);
        self.draw_astar(state);
        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.screen.display();
    }

    fn draw_astar(&mut self, state: &State) {
        {
            let astar_path_sprite = Sprite::from_color_text(" * ", Color::new(28, 0));

            if let Some(path) = &state.astar_path {
                for step in path {
                    self.screen.draw(
                        &astar_path_sprite,
                        MapPoint::new(step.x + state.map_pos.x, step.y + state.map_pos.y).into(),
                    );
                }
            }
        }

        {
            let astar_start_sprite = Sprite::from_color_text(" S ", Color::new(34, 0));

            self.screen.draw(
                &astar_start_sprite,
                MapPoint::new(
                    state.astar_start.x + state.map_pos.x,
                    state.astar_start.y + state.map_pos.y,
                )
                .into(),
            );
        }

        {
            let astar_goal_sprite = Sprite::from_color_text(" G ", Color::new(34, 0));

            self.screen.draw(
                &astar_goal_sprite,
                MapPoint::new(
                    state.astar_goal.x + state.map_pos.x,
                    state.astar_goal.y + state.map_pos.y,
                )
                .into(),
            );
        }
    }

    fn draw_cursor(&mut self, state: &State) {
        let pixels = vec![Pixel {
            ch: 'X',
            color: Color {
                bg_color: None,
                fg_color: Some(2),
            },
        }];

        let cursor = Sprite {
            pixels,
            size: ScreenPoint::new(1, 1),
        };

        self.screen
            .draw(&cursor, ScreenPoint::from(state.cursor_pos.clone()).right());
    }

    fn draw_debug_info(&mut self, state: &State) {
        if !state.show_debug_info {
            return;
        }

        let state_info_str = format!(
            "cols: {}, rows: {}, tiles_x: {}, tiles_y: {}, time: {}",
            self.screen.size.width(),
            self.screen.size.height(),
            state.screen_size.width(),
            state.screen_size.height(),
            state.elapsed_time,
        );
        let state_info = Sprite::from(state_info_str);
        self.screen
            .draw(&state_info, ScreenPoint::new(10, 3));

        let pos_info_str = format!(
            "map_x: {}, map_y: {}, cursor_x: {}, cursor_y: {}, astar_path: {:?}",
            state.map_pos.x,
            state.map_pos.y,
            state.cursor_pos.x,
            state.cursor_pos.y,
            state
                .astar_path
                .as_ref()
                .map(|p| p.len()),
        );
        let pos_info = Sprite::from(pos_info_str);
        self.screen
            .draw(&pos_info, ScreenPoint::new(10, 4));
    }

    fn draw_floor(&mut self, state: &State) {
        let mut pixels = Vec::new();
        let width = state.screen_size.width();
        let height = state.screen_size.height();

        for _i in 0..(width * height) {
            pixels.push(Pixel::from('['));
            pixels.push(Pixel::from('-'));
            pixels.push(Pixel::from(']'));
        }

        let sprite = Sprite {
            pixels,
            size: state.screen_size.clone().into(),
        };

        self.screen
            .draw(&sprite, ScreenPoint::new(0, 0));
    }

    fn draw_map(&mut self, state: &State) {
        let sprite = state.get_map_sprite();

        self.screen
            .draw(&sprite, state.map_pos.clone().into());
    }
}
