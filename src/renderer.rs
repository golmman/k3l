use crate::color::Color;
use crate::common::MapPoint;
use crate::common::ScreenPoint;
use crate::screen::DefaultScreen;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::state::State;

pub mod debug_info;

pub struct Renderer {
    screen: DefaultScreen,

    debug_line_y: i32,
}

impl Renderer {
    pub fn new() -> Self {
        let screen = DefaultScreen::new();

        Self {
            screen,
            debug_line_y: 0,
        }
    }

    pub fn resize(&mut self) -> ScreenPoint {
        self.screen.resize()
    }

    pub fn display(&mut self, state: &State) {
        self.screen.clear();

        self.draw_floor(state);
        self.draw_map(state);
        self.draw_astar(state);
        self.draw_npcs(state);
        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.screen.display();
    }

    fn draw_npcs(&mut self, state: &State) {
        for npc in &state.npcs {
            let npc_sprite = Sprite::from_color_text(":-D", Color::new(52, 0));
            self.screen.draw(
                &npc_sprite,
                MapPoint::new(npc.pos.x + state.map_pos.x, npc.pos.y + state.map_pos.y).into(),
            );
        }
    }

    fn draw_astar(&mut self, state: &State) {
        {
            let astar_path_sprite = Sprite::from_color_text(" * ", Color::new(28, 0));

            for step in &state.astar_path {
                self.screen.draw(
                    &astar_path_sprite,
                    MapPoint::new(step.x + state.map_pos.x, step.y + state.map_pos.y).into(),
                );
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
