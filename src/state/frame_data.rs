use log::debug;

// Frames that are currently supported by the program
pub static SUPPORTED_FRAMES: [FrameData; 4] = [
    FrameData {
        name: "Title",
        id: "TIT2",
        description: "Title",
    },
    FrameData {
        name: "Album",
        id: "TALB",
        description: "Album",
    },
    FrameData {
        name: "Artist",
        id: "TPE1",
        description: "Artist",
    },
    FrameData {
        name: "Track",
        id: "TRCK",
        description: "Track number",
    },
];

#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    pub name: &'static str,
    pub id: &'static str,
    pub description: &'static str,
}

pub fn id_to_name(id: &str) -> Result<String, String> {
    debug!("id_to_name: {}", id);
    match id {
        "TIT2" => Ok("Title".to_string()),
        "TALB" => Ok("Album".to_string()),
        "TPE1" => Ok("Artist".to_string()),
        "TRCK" => Ok("Track".to_string()),
        _ => Err("Frame not supported".to_string()),
    }
}
