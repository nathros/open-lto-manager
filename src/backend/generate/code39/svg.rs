use super::options::LabelOptions;

pub struct SvgLabel {
    buffer: String,
}

type GroupFn = Box<dyn FnMut(i32, &mut SvgLabel)>;

impl SvgLabel {
    pub fn new(options: &LabelOptions) -> Self {
        let mut svg = Self {
            buffer: format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}mm\" height=\"{}mm\" viewBox=\"0 0 {} {}\" preserveAspectRatio=\"none\">\n",
                options.width, options.height, options.width, options.height
            ),
        };
        svg.buffer.push_str("	<g transform=\"translate(1 1)\">\n");
        svg.buffer.push_str(format!("		<rect width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"#fff\" stroke=\"#000\" stroke-width=\"{}\" />\n",
                                options.width - 2_f64, options.height - 2_f64, options.radius_outer, options.radius_outer, options.stroke_outer).as_str());
        svg.buffer.push_str("	</g>\n");
        svg
    }

    pub fn result(&mut self) -> String {
        self.buffer.push_str("</svg>");
        self.buffer.to_owned()
    }

    pub fn append_group(&mut self, tab_index: i32, group: &str, mut fun: GroupFn) {
        self.append_line(tab_index, format!("<{}>", group).as_str());
        fun(tab_index + 1, self);
        self.append_line(tab_index, format!("</{}>", group).as_str());
    }

    pub fn append_line(&mut self, tab_index: i32, text: &str) {
        self.buffer
            .push_str((0..tab_index).map(|_| "	").collect::<String>().as_str());
        self.buffer.push_str(text);
        self.buffer.push('\n');
    }
}
