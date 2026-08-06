use std::sync::Arc;

use fontdb::Database;
use krilla::{
    Document,
    geom::{Size, Transform},
    page::PageSettings,
};
use krilla_svg::{SurfaceExt, SvgSettings};
use usvg::{Options, Tree};

use crate::{
    backend::generate::lto_label::svg::generate::generate_lto_label_svg_pages,
    shared::models::database::label_preset::model_label_preset::{LabelOptions, PDFPageType},
};

pub fn generate_lto_label_pdf_options(options: LabelOptions) -> Vec<u8> {
    let svg_pages = generate_lto_label_svg_pages(&options);
    generate_lto_label_pdf(svg_pages, options.page)
}

pub fn generate_lto_label_pdf(pages_str: Vec<String>, page_type: PDFPageType) -> Vec<u8> {
    let mut fontdb = Database::new(); // Reusable font database from system
    fontdb.load_system_fonts();
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
