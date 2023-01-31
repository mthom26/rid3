// Frames that are currently supported by the program
pub static SUPPORTED_FRAMES: [FrameData; 7] = [
    FrameData {
        name: "Content Group",
        id: "TIT1",
        description: "The 'Content group description' frame is used if the sound belongs to a \
            larger category of sounds/music. For example, classical music is often sorted in \
            different musical sections (e.g. \"Piano Concerto\", \"Weather - Hurricane\").",
    },
    FrameData {
        name: "Title",
        id: "TIT2",
        description: "The 'Title/Songname/Content description' frame is the actual name of \
            the piece (e.g. \"Adagio\", \"Hurricane Donna\").",
    },
    FrameData {
        name: "Album",
        id: "TALB",
        description: "The 'Album/Movie/Show title' frame is intended for the title of the \
            recording (or source of sound) from which the audio in the file is taken.",
    },
    FrameData {
        name: "Artist",
        id: "TPE1",
        description: "The 'Lead artist/Lead performer/Soloist/Performing group' is used for \
            the main artist.",
    },
    FrameData {
        name: "Track",
        id: "TRCK",
        description: "The 'Track number/Position in set' frame is a numeric string containing \
            the order number of the audio-file on its original recording. This MAY be extended \
            with a \"/\" character and a numeric string containing the total number of \
            tracks/elements on the original recording. E.g. \"4/9\".",
    },
    FrameData {
        name: "Date",
        id: "TDRC",
        description: "The 'Recording time' frame contains a timestamp describing when the audio \
            was recorded.",
    },
    FrameData {
        name: "User Defined Text",
        id: "TXXX",
        description: "This frame is intended for one-string text information concerning the \
            audio file in a similar way to the other \"T\"-frames. The frame body consists of \
            a description of the string, represented as a terminated string, followed by the \
            actual string. There may be more than one \"TXXX\" frame in each tag, but only one \
            with the same description.",
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

pub fn name_to_id(name: &str) -> Result<String, String> {
    match name {
        // TODO - Add all necessary names
        "title" => Ok("TIT2".to_string()),
        "track" => Ok("TRCK".to_string()),
        _ => Err("Name not recognised".to_string()),
    }
}
