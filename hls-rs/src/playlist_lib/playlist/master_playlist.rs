use super::{
    rendition::Rendition, session_data::SessionData, variant::Variant, BasicTagData,
    MediaOrMasterPlaylistTagData,
};

struct MasterPlaylist {
    basic_data: BasicTagData,
    media_or_master_tag_data: MediaOrMasterPlaylistTagData,
    variants: Vec<Variant>,
    renditions: Vec<Rendition>,
    session_data: Option<SessionData>,
}
