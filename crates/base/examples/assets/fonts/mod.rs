use std::borrow::Cow;

pub(crate) const FAMILY: &str = "Pretendard";

pub(crate) fn register(cx: &gpui::App) {
    cx.text_system()
        .add_fonts(vec![
            Cow::Borrowed(include_bytes!("Pretendard-Regular.otf")),
            Cow::Borrowed(include_bytes!("Pretendard-Medium.otf")),
            Cow::Borrowed(include_bytes!("Pretendard-SemiBold.otf")),
            Cow::Borrowed(include_bytes!("Pretendard-Bold.otf")),
        ])
        .expect("register the bundled Pretendard fonts");
}
