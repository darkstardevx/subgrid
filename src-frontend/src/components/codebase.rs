use leptos::prelude::*;
use crate::models::{CodeLanguage, CodeSnippet};

#[component]
pub fn Codebase() -> impl IntoView {
    let (selected_lang, set_selected_lang) = signal(CodeLanguage::Rust);
    let (live_code, set_live_code) = signal(String::new());
    let (api_filter, set_api_filter) = signal(String::new());
    
    // Snippets list
    let (snippets, set_snippets) = signal(vec![
        CodeSnippet {
            name: "arch-autopsy-core".to_string(),
            lang: CodeLanguage::Rust,
            description: "High-performance systems ledger scanner.".to_string(),
            api_docs: "fn audit_system_nodes(target_path: &Path) -> Result<SystemManifest, AutopsyError>;".to_string(),
            source_code: "use std::fs;\nfn main() { println!(\"🚀 Initializing...\"); }".to_string(),
        },
        CodeSnippet {
            name: "hadal-hud-render".to_string(),
            lang: CodeLanguage::Lua,
            description: "High-frequency dynamic interface telemetry loop.".to_string(),
            api_docs: "hadal.render_frame(hud_object, buffer_ptr)".to_string(),
            source_code: "local hud = require(\"hadal.core\")\nhud.render_frame(layout, 0x7FFF00)".to_string(),
        },
    ]);

    let (active_snippet_index, set_active_snippet_index) = signal(0);
    let (new_name, set_new_name) = signal(String::new());
    let (new_desc, set_new_desc) = signal(String::new());
    let (new_code, set_new_code) = signal(String::new());

    // Sync playground when snippet changes
    Effect::new(move |_| {
        let idx = active_snippet_index.get();
        if let Some(snip) = snippets.get().get(idx) {
            set_live_code.set(snip.source_code.clone());
        }
    });

    view! {
        <div class="animate-fadeIn space-y-6">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between border-b border-[#2d204d] pb-4 gap-4">
                <div>
                    <h2 class="text-2xl font-bold text-[#b829c2]">"Codebase Asset Explorer"</h2>
                </div>
                <div class="flex bg-[#161925] p-1 rounded border border-[#2d204d] font-mono text-xs">
                    <button class="px-3 py-1.5 rounded transition-all bg-[#b829c2] text-white" on:click=move |_| set_selected_lang.set(CodeLanguage::Rust)>"Rust"</button>
                    <button class="px-3 py-1.5 rounded transition-all text-purple-400" on:click=move |_| set_selected_lang.set(CodeLanguage::Lua)>"Lua"</button>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                // Left Column: Vault
                <div class="space-y-4 lg:col-span-1">
                    <div class="bg-[#161925] border border-[#2d204d] p-4 rounded shadow-md">
                        <h3 class="text-xs font-bold text-purple-400 uppercase mb-3">"📁 Snippet Vault"</h3>
                        <div class="space-y-2">
                            {move || snippets.get().iter().enumerate().map(|(index, snip)| {
                                let is_active = active_snippet_index.get() == index;
                                view! {
                                    <button on:click=move |_| set_active_snippet_index.set(index) 
                                            class=move || format!("w-full text-left p-3 rounded border font-mono {}", if is_active { "bg-[#1f2335] border-[#b829c2]" } else { "bg-[#0d0e15] border-[#2d204d]" })>
                                        <div class="text-xs font-bold text-[#00ffcc]">{snip.name.clone()}</div>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>

                // Middle: Playground
                <div class="lg:col-span-2 flex flex-col space-y-4">
                    <div class="bg-[#161925] border border-[#2d204d] rounded overflow-hidden flex-1 flex flex-col">
                        <textarea class="w-full h-[400px] bg-[#0d0e15] text-[#e2e8f0] p-4 font-mono focus:outline-none" 
                                  prop:value=live_code on:input=move |ev| set_live_code.set(event_target_value(&ev))/>
                        <button class="bg-[#b829c2] p-2 text-white font-bold" on:click=move |_| {
                            web_sys::window().unwrap().alert_with_message(&format!("Running: {}", live_code.get())).unwrap();
                        }>"RUN INTERACTIVE CODE"</button>
                    </div>
                </div>

                // Right: API Docs
                <div class="lg:col-span-1">
                    <div class="bg-[#161925] border border-[#2d204d] p-4 rounded">
                        <h4 class="text-xs font-bold text-[#00ffcc] mb-2">"Active Endpoint Spec"</h4>
                        <div class="text-[11px] text-purple-200 font-mono">
                            {move || snippets.get().get(active_snippet_index.get()).map(|s| s.api_docs.clone()).unwrap_or_default()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
