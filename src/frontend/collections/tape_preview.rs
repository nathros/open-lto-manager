use dioxus::{dioxus_core, fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_generate_lto_label::generate_svg_label,
    frontend::assets::{IMG_TAPE_PREVIEW, IMG_TAPE_PREVIEW_TAB, LOGO_ASSET},
    shared::models::database::{
        model_manufacturer::RecordManufacturer, model_tape::RecordTape,
        model_tape_type::RecordTapeType,
    },
};

#[component]
pub fn TapePreview(
    preview: ReadSignal<RecordTape>,
    manufacturers: Vec<RecordManufacturer>,
    tapes_list: Vec<RecordTapeType>,
    designation: String,
    size: &'static str, // In rem
) -> Element {
    let manufacturer_name: String = manufacturers
        .iter()
        .find(|p| p.id == preview().manufacturer_id)
        .unwrap_or(&RecordManufacturer::blank())
        .name
        .to_lowercase();

    let tape_type: RecordTapeType = tapes_list
        .iter()
        .find(|p| p.id == preview().tape_type_id)
        .unwrap_or(&RecordTapeType::default())
        .clone();

    let svg_barcode: Loader<String> =
        use_loader(move || generate_svg_label(preview().barcode, designation.clone()))?;

    let hp = "hp"; // HP have their own colour scheme
    let main_class: String = match preview().worm {
        true => {
            if manufacturer_name == hp {
                tape_type.colour_worm_hp.clone()
            } else {
                tape_type.colour_worm_reg.clone()
            }
        }
        false => {
            if manufacturer_name == hp {
                tape_type.colour_hp.clone()
            } else {
                tape_type.colour_reg.clone()
            }
        }
    };

    let worm_show = match preview().worm {
        true => "show",
        false => "",
    };

    let (generation_str, ultrium) = if tape_type.generation == 0 {
        ("".to_string(), "")
    } else {
        (tape_type.generation.to_string(), "ultrium")
    };

    let (range_str, tape_size_str) = match tape_type.native_capacity {
        0 => ("", "".to_string()),
        1..=1000000000000 => ("GB", (tape_type.native_capacity / 1000000000).to_string()),
        _ => (
            "TB",
            ((tape_type.native_capacity as f64) / 1000000000000_f64).to_string(),
        ),
    };

    // Styles in: tape-preview.css
    rsx! {
        div {
            class: format!("tape-preview {}", main_class),
            "data-preview-size": size,

            img { class: "tape-main", src: IMG_TAPE_PREVIEW }
            img {
                class: format!("tape-worm {}", worm_show),
                src: IMG_TAPE_PREVIEW,
            }
            img { class: "tape-tab", src: IMG_TAPE_PREVIEW_TAB }
            div { class: "tape-overlay",
                img {
                    class: "tape-company",
                    src: format!("{}#{}", LOGO_ASSET, manufacturer_name),
                }
                div { class: "tape-ulto",
                    img { src: format!("{}#{}", LOGO_ASSET, ultrium) }
                    span { "{generation_str}" }
                }
            }
            div { class: "tape-size",
                "{tape_size_str}"
                div { "{range_str}" }
            }

            img { class: "tape-barcode", src: svg_barcode }
        }
    }
}
