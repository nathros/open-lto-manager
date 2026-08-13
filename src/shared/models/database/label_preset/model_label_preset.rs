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

use crate::shared::models::select_option::EnumStr;

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
    pub barcode_scale: f64,
    pub text_box_width: f64,
    pub text_box_height: f64,
    pub background_colour: Option<String>,
    pub page: PDFPageType,
    pub page_x_offset: f32,
    pub page_y_offset: f32,
    pub page_inner_x_gap: f32,
    pub page_inner_y_gap: f32,
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
            start_index: 1,
            quantity: 16,
            designation: "".to_string(),
            prefix: "P".to_string(),
            postfix: "S".to_string(),
            theme: LabelTheme::Standard,
            font: LabelFont::SansSerif,
            text_direction: LabelTextDirection::Normal,
            text_orientation: LabelTextOrientation::Normal,
            stroke_outer: 0.035, // 0.01 PostScript point
            stroke_inner: 0.035, // 0.01 PostScript point
            radius_outer: 0.0,
            radius_inner: 0.0,
            barcode_scale: 1.0,
            text_box_width: 10.0,
            text_box_height: 5.8,
            background_colour: Some("#FFF".to_string()),
            page: PDFPageType::A4,
            page_x_offset: 0.0,
            page_y_offset: 0.0,
            page_inner_x_gap: 0.0,
            page_inner_y_gap: 0.0,
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
    pub fn switch_page(&mut self, page: PDFPageType) {
        let default = LabelOptions::default();
        match page {
            PDFPageType::A4 | PDFPageType::Letter => {
                self.stroke_outer = default.stroke_outer;
                self.text_box_width = default.text_box_width;
            }
            PDFPageType::Avery3420 | PDFPageType::Herma4459 | PDFPageType::Herma4611 => {
                self.stroke_outer = 0.0;
                self.text_box_width = 9.95;
            }
            _ => {
                self.stroke_outer = 0.0;
                self.text_box_width = default.text_box_width;
            }
        }
        self.page = page;
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

impl From<i64> for LabelTheme {
    fn from(value: i64) -> Self {
        match value {
            _ if value == LabelTheme::Standard as i64 => LabelTheme::Standard,
            _ if value == LabelTheme::Warm as i64 => LabelTheme::Warm,
            _ if value == LabelTheme::Greyscale as i64 => LabelTheme::Greyscale,
            _ => LabelTheme::Standard,
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Sequence)]
pub enum LabelFont {
    SansSerif = 0,
    Serif = 1,
    Monospace = 2,
}

impl From<i64> for LabelFont {
    fn from(value: i64) -> Self {
        match value {
            _ if value == LabelFont::SansSerif as i64 => LabelFont::SansSerif,
            _ if value == LabelFont::Serif as i64 => LabelFont::Serif,
            _ if value == LabelFont::Monospace as i64 => LabelFont::Monospace,
            _ => LabelFont::SansSerif,
        }
    }
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Sequence)]
pub enum LabelTextDirection {
    Normal = 0,
    Reversed = 1,
}

impl From<i64> for LabelTextDirection {
    fn from(value: i64) -> Self {
        match value {
            _ if value == LabelTextDirection::Normal as i64 => LabelTextDirection::Normal,
            _ if value == LabelTextDirection::Reversed as i64 => LabelTextDirection::Reversed,
            _ => LabelTextDirection::Normal,
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Sequence)]
pub enum LabelTextOrientation {
    Normal = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
}

impl From<i64> for LabelTextOrientation {
    fn from(value: i64) -> Self {
        match value {
            _ if value == LabelTextOrientation::Normal as i64 => LabelTextOrientation::Normal,
            _ if value == LabelTextOrientation::Rotate90 as i64 => LabelTextOrientation::Rotate90,
            _ if value == LabelTextOrientation::Rotate180 as i64 => LabelTextOrientation::Rotate180,
            _ if value == LabelTextOrientation::Rotate270 as i64 => LabelTextOrientation::Rotate270,
            _ => LabelTextOrientation::Normal,
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Sequence, Clone, Copy)]
pub enum PDFPageType {
    A4 = 0,
    Avery3420 = 1,
    AveryL7162 = 2,
    Herma4459 = 3, // Clone of Avery3420
    Herma4611 = 4, // Clone of Avery3420
    Letter = 5,
    Avery5366 = 6,
    Avery6571_6577 = 7,
    Avery94214 = 8, // Clone of Avery6571_6577
    NetCllc749303_12301 = 9,
    OnlineLabelsOL173 = 10, // Clone of Avery6571_6577
    WorldLabelWl173 = 11,   // Clone of Avery6571_6577
}

impl EnumStr for PDFPageType {
    fn as_str(&self) -> &str {
        match self {
            PDFPageType::A4 => "(A4) Blank",
            PDFPageType::Avery3420 => "(A4) Avery 3420",
            PDFPageType::AveryL7162 => "(A4) Avery L7162",
            PDFPageType::Herma4459 => "(A4) Herma 4459",
            PDFPageType::Herma4611 => "(A4) Herma 4611",
            PDFPageType::Letter => "(Letter) Blank",
            PDFPageType::Avery5366 => "(Letter) Avery 5366",
            PDFPageType::Avery6571_6577 => "(Letter) Avery 6571/6577",
            PDFPageType::Avery94214 => "(Letter) Avery 94214",
            PDFPageType::NetCllc749303_12301 => "(Letter) NetC LLC #749303-12301",
            PDFPageType::OnlineLabelsOL173 => "(Letter) OnlineLabels OL173",
            PDFPageType::WorldLabelWl173 => "(Letter) WorldLabel WL-173",
        }
    }
}

impl From<i64> for PDFPageType {
    fn from(value: i64) -> Self {
        match value {
            _ if value == PDFPageType::A4 as i64 => PDFPageType::A4,
            _ if value == PDFPageType::Avery3420 as i64 => PDFPageType::Avery3420,
            _ if value == PDFPageType::AveryL7162 as i64 => PDFPageType::AveryL7162,
            _ if value == PDFPageType::Herma4459 as i64 => PDFPageType::Herma4459,
            _ if value == PDFPageType::Herma4611 as i64 => PDFPageType::Herma4611,
            _ if value == PDFPageType::Letter as i64 => PDFPageType::Letter,
            _ if value == PDFPageType::Avery5366 as i64 => PDFPageType::Avery5366,
            _ if value == PDFPageType::Avery6571_6577 as i64 => PDFPageType::Avery6571_6577,
            _ if value == PDFPageType::Avery94214 as i64 => PDFPageType::Avery94214,
            _ if value == PDFPageType::NetCllc749303_12301 as i64 => {
                PDFPageType::NetCllc749303_12301
            }
            _ if value == PDFPageType::OnlineLabelsOL173 as i64 => PDFPageType::OnlineLabelsOL173,
            _ if value == PDFPageType::WorldLabelWl173 as i64 => PDFPageType::WorldLabelWl173,
            _ => PDFPageType::A4,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use enum_iterator::{all, cardinality};
    use rusqlite::{
        ToSql,
        types::{FromSql, FromSqlResult, ValueRef},
    };

    use crate::shared::models::database::label_preset::model_label_preset::{
        LabelTextDirection, PDFPageType,
    };

    use super::{LabelOptions, LabelTheme};

    #[test]
    fn label_options_sql_deserialise() {
        {
            // Empty
            let default = LabelOptions::default();
            let json_str = r#"{}"#;
            let val_ref = ValueRef::Text(json_str.as_bytes());
            let options_result: FromSqlResult<LabelOptions> = FromSql::column_result(val_ref);
            //println!("{:?}", options_result);
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
                "start_index":1,
                "quantity":16,
                "designation":"",
                "prefix":"P",
                "postfix":"S",
                "theme":"Warm",
                "font":"SansSerif",
                "text_direction":"Normal",
                "text_orientation":"Normal",
                "stroke_outer":0.035,
                "stroke_inner":0.035,
                "radius_outer":0.0,
                "radius_inner":0.0,
                "barcode_scale":1.0,
                "text_box_width":10.0,
                "text_box_height":5.8,
                "background_colour":"#FFF",
                "page":"A4",
                "page_x_offset":0.0,
                "page_y_offset":0.0,
                "page_inner_x_gap":0.0,
                "page_inner_y_gap":0.0,
                "include_view_box":false
            }")
        )"##
        .to_string()
        .replace([' ', '\n'], "");

        assert_eq!(expected, sql_output_string);
    }

    #[test]
    fn pdf_page_type_from_i64() {
        let all_types: HashSet<PDFPageType> =
            HashSet::from_iter(all::<PDFPageType>().collect::<Vec<_>>());

        let check_types: HashSet<PDFPageType> = HashSet::from_iter(
            (0..cardinality::<PDFPageType>()).map(|i| PDFPageType::from(i as i64)),
        );

        assert_eq!(
            all_types, check_types,
            "impl From<i64> for PDFPageType {{}} does not cover all cases"
        );
    }
}
