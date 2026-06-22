use leptos::prelude::*;
use crate::api::{fetch_github_repos, fetch_aur_packages};

#[component]
pub fn Projects() -> impl IntoView {
    // This resource handles the async API call
    let gh_repos = LocalResource::new(move || fetch_github_repos("YOUR_GITHUB_USER".to_string()));

    view! {
        <div class="p-6">
            <h2 class="text-2xl font-bold text-[#ff007f]">"Active GitHub Upstreams"</h2>
            <Suspense fallback=move || view! { <p>"Loading repositories..."</p> }>
                {move || match gh_repos.get() {
                    Some(Ok(repos)) => view! {
                        <div class="grid gap-4 mt-4">
                            {repos.into_iter().map(|repo| view! {
                                <div class="bg-[#161925] p-4 border border-[#2d204d] rounded">
                                    <h3 class="text-[#00ffcc] font-bold">{repo.name}</h3>
                                    <p class="text-sm text-purple-200">{repo.description.unwrap_or_default()}</p>
                                </div>
                            }).collect_view()}
                        </div>
                    }.into_any(),
                    _ => view! { <p>"Failed to load."</p> }.into_any(),
                }}
            </Suspense>
        </div>
    }
}
