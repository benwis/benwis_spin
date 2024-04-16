use leptos::*;

#[component]
pub fn PortfolioCard(
    img: String,
    href: String,
    heading: String,
    sub_heading: String,
    description: String,
) -> impl IntoView {
    view! {
        <a class="post-card" href=href>
            <img class="post-card__underlay" src=img alt=""/>
            <div class="post-card__layout">
                <h3 class="post-card__heading">{heading}</h3>
                <p class="post-card__meta">{description}</p>
            </div>
        </a>
    }
}
