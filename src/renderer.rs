use crate::color::Color;
use crate::common::TILE_W;
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

    pub fn resize(&mut self) -> (u16, u16) {
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
            let astar_start_sprite = Sprite::from_color_text(
                " S ",
                Color {
                    bg_color: 34,
                    fg_color: 0,
                },
            );

            self.screen.draw(
                &astar_start_sprite,
                state.astar_start.x + state.map_pos.x,
                state.astar_start.y + state.map_pos.y,
            );
        }

        {
            let astar_goal_sprite = Sprite::from_color_text(
                " G ",
                Color {
                    bg_color: 34,
                    fg_color: 0,
                },
            );

            self.screen.draw(
                &astar_goal_sprite,
                state.astar_goal.x + state.map_pos.x,
                state.astar_goal.y + state.map_pos.y,
            );
        }
    }

    fn draw_cursor(&mut self, state: &State) {
        let pixels = vec![Pixel {
            ch: 'X',
            color: Color {
                bg_color: 2,
                fg_color: 0,
            },
        }];
        let width = 1;
        let height = 1;

        let cursor = Sprite {
            pixels,
            width,
            height,
        };

        self.screen.draw(
            &cursor,
            state.cursor_pos.x as i16,
            state.cursor_pos.y as i16,
        );
    }

    fn draw_debug_info(&mut self, state: &State) {
        let state_info_str = format!(
            "cols: {}, rows: {}, time: {}",
            state.screen_width, state.screen_height, state.elapsed_time,
        );
        let state_info = Sprite::from(state_info_str);
        self.screen.draw(&state_info, 10, 3);

        let pos_info_str = format!(
            "map_x: {}, map_y: {}, cursor_x: {}, cursor_y: {}",
            state.map_pos.x, state.map_pos.y, state.cursor_pos.x, state.cursor_pos.y
        );
        let pos_info = Sprite::from(pos_info_str);
        self.screen.draw(&pos_info, 10, 4);
    }

    fn draw_floor(&mut self, state: &State) {
        let mut pixels = Vec::new();
        let width = (state.screen_width / TILE_W) * TILE_W;
        let height = state.screen_height;

        for _i in 0..((width / TILE_W) * height) {
            pixels.push(Pixel::from('['));
            pixels.push(Pixel::from('-'));
            pixels.push(Pixel::from(']'));
        }

        let sprite = Sprite {
            pixels,
            width,
            height,
        };

        self.screen.draw(&sprite, 0, 0);
    }

    fn draw_map(&mut self, state: &State) {
        let sprite = state.get_map_sprite();

        self.screen
            .draw(&sprite, state.map_pos.x, state.map_pos.y);
    }
}
