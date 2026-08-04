use std::sync::Arc;

use fontdb::Database;
use krilla::{
    Document,
    geom::{Size, Transform},
    page::PageSettings,
};
use krilla_svg::{SurfaceExt, SvgSettings};
use usvg::{Options, Tree};

use super::{page::PDFPageType, position::PDFLabelPosition};

pub fn generate_lto_label_pdf(labels_str: Vec<String>, page_type: PDFPageType) -> Vec<u8> {
    let mut fontdb = Database::new(); // Reusable font database from system
    fontdb.load_system_fonts();
    let opts = Options {
        fontdb: Arc::new(fontdb),
        ..Default::default()
    };

    // Parse and convert SVG Strings into parsed Tree
    let svg_trees: Vec<Tree> = labels_str
        .iter()
        .filter_map(|label_str| {
            Tree::from_data(label_str.as_bytes(), &opts).ok() // Invalid SVG Strings will be skipped
        })
        .collect();

    const SCALE: f32 = 1.0 + (1.0 / 3.0);
    let page_config = page_type.get_config();
    let page_settings =
        PageSettings::from_wh(page_config.width, page_config.height).unwrap_or_default();

    let mut document = Document::new(); // Create new document

    let mut page = document.start_page_with(page_settings.clone());
    let mut surface = page.surface();

    let mut position = PDFLabelPosition::new(page_config);

    for (index, label) in svg_trees.iter().enumerate() {
        //info!("w={}, h={}", label.size().width(), label.size().height());
        // For some reason SVG label.size() is 33.3% too big, looks fine in Inkscape
        if let Some(svg_size) =
            Size::from_wh(label.size().width() / SCALE, label.size().height() / SCALE)
        {
            surface.push_transform(&Transform::from_translate(position.x, position.y));
            surface.draw_svg(label, svg_size, SvgSettings::default());
            surface.pop();

            // Advance position and create new page if returns true, do not create new page for last label
            if position.next() && index + 1 != svg_trees.len() {
                surface.finish(); // Finish current page
                page.finish(); // Finish current page

                page = document.start_page_with(page_settings.clone());
                surface = page.surface();
            }
        }
    }

    // Finish document
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
