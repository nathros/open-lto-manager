pub struct Svg {
    buffer: String,
}

type GroupFn = Box<dyn FnMut(i32, &mut Svg)>;

impl Svg {
    pub fn new() -> Self {
        let mut svg = Self {
            buffer: "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"80.5mm\" height=\"18.5mm\" viewBox=\"0 0 80.5 18.5\" preserveAspectRatio=\"none\">\n".to_string(),
        };
        svg.buffer.push_str("	<g transform=\"translate(1 1)\">\n");
        svg.buffer.push_str("		<rect width=\"78.5\" height=\"16.5\" x=\"0\" y=\"0\" rx=\"1\" ry=\"1\" fill=\"#fff\" stroke=\"#000\" stroke-width=\"0.035\" />\n");
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
