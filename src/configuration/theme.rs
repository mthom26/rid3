use colorsys::Rgb;
use serde::{de::Visitor, Deserialize};
use tui::style::Color as TuiColor;

#[derive(Debug, Clone, Copy)]
pub struct Color(tui::style::Color);

impl Into<TuiColor> for Color {
    fn into(self) -> TuiColor {
        self.0
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("derp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("{}", v);
        match v {
            "Reset" => Ok(Color(TuiColor::Reset)),
            "Black" => Ok(Color(TuiColor::Black)),
            "Red" => Ok(Color(TuiColor::Red)),
            "Green" => Ok(Color(TuiColor::Green)),
            "Yellow" => Ok(Color(TuiColor::Yellow)),
            "Blue" => Ok(Color(TuiColor::Blue)),
            "Magenta" => Ok(Color(TuiColor::Magenta)),
            "Cyan" => Ok(Color(TuiColor::Cyan)),
            "Gray" => Ok(Color(TuiColor::Gray)),
            "DarkGray" => Ok(Color(TuiColor::DarkGray)),
            "LightRed" => Ok(Color(TuiColor::LightRed)),
            "LightGreen" => Ok(Color(TuiColor::LightGreen)),
            "LightYellow" => Ok(Color(TuiColor::LightYellow)),
            "LightBlue" => Ok(Color(TuiColor::LightBlue)),
            "LightMagenta" => Ok(Color(TuiColor::LightMagenta)),
            "LightCyan" => Ok(Color(TuiColor::LightCyan)),
            "White" => Ok(Color(TuiColor::White)),
            s => match Rgb::from_hex_str(s) {
                Ok(col) => {
                    let rgb: [u8; 3] = col.into();
                    Ok(Color(TuiColor::Rgb(rgb[0], rgb[1], rgb[2])))
                }
                Err(e) => Err(serde::de::Error::custom(e)),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub list_item_fg: Color,
    pub list_item_bg: Color,
    pub active_list_item_fg: Color,
    pub active_list_item_bg: Color,
    pub inactive_list_item_fg: Color,
    pub inactive_list_item_bg: Color,

    pub help_border: Color,

    pub log_error_fg: Color,
    pub log_info_fg: Color,
    pub log_trace_fg: Color,
    pub log_warn_fg: Color,
}
