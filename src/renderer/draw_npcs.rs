use super::Renderer;
use crate::screen::{Animation, Sprite};
use crate::state::npc::NpcAnimationId;
use crate::state::State;

impl Renderer {
    pub fn draw_npcs(&mut self, state: &State) {
        for npc in &state.npcs {
            let base_npc = &state.get_base_npc(npc);

            let sprite = match npc.animation {
                NpcAnimationId::Idle => Renderer::get_sprite(state, &base_npc.animation_idle),
                NpcAnimationId::Walk => Renderer::get_sprite(state, &base_npc.animation_walk),
                NpcAnimationId::Run => todo!(),
                NpcAnimationId::Attack => todo!(),
            };

            self.screen
                .draw(&sprite, (&npc.pos + &state.map_pos).into());
        }
    }

    fn get_sprite<'a>(state: &'a State, animations: &'a Vec<Animation>) -> &'a Sprite {
        let sprites = &animations[0].sprites;
        let frame = (state.elapsed_time % sprites.len() as u64) as usize;

        &sprites[frame]
    }
}
