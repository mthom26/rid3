// Frames that are currently supported by the program
pub static SUPPORTED_FRAMES: [FrameData; 6] = [
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
    FrameData {
        name: "Date",
        id: "TDRC",
        description: "Recording Time",
    },
    FrameData {
        name: "User Defined Text",
        id: "TXXX",
        description: "User defined text frame",
    },
];

#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    pub name: &'static str,
    pub id: &'static str,
    pub description: &'static str,
}

pub fn id_to_name(id: &str) -> Result<String, String> {
    // debug!("id_to_name: {}", id);
    match id {
        "TIT2" => Ok("Title".to_string()),
        "TALB" => Ok("Album".to_string()),
        "TPE1" => Ok("Artist".to_string()),
        "TRCK" => Ok("Track".to_string()),
        "TDRC" => Ok("Date".to_string()),
        "TXXX" => Ok("User Defined Text".to_string()),
        _ => Err("Frame not supported".to_string()),
    }
}
