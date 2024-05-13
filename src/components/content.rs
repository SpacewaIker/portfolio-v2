use crate::{components::NotFound, data_loading::get_content, AppLanguage};
use leptos::{
    component, create_memo, use_context, view, Await, IntoView, SignalGet, SignalGetUntracked,
};
use leptos_router::{use_params_map, A};
use std::borrow::ToOwned;

mod content_parts;

#[allow(clippy::wildcard_imports)]
use content_parts::*;

/// Component for showing a detailed view of a piece of content
///
/// This is intended as a full-page view
#[component]
pub fn ContentDetailsView(directory: String) -> impl IntoView {
    let params = use_params_map().get_untracked();

    let id = params.get("id");

    if id.is_none() {
        return NotFound().into_view();
    }

    let id = id.unwrap().clone();

    view! {
        <Await
            future=move || get_content(format!("{directory}/{id}"))
            let:content
        >
            <ContentDetailsViewInner content=content.clone() />
        </Await>
    }
    .into_view()
}

#[component]
fn ContentDetailsViewInner(content: toml::Table) -> impl IntoView {
    let lang = use_context::<AppLanguage>().expect("No context found!").0;

    let content = create_memo(move |_| content.get(&lang.get()).unwrap().clone());

    let title = move || {
        content
            .get()
            .get("title")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let icon = move || {
        content.get()
        .get("icon")
        .map(|v| view! { <img src=v.as_str().unwrap().to_owned() class="relative w-1/2 left-1/4 -top-8 mb-4" /> })
    };

    let body_html = move || markdown::to_html(content.get().get("body").unwrap().as_str().unwrap());
    let body = view! { <div inner_html=body_html class="font-paragraph text-darkpurple text-lg styled-body" /> };

    view! {
        <div class="bg-beige h-fit min-h-screen p-10 pt-20">
            <h1 class="font-title text-4xl font-bold underline text-darkpurple inline-block mb-4">{title}</h1>
            <ContentLinkIcons links=content.get_untracked().get("links").map(ToOwned::to_owned) />
            <ContentDate date=content.get_untracked().get("date").map(ToOwned::to_owned) lang=lang />
            <ContentTags tags=move || content.get().get("tags").map(ToOwned::to_owned) />
            <div class="flex flex-row space-x-8">
                <div class="ml-8 basis-3/4">
                    <ContentResumeLines lines=move || content.get().get("resume_lines").map(ToOwned::to_owned) />
                    {body}
                </div>
                <div class="basis-1/4">
                    {icon}
                    <ContentImageGallery images=content.get_untracked().get("media").map(ToOwned::to_owned) show_all=true />
                </div>
            </div>
            <div class="mt-16">
                <ContentImageGalleryL images=content.get_untracked().get("media").map(ToOwned::to_owned) />
            </div>
        </div>
    }
}

/// Component for showing a summarized view of a piece of content
///
/// This is intended as a preview in a list
#[component]
pub fn ContentSummaryView(directory: String, id: String) -> impl IntoView {
    let directory2 = directory.clone();
    let id2 = id.clone();
    view! {
        <Await
            future=move || get_content(format!("{directory}/{id}"))
            let:content
        >
            <ContentSummaryViewInner directory=&directory2 id=&id2 content=content.clone() />
        </Await>
    }
}

#[component]
#[allow(clippy::needless_lifetimes)]
fn ContentSummaryViewInner<'a>(
    directory: &'a str,
    id: &'a str,
    content: toml::Table,
) -> impl IntoView {
    let lang = use_context::<AppLanguage>().expect("No context found!").0;

    let content = create_memo(move |_| content.get(&lang.get()).unwrap().clone());

    let title = move || {
        content
            .get()
            .get("title")
            .map(|v| v.as_str().unwrap_or("").to_owned())
    };

    let icon = move || {
        content.get()
        .get("icon")
        .map(|v| view! { <img src=v.as_str().unwrap().to_owned() class="relative w-1/2 left-1/4 -top-8 mb-4" /> })
    };

    let summary_html =
        move || markdown::to_html(content.get().get("summary").unwrap().as_str().unwrap());
    let summary = view! { <div inner_html=summary_html class="font-paragraph text-darkpurple text-lg styled-body" /> };

    view! {
        <A href=format!("/{directory}/{id}") class="block mt-28 mb-4 rounded-md hover:outline-purple hover:outline hover:outline-4">
            <div class="bg-beige p-4">
                <h1 class="font-title text-4xl font-bold underline text-darkpurple inline-block mb-4">{title}</h1>
                <ContentLinkIcons links=content.get_untracked().get("links").map(ToOwned::to_owned) />
                <ContentDate date=content.get_untracked().get("date").map(ToOwned::to_owned) lang=lang />
                <ContentTags tags=move || content.get().get("tags").map(ToOwned::to_owned) />
                <div class="flex flex-row space-x-8">
                    <div class="ml-8 basis-3/4">
                        <ContentResumeLines lines=move || content.get().get("resume_lines").map(ToOwned::to_owned) />
                        {summary}
                    </div>
                    <div class="basis-1/4">
                        {icon}
                        <ContentImageGallery images=content.get_untracked().get("media").map(ToOwned::to_owned) show_all=false />
                    </div>
                </div>
            </div>
        </A>
    }.into_view()
}
