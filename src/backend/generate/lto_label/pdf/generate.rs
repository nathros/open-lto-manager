use std::sync::Arc;

use fontdb::Database;
use krilla::{
    Document,
    geom::{Size, Transform},
    page::PageSettings,
};
use krilla_svg::{SurfaceExt, SvgSettings};
use tracing::error;
use usvg::{Options, Tree};

use crate::{
    backend::generate::lto_label::svg::generate::generate_lto_label_svg_pages,
    shared::models::database::label_preset::model_label_preset::{LabelOptions, PDFPageType},
};

const SANS_SERIF: &str = "Lato";
const MONOSPACE: &str = "JetBrains Mono";
const SERIF: &str = "Noto Serif";

pub fn generate_lto_label_pdf_options(options: LabelOptions) -> Vec<u8> {
    let svg_pages = generate_lto_label_svg_pages(&options);
    generate_lto_label_pdf(svg_pages, options.page)
}

pub fn generate_lto_label_pdf(pages_str: Vec<String>, page_type: PDFPageType) -> Vec<u8> {
    let fontdb = match get_font_db() {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to load PDF fonts {}", e);
            let mut db = Database::new();
            db.load_system_fonts(); // Fallback to system
            db
        }
    };

    let opts = Options {
        fontdb: Arc::new(fontdb),
        ..Default::default()
    };

    // Parse and convert SVG Strings into parsed Tree
    let svg_trees: Vec<Tree> = pages_str
        .iter()
        .filter_map(|label_str| {
            Tree::from_data(label_str.as_bytes(), &opts).ok() // Invalid SVG Strings will be skipped
        })
        .collect();

    const SCALE: f32 = 1.0 + (1.0 / 3.0); // For some reason SVG label.size() is 33.3% too big, looks fine in Inkscape
    let page_config = page_type.get_config();
    let page_settings =
        PageSettings::from_wh(page_config.width_pt, page_config.height_pt).unwrap_or_default();

    let mut document = Document::new(); // Create new document

    let mut page = document.start_page_with(page_settings.clone());
    let mut surface = page.surface();

    for (index, page_tree) in svg_trees.iter().enumerate() {
        if let Some(svg_size) = Size::from_wh(
            page_tree.size().width() / SCALE,
            page_tree.size().height() / SCALE,
        ) {
            surface.push_transform(&Transform::from_translate(0_f32, 0_f32));
            surface.draw_svg(page_tree, svg_size, SvgSettings::default());
            surface.pop();

            // Do not create new page after last item
            if index + 1 != svg_trees.len() {
                surface.finish(); // Finish current page
                page.finish(); // Finish current page
                page = document.start_page_with(page_settings.clone());
                surface = page.surface();
            }
        }
    }

    surface.finish();
    page.finish();
    document.finish().unwrap_or_default()

    /*if let Ok(pdf) = document.finish()
        && let Ok(path) = std::path::absolute("test.pdf")
    {
        let _ = std::fs::write(path, &pdf);
        return pdf;
    }
    vec![]*/
}

fn get_font_db() -> Result<Database, std::io::Error> {
    let mut fontdb = Database::new();
    fontdb.load_font_file("assets/font/lato-v25-normal-400.ttf")?;
    fontdb.set_sans_serif_family(SANS_SERIF);

    fontdb.load_font_file("assets/font/jetbrains-mono-v24-normal-100-800.ttf")?;
    fontdb.set_monospace_family(MONOSPACE);

    // Originally selected Source Serif 4 but as name contains number cannot use
    // https://github.com/linebender/resvg/issues/804
    fontdb.load_font_file("assets/font/noto-serif-v33-normal-100-900.ttf")?;
    fontdb.set_serif_family(SERIF);

    Ok(fontdb)
}

#[cfg(test)]
pub mod tests {
    use fontdb::{Family, Weight};

    use crate::backend::generate::lto_label::pdf::generate::{
        MONOSPACE, SANS_SERIF, SERIF, get_font_db,
    };

    #[test]
    fn get_fonts() {
        let result = get_font_db();
        assert!(result.is_ok());
        let fontdb = result.unwrap();
        assert_eq!(fontdb.len(), 3); // Sans serif, Serif and Monospace

        assert_eq!(fontdb.family_name(&Family::SansSerif), SANS_SERIF);
        assert_eq!(fontdb.family_name(&Family::Monospace), MONOSPACE);
        assert_eq!(fontdb.family_name(&Family::Serif), SERIF);

        for face_info in fontdb.faces() {
            assert_eq!(face_info.weight, Weight::default());

            for family in face_info.families.iter().as_ref() {
                match family.0.as_str() {
                    SANS_SERIF => {
                        assert!(!face_info.monospaced);
                    }
                    MONOSPACE => {
                        assert!(face_info.monospaced);
                    }
                    SERIF => {
                        assert!(!face_info.monospaced);
                    }
                    _ => unreachable!("Unexpected Font"),
                }
            }
        }
    }
}
