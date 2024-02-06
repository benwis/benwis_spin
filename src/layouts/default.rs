use crate::components::{Footer, Nav};
use crate::providers::color_scheme::ColorScheme;
use leptos::nonce::use_nonce;
use leptos::*;
use leptos_meta::*;
use leptos_use::ColorMode;

#[component]
pub fn Default(children: Children) -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>().expect("Failed to find ColorScheme");
    logging::log!("CS: {color_scheme:#?}");
    view! {
        <Html lang="en-US" class=move || {
            let classes = "h-full";
            let theme = match color_scheme.read.get() {
                ColorMode::Dark => "dark",
                ColorMode::Light => "",
                ColorMode::Auto => "",
                ColorMode::Custom(_) => "",
            };
            format!("{} {}", classes, theme)
        }/>
        <Body class="h-screen bg-white dark:bg-gray-900 max-w-7xl mx-auto flex flex-col"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link
            rel="alternate"
            type_="application/rss+xml"
            href="https://benw.is/rss.xml"
            title="benwis Blog"
        />

        // Fathom - beautiful, simple website analytics
        <script nonce={use_nonce()} src="https://cdn.usefathom.com/script.js" data-site="MTMCOBMG" defer></script>
        <Stylesheet id="leptos" href="/pkg/benwis-leptos.css"/>
        <Meta charset="utf-8"/>
        <Nav/>
        <main class="mx-auto flex w-full flex-col items-center justify-center border-gray-200 px-4 pb-16 md:pt-4 dark:border-gray-900 sm:px-8">
            {children()}
        </main>
        <Footer/>
    }
}
