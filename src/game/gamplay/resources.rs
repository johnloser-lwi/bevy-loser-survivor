use bevy::prelude::*;



#[derive(Resource)]
pub struct GameplayData {
    pub xp: u32,
    pub head_count: u32,
    pub level: u32,
    pub xp_to_next_level: u32
}

impl Default for GameplayData {
    fn default() -> Self {
        let mut data = GameplayData {
            xp: 0,
            head_count: 0,
            level: 1,
            xp_to_next_level: 0
        };
        data.set_xp_to_next_level();

        data
    }
}

impl GameplayData {
    pub fn set_xp_to_next_level(&self) -> u32 {
        self.xp + self.get_xp_offset_to_next_level()
    }

    pub fn get_xp_offset_to_next_level(&self) -> u32 {
        10 + (self.level - 1) * 5
    }
}
