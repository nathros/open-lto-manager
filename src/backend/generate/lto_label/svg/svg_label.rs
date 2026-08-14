use crate::{
    backend::generate::lto_label::pdf::page::PDFPageConfig,
    shared::models::database::label_preset::model_label_preset::LabelOptions,
};

pub struct SvgLabel {
    buffer: String,
}

type GroupFn = Box<dyn FnMut(i32, &mut SvgLabel, &LabelOptions)>;

impl SvgLabel {
    pub fn new(options: &LabelOptions, page_config: &PDFPageConfig) -> Self {
        let view_box = if options.include_view_box {
            format!(
                "viewBox=\"0 0 {} {}\" ",
                page_config.label_width, page_config.label_height
            )
        } else {
            "".to_string()
        };
        Self {
            buffer: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}mm\" height=\"{}mm\" {}preserveAspectRatio=\"none\" font-family=\"{}\">\n",
                page_config.label_width, page_config.label_height, view_box, options.font
            ),
        }
    }

    pub fn result(&mut self) -> String {
        self.buffer.push_str("</svg>");
        self.buffer.to_owned()
    }

    pub fn append_group(
        &mut self,
        tab_index: i32,
        group: &str,
        options: &LabelOptions,
        mut function: GroupFn,
    ) {
        self.append_line(tab_index, format!("<{}>", group).as_str());
        function(tab_index + 1, self, options);
        self.append_line(tab_index, format!("</{}>", group).as_str());
    }

    pub fn append_line(&mut self, tab_index: i32, text: &str) {
        self.buffer
            .push_str((0..tab_index).map(|_| "	").collect::<String>().as_str());
        self.buffer.push_str(text);
        self.buffer.push('\n');
    }
}
