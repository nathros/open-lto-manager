use dioxus::fullstack::serde::{Deserialize, Serialize};
use enum_iterator::Sequence;
#[cfg(feature = "server")]
use rusqlite::{
    ToSql, ffi,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use std::fmt;
#[cfg(feature = "server")]
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordLabelPreset {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub options: LabelOptions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct LabelOptions {
    pub start_index: usize,
    pub quantity: usize,
    pub designation: String,
    pub prefix: String,
    pub postfix: String,
    pub theme: LabelTheme,
    pub font: LabelFont,
    pub text_direction: LabelTextDirection,
    pub text_orientation: LabelTextOrientation,
    pub stroke_outer: f64, // Units in millimeter
    pub stroke_inner: f64, // Units in millimeter
    pub radius_outer: f64,
    pub radius_inner: f64,
    pub width: f64,  // Units in millimeter
    pub height: f64, // Units in millimeter
    pub barcode_scale: f64,
    pub text_box_width: f64,
    pub text_box_height: f64,
    pub background_colour: Option<String>,
    pub page: PDFPageType,
    pub include_view_box: bool,
}

impl fmt::Display for LabelOptions {
    // this is the function signature the fmt::display trait is looking for.
    // https://doc.rust-lang.org/std/fmt/trait.Display.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use write! macro just like println! macro, but output gets written to
        // the formatter struct.
        write!(f, "{:?}", self)
    }
}

impl Default for LabelOptions {
    fn default() -> Self {
        Self {
            start_index: 0,
            quantity: 5,
            designation: "".to_string(),
            prefix: "".to_string(),
            postfix: "".to_string(),
            theme: LabelTheme::Standard,
            font: LabelFont::SansSerif,
            text_direction: LabelTextDirection::Normal,
            text_orientation: LabelTextOrientation::Normal,
            stroke_outer: 0.035, // 0.01 PostScript point
            stroke_inner: 0.035, // 0.01 PostScript point
            radius_outer: 0.0,
            radius_inner: 0.0,
            width: 80.5,
            height: 18.5,
            barcode_scale: 1.0,
            text_box_width: 10.0_f64,
            text_box_height: 5.8_f64,
            background_colour: Some("#FFF".to_string()),
            page: PDFPageType::A4,
            include_view_box: false,
        }
    }
}

impl LabelOptions {
    pub fn default_preview() -> LabelOptions {
        LabelOptions {
            radius_outer: 1.0,
            include_view_box: true,
            ..Default::default()
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for LabelOptions {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match serde_json::to_string(self) {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(rusqlite::Error::SqliteFailure(
                ffi::Error::new(e.column() as i32),
                Some(format!("{}", e)),
            )),
        }
    }
}

#[cfg(feature = "server")]
impl FromSql for LabelOptions {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str() {
            Ok(str) => match serde_json::from_str::<LabelOptions>(str) {
                Ok(result) => FromSqlResult::Ok(result),
                Err(e) => FromSqlResult::Err(FromSqlError::OutOfRange(e.column() as i64)),
            },
            Err(e) => FromSqlResult::Err(e),
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq, Hash, Sequence)]
pub enum LabelTheme {
    Standard = 0,
    Warm = 1,
    Greyscale = 2,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum LabelFont {
    SansSerif = 0,
    Serif = 1,
    Monospace = 2,
}

#[cfg(feature = "server")]
impl Display for LabelFont {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            LabelFont::SansSerif => write!(formatter, "sans-serif"),
            LabelFont::Serif => write!(formatter, "serif"),
            LabelFont::Monospace => write!(formatter, "monospace"),
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum LabelTextDirection {
    Normal = 0,
    Reversed = 1,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum LabelTextOrientation {
    Normal = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Sequence, Clone, Copy)]
pub enum PDFPageType {
    A4 = 0,
    Letter = 1,
}

#[cfg(test)]
mod tests {
    use rusqlite::{
        ToSql,
        types::{FromSql, FromSqlResult, ValueRef},
    };

    use crate::shared::models::database::label_preset::model_label_preset::LabelTextDirection;

    use super::{LabelOptions, LabelTheme};

    #[test]
    fn label_options_sql_deserialise() {
        {
            // Empty
            let default = LabelOptions::default();
            let json_str = r#"{}"#;
            let val_ref = ValueRef::Text(json_str.as_bytes());
            let options_result: FromSqlResult<LabelOptions> = FromSql::column_result(val_ref);
            println!("{:?}", options_result);
            assert!(options_result.is_ok(), "Failed to deserialise");
            assert_eq!(
                default,
                options_result.unwrap(),
                "Expected empty to be default"
            );
        }
        {
            // Partial
            let check = LabelOptions {
                text_direction: LabelTextDirection::Reversed,
                ..Default::default()
            };
            let json_str = r#"{"text_direction":"Reversed"}"#;
            let val_ref = ValueRef::Text(json_str.as_bytes());
            let options_result: FromSqlResult<LabelOptions> = FromSql::column_result(val_ref);
            println!("{:?}", options_result);
            assert!(options_result.is_ok(), "Failed to deserialise");
            assert_eq!(check, options_result.unwrap(), "Expected to be the same");
        }
    }

    #[test]
    fn label_options_sql_serialise() {
        let reference = LabelOptions {
            theme: LabelTheme::Warm,
            ..Default::default()
        };
        let sql_result = reference.to_sql();
        assert!(sql_result.is_ok(), "Serialise failure");
        let sql_output = sql_result.unwrap();

        let sql_output_string = format!("{:?}", sql_output).replace("\\", "");
        let expected = r##"
        Owned(
            Text("{
                "start_index":0,
                "quantity":5,
                "designation":"",
                "prefix":"",
                "postfix":"",
                "theme":"Warm",
                "font":"SansSerif",
                "text_direction":"Normal",
                "text_orientation":"Normal",
                "stroke_outer":0.035,
                "stroke_inner":0.035,
                "radius_outer":0.0,
                "radius_inner":0.0,
                "width":80.5,
                "height":18.5,
                "barcode_scale":1.0,
                "text_box_width":10.0,
                "text_box_height":5.8,
                "background_colour":"#FFF",
                "page":"A4",
                "include_view_box":false
            }")
        )"##
        .to_string()
        .replace([' ', '\n'], "");

        assert_eq!(expected, sql_output_string);
    }
}
