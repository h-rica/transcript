use leptos::prelude::*;
use singlestage::icon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppIcon {
    About,
    ChevronDown,
    ChevronLeft,
    Copy,
    Export,
    Home,
    Models,
    Privacy,
    Search,
    Settings,
    Transcription,
    Upload,
}

#[component]
pub fn UiIcon(
    icon_name: AppIcon,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let class_name = move || class.get().unwrap_or_else(|| "h-4 w-4".to_string());

    view! {
        {move || match icon_name {
            AppIcon::About => icon!(icondata::FiInfo, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::ChevronDown => icon!(icondata::FiChevronDown, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::ChevronLeft => icon!(icondata::FiChevronLeft, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Copy => icon!(icondata::FiCopy, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Export => icon!(icondata::FiDownload, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Home => icon!(icondata::FiHome, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Models => icon!(icondata::FiCpu, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Privacy => icon!(icondata::FiShield, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Search => icon!(icondata::FiSearch, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Settings => icon!(icondata::FiSettings, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Transcription => icon!(icondata::FiSliders, class=class_name(), stroke_width=1.75).into_any(),
            AppIcon::Upload => icon!(icondata::FiUpload, class=class_name(), stroke_width=1.75).into_any(),
        }}
    }
}
