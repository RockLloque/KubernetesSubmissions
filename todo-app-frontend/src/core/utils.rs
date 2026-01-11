use leptos::prelude::*;
use regex::Regex;

static URL_PATTERN: &str = r"(https?://[^\s<]+)";

/// Converts text containing URLs into HTML with clickable links
pub fn linkify(text: &str) -> View {
    let url_regex = Regex::new(URL_PATTERN).unwrap();

    let mut last_end = 0;
    let mut fragments = Vec::new();

    for capture in url_regex.captures_iter(text) {
        let match_obj = capture.get(0).unwrap();
        let start = match_obj.start();
        let end = match_obj.end();
        let url = match_obj.as_str();

        // Add text before the URL
        if start > last_end {
            let text_fragment = &text[last_end..start];
            fragments.push(view! { {text_fragment} }.into_any());
        }

        // Add the URL as a link
        fragments.push(view! {
            <a href={url} target="_blank" rel="noopener noreferrer">
                {url}
            </a>
        }.into_any());

        last_end = end;
    }

    // Add remaining text after the last URL
    if last_end < text.len() {
        let text_fragment = &text[last_end..];
        fragments.push(view! { {text_fragment} }.into_any());
    }

    // If no URLs were found, just return the original text
    if fragments.is_empty() {
        return view! { {text} }.into_any();
    }

    view! {
        {fragments}
    }.into_any()
}
