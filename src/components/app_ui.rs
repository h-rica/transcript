#![allow(dead_code)]

use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::components::A;
use singlestage::{Badge, Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn ActionBar(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-wrap items-center gap-3">
            {children()}
        </div>
    }
}

#[component]
pub fn ActionButton(
    children: Children,
    on_click: Callback<MouseEvent>,
    #[prop(optional, into)] variant: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                let variant_class = match variant.get().unwrap_or_else(|| "primary".into()).as_str() {
                    "secondary" => "singlestage-btn-secondary",
                    "outline" => "singlestage-btn-outline",
                    "ghost" => "singlestage-btn-ghost",
                    "link" => "singlestage-btn-link",
                    "destructive" => "singlestage-btn-destructive",
                    _ => "singlestage-btn-primary",
                };
                let size_class = match size.get().unwrap_or_default().as_str() {
                    "sm" | "small" => "singlestage-btn-sm",
                    "lg" | "large" => "singlestage-btn-lg",
                    "icon" => "singlestage-btn-icon",
                    "sm-icon" | "icon-sm" => "singlestage-btn-sm-icon",
                    "lg-icon" | "icon-lg" => "singlestage-btn-lg-icon",
                    _ => "",
                };
                format!("{} {} {}", variant_class, size_class, class.get().unwrap_or_default())
            }
            disabled=move || disabled.get().unwrap_or(false)
            on:click=move |event| on_click.run(event)
            type="button"
        >
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton(
    children: Children,
    href: &'static str,
    #[prop(optional, into)] variant: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <A
            attr:class=move || {
                let variant_class = match variant.get().unwrap_or_else(|| "outline".into()).as_str() {
                    "secondary" => "singlestage-btn-secondary",
                    "ghost" => "singlestage-btn-ghost",
                    "link" => "singlestage-btn-link",
                    "primary" => "singlestage-btn-primary",
                    _ => "singlestage-btn-outline",
                };
                format!("{} {}", variant_class, class.get().unwrap_or_default())
            }
            href=href
        >
            {children()}
        </A>
    }
}

#[component]
pub fn MetricCard(
    label: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] description: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <Card class="h-full">
            <CardHeader>
                <CardDescription>{label}</CardDescription>
                <CardTitle>{move || value.get()}</CardTitle>
            </CardHeader>
            <Show when=move || description.get().is_some()>
                <CardContent class="pt-0 text-sm text-slate-600 dark:text-slate-300">
                    {move || description.get().unwrap_or_default()}
                </CardContent>
            </Show>
        </Card>
    }
}

#[component]
pub fn StatusBadge(#[prop(into)] value: Signal<String>, variant: &'static str) -> impl IntoView {
    view! {
        <Badge variant=variant>
            {move || value.get()}
        </Badge>
    }
}

#[component]
pub fn SpeakerPill(name: String) -> impl IntoView {
    view! {
        <Badge variant="outline">{name}</Badge>
    }
}
