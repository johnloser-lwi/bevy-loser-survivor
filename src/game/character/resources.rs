use bevy::prelude::*;


#[derive(Resource)]
pub struct CharacterTextureAtlasLayout {
    pub handle: Handle<TextureAtlasLayout>
}

impl Default for CharacterTextureAtlasLayout {
    fn default() -> Self {
        CharacterTextureAtlasLayout {
            handle: Handle::default()
        }
    }
}