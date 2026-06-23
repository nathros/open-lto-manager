use dioxus::prelude::*;

pub fn fn_link(link: String) -> impl Fn(Event<MouseData>) {
    move |evt: MouseEvent| {
        evt.stop_propagation();
        use_navigator().push(link.to_owned());
    }
}

pub fn fn_link_follow(link: String) {
    use_navigator().push(link);
}
