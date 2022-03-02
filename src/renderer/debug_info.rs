use crate::common::{MapPoint, ScreenPoint};
use crate::screen::Sprite;
use crate::state::State;

use super::Renderer;

pub const DEBUG_INFO_PAGE_TOTAL: i32 = 2;

impl Renderer {
    pub fn draw_debug_info(&mut self, state: &State) {
        match state.debug_info_page {
            0 => return,
            1 => self.draw_debug_info_general(state),
            2 => self.draw_debug_info_tasks(state),
            _ => panic!("debug info page {} out of bounds", state.debug_info_page),
        }
    }

    fn draw_next_line(&mut self, formatted_string: String) {
        let sprite = Sprite::from(formatted_string);
        self.screen
            .draw(&sprite, ScreenPoint::new(0, self.debug_line_y));
        self.debug_line_y += 1;
    }

    fn draw_page_info(&mut self, state: &State, text: &str) {
        self.debug_line_y = 0;

        self.draw_next_line(format!(
            //"\x1b[1m{}/{} {}\x1b[22m",
            "{}/{} {}",
            state.debug_info_page, DEBUG_INFO_PAGE_TOTAL, text,
        ));
    }

    fn draw_debug_info_general(&mut self, state: &State) {
        self.draw_page_info(state, "General");

        self.draw_next_line(format!(
            "cols: {}, rows: {}, tiles_x: {}, tiles_y: {}, time: {}",
            self.screen.size.width(),
            self.screen.size.height(),
            state.screen_size.width(),
            state.screen_size.height(),
            state.elapsed_time,
        ));

        self.draw_next_line(format!(
            "map_x: {}, map_y: {}, cursor_x: {}, cursor_y: {}, astar_path: {:?}",
            state.map_pos.x,
            state.map_pos.y,
            state.cursor_pos.x,
            state.cursor_pos.y,
            state.astar_path.len(),
        ));

        self.draw_next_line(format!("{:?}", state.selection));

        let cursor_map_coordinates = MapPoint::new(
            state.cursor_pos.x - state.map_pos.x,
            state.cursor_pos.y - state.map_pos.y,
        );
        let base_tile = state.get_base_tile_at(&cursor_map_coordinates);
        self.draw_next_line(format!(
            "tile_name: {:?}",
            base_tile.map(|b| b.name.clone())
        ));
    }

    fn draw_debug_info_tasks(&mut self, state: &State) {
        self.draw_page_info(state, "Tasks");
        for task in &state.cursor_tasks {
            self.draw_next_line(format!("{}", task));
        }
    }
}
