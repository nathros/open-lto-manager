use enum_iterator::Sequence;

use super::css::Css;

#[derive(Sequence)]
pub enum Id {
    HeaderNotificationIcon,
    HeaderNotificationMenu,
    HeaderInfoIcon,
    HeaderInfoMenu,
    HeaderUserIcon,
    HeaderUserMenu,
    HeaderAccentIcon,
    HeaderAccentMenu,
    HeaderIconIcon,
    HeaderIconMenu,
    HeaderThemeIcon,
    HeaderThemeMenu,

    AccentPicker,

    ModalJob,
    ModalSandpitError,
    ModalSandpitWarning,
    ModalSandpitInfo,
    ModalSandpitSuccess,
}

impl Id {
    pub const fn as_str(&self) -> &str {
        match self {
            Id::HeaderNotificationIcon => "h-not_i",
            Id::HeaderNotificationMenu => "h-not_m",
            Id::HeaderInfoIcon => "h-info_i",
            Id::HeaderInfoMenu => "h-info_m",
            Id::HeaderUserIcon => "h-user_i",
            Id::HeaderUserMenu => "h-user_m",
            Id::HeaderAccentIcon => "h-acc_i",
            Id::HeaderAccentMenu => "h-acc_m",
            Id::HeaderIconIcon => "h-ico_i",
            Id::HeaderIconMenu => "h-ico_m",
            Id::HeaderThemeIcon => "h-the_i",
            Id::HeaderThemeMenu => "h-the_m",

            Id::AccentPicker => Css::ID_ACCENT_PICKER,

            Id::ModalJob => "modal-b_job",
            Id::ModalSandpitError => "modal-sandpit_e",
            Id::ModalSandpitWarning => "modal-sandpit_w",
            Id::ModalSandpitInfo => "modal-sandpit_i",
            Id::ModalSandpitSuccess => "modal-sandpit_s",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use enum_iterator::all;

    use super::Id;

    #[test]
    fn check_duplicates() {
        let enums = all::<Id>().collect::<Vec<Id>>();

        let all_str_vec: Vec<&str> = enums.iter().map(|f| f.as_str()).collect();
        let all_str_map: HashSet<&str> = HashSet::from_iter(all_str_vec.iter().cloned());

        assert!(all_str_map.len() == all_str_vec.len(), "Duplicate id found");
    }
}
