#[component]
fn App() -> impl IntoView {
    // 1. Define the signal
    let (active_page, set_active_page) = signal(SubgridPage::Dashboard);

    view! {
        <main class="min-h-screen bg-black text-white">
            // 2. Navigation bar: This consumes 'set_active_page'
            <nav>
                <button on:click=move |_| set_active_page.set(SubgridPage::Dashboard)>"Dashboard"</button>
                <button on:click=move |_| set_active_page.set(SubgridPage::Projects)>"Projects"</button>
            </nav>

            // 3. The Match Statement: This consumes 'active_page'
            // We use 'move ||' so the view knows it must re-run when the signal changes.
            <main class="max-w-7xl mx-auto px-4 py-8">
    {move || match active_page.get() {
        SubgridPage::Dashboard => view! { <Dashboard /> }.into_view(),
        SubgridPage::Codebase => view! { <CodebaseExplorer /> }.into_view(),
        SubgridPage::Projects => view! { <ProjectsPage /> }.into_view(),
        SubgridPage::Documentation => view! { <DocumentationPage /> }.into_view(),
    }}
</main>
    }
}
