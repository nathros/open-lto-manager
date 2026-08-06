use crate::frontend::css::Css;

#[derive(PartialEq, Eq, Clone)]
pub enum Size {
    Lg,
    Md,
    Sm,
}

impl Size {
    pub const fn to_class(&self) -> &str {
        match self {
            Size::Lg => Css::LG,
            Size::Md => Css::MD,
            Size::Sm => Css::SM,
        }
    }
}
