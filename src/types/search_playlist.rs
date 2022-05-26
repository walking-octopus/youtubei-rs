use super::{search_playlist_video::SearchPlaylistVideo, endpoints::EndpointBrowse};

pub struct SearchPlaylist{
    pub title:String,
    pub id: String,
    pub author: String,
    pub ucid:String,
    pub video_count: i64,
    pub videos: Vec<SearchPlaylistVideo>,
    pub thumbnail: String,
    pub author_verified: bool,
    pub play_endpoint: EndpointBrowse,
    pub browse_endpoint: EndpointBrowse,
}

impl SearchPlaylist {
    
}