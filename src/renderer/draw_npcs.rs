use super::Renderer;
use crate::screen::{Sprite, Animation};
use crate::state::npc::NpcAnimationId;
use crate::state::State;

impl Renderer {
    pub fn draw_npcs(&mut self, state: &State) {
        for npc in &state.npcs {
            let base_npc = &state.get_base_npc(npc);
            let color = base_npc.color;

            let animation_frame = match npc.animation {
                NpcAnimationId::Idle => {
                    Renderer::get_animation_frame(state, &base_npc.animation_idle)
                }
                NpcAnimationId::Walk => {
                    Renderer::get_animation_frame(state, &base_npc.animation_walk)
                }
                NpcAnimationId::Run => todo!(),
                NpcAnimationId::Attack => todo!(),
            };

            let npc_sprite = Sprite::from_color_text(animation_frame, color);
            self.screen
                .draw(&npc_sprite, (&npc.pos + &state.map_pos).into());
        }
    }

    fn get_animation_frame<'a>(state: &'a State, animations: &'a Vec<Animation>) -> &'a str {
        let animation = &animations[0].sprites;
        let frame = (state.elapsed_time % animation.len() as u64) as usize;

        &animation[frame]
    }
}
