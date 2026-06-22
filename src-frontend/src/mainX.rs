#![allow(unused_variables)]
#![allow(dead_code)]

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use leptos::task::spawn_local;
use web_sys::HtmlElement;
use leptos::wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq)]
enum SubgridPage {
    Dashboard,
    Projects,
    Codebase,
    Documentation,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum CodeLanguage {
    Rust, Lua, Bash, 
    Python, JavaScript, TypeScript, 
    Java, CSharp, Cpp, C, Go, 
    Sql, Html, Css, Sass, Scss, 
    R, Kotlin, Asm,
}

#[derive(Clone, Debug, PartialEq)]
struct CodeSnippet {
    name: String,
    lang: CodeLanguage,
    description: String,
    api_docs: String,
    source_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GithubRepo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(rename = "URL")] 
    pub url: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string_or_int")]
    pub num_votes: u32,
    pub popularity: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AurRpcResponse {
    pub version: u32,
    pub r#type: String,
    pub resultcount: u32,
    pub results: Vec<AurPackage>,
}

fn deserialize_number_from_string_or_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Int(u32),
        Str(String),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::Int(i) => Ok(i),
        StringOrInt::Str(s) => s.parse::<u32>().map_err(serde::de::Error::custom),
    }
}

async fn fetch_github_repos(username: String) -> Result<Vec<GithubRepo>, String> {
    if username.is_empty() {
        return Ok(Vec::new());
    }
    let url = format!("https://api.github.com/users/{}/repos?sort=updated&per_page=5", username);
    let response = Request::get(&url).send().await.map_err(|e| format!("Network request failed: {e}"))?;
    if !response.ok() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }
    response.json::<Vec<GithubRepo>>().await.map_err(|e| format!("Failed to parse repository payload: {e}"))
}

async fn fetch_aur_packages(maintainer: String) -> Result<Vec<AurPackage>, String> {
    if maintainer.is_empty() {
        return Ok(Vec::new());
    }
    // Updated: Pointing to local backend proxy
    let url = format!("http://127.0.0.1:3000/api/aur/search?arg={}", maintainer);
    
    let response = Request::get(&url)
        .send().await.map_err(|e| format!("AUR API Request failed: {e}"))?;
    if !response.ok() {
        return Err(format!("AUR RPC engine proxy returned status code: {}", response.status()));
    }
    
    let data = response.json::<AurRpcResponse>().await.map_err(|e| format!("Failed to parse AUR JSON payload: {e}"))?;
    Ok(data.results)
}

async fn search_aur_packages(query: String) -> Result<Vec<AurPackage>, String> {
    if query.len() < 3 { return Ok(Vec::new()); }
    // Updated: Pointing to local backend proxy
    let url = format!("http://127.0.0.1:3000/api/aur/search?arg={}", query);
    
    let response = Request::get(&url)
        .send().await.map_err(|e| format!("AUR Search failed: {e}"))?;
    
    if !response.ok() {
        return Err(format!("AUR Search error: {}", response.status()));
    }
    
    let data = response.json::<AurRpcResponse>().await.map_err(|e| format!("Parse error: {e}"))?;
    Ok(data.results)
}

async fn compile_remote(code: String, lang: String) -> Result<String, String> {
    // Updated: Pointing to local backend
    let url = "http://127.0.0.1:3000/api/compile";
    let response = Request::post(url)
        .json(&serde_json::json!({ "code": code, "lang": lang }))
        .map_err(|e| format!("Serialization error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Compiler network error: {e}"))?;

    if !response.ok() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "No error details available".to_string());
        return Err(format!("Backend rejected request (Status {}): {}", status, error_body));
    }

    response.text().await.map_err(|e| format!("Failed to read compiler output: {e}"))
}

#[component]
pub fn App() -> impl IntoView {
    let (active_page, set_active_page) = signal(SubgridPage::Dashboard);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_lang, set_selected_lang) = signal(CodeLanguage::Rust);
    let (live_code, set_live_code) = signal(String::new());
    let (output_text, set_output_text) = signal(String::from("Ready to compile..."));
    let (selected_compiler, set_selected_compiler) = signal("rust-1.78".to_string());

    let (aur_search_query, set_aur_search_query) = signal(String::new());

    let (show_settings, set_show_settings) = signal(false);
    let (font_size, set_font_size) = signal(14);
    let (tab_width, set_tab_width) = signal(4);
    let (theme, set_theme) = signal("Midnight".to_string());
    let (line_numbers, set_line_numbers) = signal(true);
    let (word_wrap, set_word_wrap) = signal(true);
    let (auto_format, set_auto_format) = signal(false);

    let (snippets, set_snippets) = signal(vec![
        CodeSnippet { name: "hello-rust".to_string(), lang: CodeLanguage::Rust, description: "Basic console print.".to_string(), api_docs: "println! macro.".to_string(), source_code: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string() },
        CodeSnippet { name: "hello-lua".to_string(), lang: CodeLanguage::Lua, description: "Basic print.".to_string(), api_docs: "print function.".to_string(), source_code: "print(\"Hello, world!\")".to_string() },
        CodeSnippet { name: "hello-bash".to_string(), lang: CodeLanguage::Bash, description: "Basic echo.".to_string(), api_docs: "echo command.".to_string(), source_code: "echo \"Hello, world!\"".to_string() },
        CodeSnippet { name: "hello-python".to_string(), lang: CodeLanguage::Python, description: "Basic print.".to_string(), api_docs: "print function.".to_string(), source_code: "print(\"Hello, world!\")".to_string() },
        CodeSnippet { name: "hello-js".to_string(), lang: CodeLanguage::JavaScript, description: "Basic console log.".to_string(), api_docs: "console.log.".to_string(), source_code: "console.log(\"Hello, world!\");".to_string() },
        CodeSnippet { name: "hello-ts".to_string(), lang: CodeLanguage::TypeScript, description: "Typed console log.".to_string(), api_docs: "console.log.".to_string(), source_code: "console.log(\"Hello, world!\");".to_string() },
        CodeSnippet { name: "hello-java".to_string(), lang: CodeLanguage::Java, description: "Standard class structure.".to_string(), api_docs: "System.out.println.".to_string(), source_code: "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, world!\");\n    }\n}".to_string() },
        CodeSnippet { name: "hello-csharp".to_string(), lang: CodeLanguage::CSharp, description: "Console application.".to_string(), api_docs: "Console.WriteLine.".to_string(), source_code: "using System;\nclass Program {\n    static void Main() {\n        Console.WriteLine(\"Hello, world!\");\n    }\n}".to_string() },
        CodeSnippet { name: "hello-cpp".to_string(), lang: CodeLanguage::Cpp, description: "Standard iostream.".to_string(), api_docs: "std::cout.".to_string(), source_code: "#include <iostream>\nint main() {\n    std::cout << \"Hello, world!\" << std::endl;\n    return 0;\n}".to_string() },
        CodeSnippet { name: "hello-c".to_string(), lang: CodeLanguage::C, description: "Standard stdio.".to_string(), api_docs: "printf.".to_string(), source_code: "#include <stdio.h>\nint main() {\n    printf(\"Hello, world!\\n\");\n    return 0;\n}".to_string() },
        CodeSnippet { name: "hello-go".to_string(), lang: CodeLanguage::Go, description: "Basic fmt print.".to_string(), api_docs: "fmt.Println.".to_string(), source_code: "package main\nimport \"fmt\"\nfunc main() {\n    fmt.Println(\"Hello, world!\")\n}".to_string() },
        CodeSnippet { name: "hello-sql".to_string(), lang: CodeLanguage::Sql, description: "Simple select.".to_string(), api_docs: "SELECT statement.".to_string(), source_code: "SELECT 'Hello, world!';".to_string() },
        CodeSnippet { name: "hello-html".to_string(), lang: CodeLanguage::Html, description: "Basic tag.".to_string(), api_docs: "h1 tag.".to_string(), source_code: "<h1>Hello, world!</h1>".to_string() },
        CodeSnippet { name: "hello-css".to_string(), lang: CodeLanguage::Css, description: "Pseudo-element content.".to_string(), api_docs: "content property.".to_string(), source_code: "body::after {\n    content: \"Hello, world!\";\n}".to_string() },
        CodeSnippet { name: "hello-sass".to_string(), lang: CodeLanguage::Sass, description: "Pseudo-element content.".to_string(), api_docs: "content property.".to_string(), source_code: "body::after\n    content: \"Hello, world!\"".to_string() },
        CodeSnippet { name: "hello-scss".to_string(), lang: CodeLanguage::Scss, description: "Pseudo-element content.".to_string(), api_docs: "content property.".to_string(), source_code: "body::after {\n    content: \"Hello, world!\";\n}".to_string() },
        CodeSnippet { name: "hello-r".to_string(), lang: CodeLanguage::R, description: "Cat function.".to_string(), api_docs: "cat function.".to_string(), source_code: "cat(\"Hello, world!\\n\")".to_string() },
        CodeSnippet { name: "hello-kotlin".to_string(), lang: CodeLanguage::Kotlin, description: "Kotlin main.".to_string(), api_docs: "println function.".to_string(), source_code: "fun main() {\n    println(\"Hello, world!\")\n}".to_string() },
        CodeSnippet { name: "hello-asm".to_string(), lang: CodeLanguage::Asm, description: "x86_64 minimal.".to_string(), api_docs: "syscall write.".to_string(), source_code: "section .data\n    msg db 'Hello, world!', 0xa\nsection .text\n    global _start\n_start:\n    mov eax, 1\n    mov edi, 1\n    mov rsi, msg\n    mov edx, 14\n    syscall\n    mov eax, 60\n    xor edi, edi\n    syscall".to_string() },
        CodeSnippet {
            name: "subgrid-autopsy-core".to_string(),
            lang: CodeLanguage::Rust,
            description: "High-performance systems ledger scanner. Performs raw kernel memory maps and root filesystem auditing without triggering standard userspace hooks.".to_string(),
            api_docs: "fn audit_system_nodes(target_path: &Path) -> Result<SystemManifest, AutopsyError>;\n\n• Returns fully serialized system telemetry maps.\n• Throws AutopsyError::MemoryLeak if memory footprint constraints break.".to_string(),
            source_code: "use std::fs;\nuse std::path::Path;\n\nfn main() {\n    println!(\"🚀 Initializing subgrid-autopsy core engine...\");\n    let target = Path::new(\"/var/log\");\n    match audit_system_nodes(target) {\n        Ok(manifest) => println!(\"Audit Verified: {} segments mapped.\", manifest.len()),\n        Err(e) => eprintln!(\"CRITICAL SEGFAULT: {:?}\", e),\n    }\n}".to_string(),
        },
        CodeSnippet {
            name: "subgrid-hud-render".to_string(),
            lang: CodeLanguage::Lua,
            description: "High-frequency dynamic interface telemetry loop.\nUpdates terminal borders, workspaces, and pipeline statuses using raw text configurations.".to_string(),
            api_docs: "subgrid.render_frame(hud_object, buffer_ptr)\n\n• hud_object: Native layout configuration table.\n• buffer_ptr: Direct terminal frame index memory pointer.".to_string(),
            source_code: "local hud = require(\"subgrid.core\")\n\nfunction _render_hud_loop()\n    local layout = hud.get_active_layout()\n    layout.border_color = \"#ff007f\"\n    layout.glow_radius = 12\n    \n    -- Direct rendering draw invocation\n    hud.render_frame(layout, 0x7FFF00)\nend".to_string(),
        },
        CodeSnippet {
            name: "subgrid-hud-audio".to_string(),
            lang: CodeLanguage::Lua,
            description: "Terminal audio engine hooks.\nFeeds real-time VU-meter data directly down layout buffers to provide interactive decibel graphs.".to_string(),
            api_docs: "subgrid.get_audio_stream_db()\n\n• Returns an integer scale value ranging from -60 to 0.".to_string(),
            source_code: "local audio = require(\"subgrid.audio\")\n\nfunction get_vu_bar()\n    local db = audio.get_audio_stream_db()\n    if db > -10 then\n        return \"[=== CRITICAL HIGHS ===]\"\n    else\n        return \"[=== NORMAL OPERATION ===]\"\n    end\nend".to_string(),
        },
        CodeSnippet {
            name: "subgrid-flush-pipeline".to_string(),
            lang: CodeLanguage::Bash,
            description: "Low-overhead memory flush scripts.\nStrips down ghost processes, broken child nodes, and orphaned Docker sockets inside production rings.".to_string(),
            api_docs: "./flush.sh --hard-reset [--verbose]\n\n• Clears Linux kernel cache allocations.\n• Forcibly drops isolated docker bridge loops.".to_string(),
            source_code: "#!/usr/bin/env bash\nset -euo pipefail\n\necho \"⚡ Commencing hard memory architecture sweep...\"\nsudo sync && echo 3 |\nsudo tee /proc/sys/vm/drop_caches\n\n# Purge dead virtualization sockets\ndocker network prune --force\necho \"✅ System pipelines cleared completely.\"".to_string(),
        },
    ]);
    let (active_snippet_index, set_active_snippet_index) = signal(0);
    
    Effect::new(move |_| {
        let target_lang = selected_lang.get();
        if let Some(first_match_idx) = snippets.get().iter().position(|s| s.lang == target_lang) {
            set_active_snippet_index.set(first_match_idx);
        }
    });

    Effect::new(move |_| {
        let idx = active_snippet_index.get();
        let current = snippets.get();
        if let Some(snip) = current.get(idx) {
            set_live_code.set(snip.source_code.clone());
        }
    });

    let target_username = "darkstardevx";

    let gh_repos_resource = LocalResource::new(move || {
        let username = target_username.to_string();
        async move { fetch_github_repos(username).await }
    });
    let aur_packages_resource = LocalResource::new(move || {
        let username = target_username.to_string();
        async move { fetch_aur_packages(username).await }
    });
    let aur_search_resource = LocalResource::new(move || {
        let query = aur_search_query.get();
        async move { search_aur_packages(query).await }
    });

    view! {
        <div class="min-h-screen bg-[#0d0e15] text-[#e2e8f0] font-sans antialiased">
            {move || if show_settings.get() {
                view! {
                    <div class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-4">
                        <div class="bg-[#161925] border border-[#2d204d] p-8 rounded shadow-2xl w-[400px]">
                            <h2 class="text-xl font-bold text-[#ff007f] mb-6">"IDE CONFIGURATION"</h2>
                            <div class="space-y-4">
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Theme"</label>
                                <select class="bg-[#0d0e15] border border-[#2d204d] p-1 rounded text-xs" prop:value=theme on:change=move|e| set_theme.set(event_target_value(&e))>
                                    <option>"Midnight"</option><option>"Cyberpunk"</option><option>"Matrix"</option>
                                </select></div>
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Font Size"</label>
                                <input type="number" class="w-16 bg-[#0d0e15] border border-[#2d204d] p-1 rounded text-xs" prop:value=font_size on:input=move|e| set_font_size.set(event_target_value(&e).parse().unwrap_or(14))/></div>
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Tab Width"</label>
                                <input type="number" class="w-16 bg-[#0d0e15] border border-[#2d204d] p-1 rounded text-xs" prop:value=tab_width on:input=move|e| set_tab_width.set(event_target_value(&e).parse().unwrap_or(4))/></div>
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Line Numbers"</label><input type="checkbox" checked=line_numbers on:change=move|e| set_line_numbers.set(event_target_checked(&e))/></div>
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Word Wrap"</label><input type="checkbox" checked=word_wrap on:change=move|e| set_word_wrap.set(event_target_checked(&e))/></div>
                                <div class="flex justify-between items-center"><label class="text-xs text-purple-300">"Auto-Format"</label><input type="checkbox" checked=auto_format on:change=move|e| set_auto_format.set(event_target_checked(&e))/></div>
                            </div>
                            <button class="mt-8 w-full bg-[#ff007f] hover:bg-[#d6006a] py-2 text-sm font-bold rounded" on:click=move|_| set_show_settings.set(false)>"APPLY SETTINGS"</button>
                        </div>
                    </div>
                }.into_any()
            } else { view! {}.into_any() }}

            <header class="bg-[#161925] text-white border-b-4 border-[#ff007f] shadow-[0_4px_20px_rgba(255,0,127,0.15)]">
                <div class="max-w-7xl mx-auto px-4 py-3 flex flex-col md:flex-row items-center justify-between gap-4">
                    <div class="flex items-center space-x-3 cursor-pointer" on:click=move |_| set_active_page.set(SubgridPage::Dashboard)>
                        <span class="text-2xl font-bold tracking-wider text-[#ff007f] drop-shadow-[0_0_8px_#ff007f]">"⚡ SUBGRID"</span>
                        <span class="text-xs bg-[#b829c2] px-2 py-0.5 rounded text-white font-mono font-bold shadow-[0_0_5px_#b829c2]">"v0.1.0"</span>
                    </div>

                    <div class="w-full md:w-1/2 relative">
                        <input 
                            type="text" 
                            placeholder="Search projects, documentation, files..."
                            class="w-full bg-[#1f2335] text-white placeholder-purple-400 border border-[#44365d] rounded px-4 py-2 focus:outline-none focus:border-[#b829c2] font-mono text-sm transition-all"
                            prop:value=search_query
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                    </div>

                    <nav class="flex space-x-1 font-semibold text-sm">
                        <button class=move || format!("px-4 py-2 rounded transition-all {}", if active_page.get() == SubgridPage::Dashboard { "bg-[#b829c2] text-white shadow-[0_0_8px_#b829c2]" } else { "text-purple-300 hover:bg-[#1f2335] hover:text-[#ff007f]" }) on:click=move |_| set_active_page.set(SubgridPage::Dashboard)>"Dashboard"</button>
                        <button class=move || format!("px-4 py-2 rounded transition-all {}", if active_page.get() == SubgridPage::Projects { "bg-[#b829c2] text-white shadow-[0_0_8px_#b829c2]" } else { "text-purple-300 hover:bg-[#1f2335] hover:text-[#ff007f]" }) on:click=move |_| set_active_page.set(SubgridPage::Projects)>"Projects"</button>
                        <button class=move || format!("px-4 py-2 rounded transition-all {}", if active_page.get() == SubgridPage::Codebase { "bg-[#b829c2] text-white shadow-[0_0_8px_#b829c2]" } else { "text-purple-300 hover:bg-[#1f2335] hover:text-[#ff007f]" }) on:click=move |_| set_active_page.set(SubgridPage::Codebase)>"Codebase"</button>
                        <button class=move || format!("px-4 py-2 rounded transition-all {}", if active_page.get() == SubgridPage::Documentation { "bg-[#b829c2] text-white shadow-[0_0_8px_#b829c2]" } else { "text-purple-300 hover:bg-[#1f2335] hover:text-[#00ffcc]" }) on:click=move |_| set_active_page.set(SubgridPage::Documentation)>"Docs"</button>
                    </nav>
                </div>
            </header>

            <main class="max-w-7xl mx-auto px-4 py-8">
                {move || match active_page.get() {
                    SubgridPage::Dashboard => view! {
                        <div class="animate-fadeIn space-y-6">
                            <div class="bg-[#161925] border-l-4 border-[#00ffcc] p-4 mb-6 rounded-r">
                                <h2 class="text-lg font-bold text-[#00ffcc]">"Subgrid Interactive Control Panel Engine"</h2>
                                <p class="text-sm text-purple-200 font-mono">"Core tracking vectors fully operational. Monitoring local nodes."</p>
                            </div>
                            
                            <div class="bg-[#161925] border border-[#2d204d] p-5 rounded shadow-md">
                                <h3 class="text-md font-mono font-bold text-white mb-4 flex items-center gap-2">
                                    <span class="text-blue-400">"🔍"</span> "AUR Package Search"
                                </h3>
                                <input 
                                    type="text" 
                                    placeholder="Search AUR by package name..."
                                    class="w-full bg-[#0d0e15] border border-[#2d204d] p-2 rounded text-sm text-white mb-4 focus:border-[#00ffcc]"
                                    on:input=move |e| set_aur_search_query.set(event_target_value(&e))
                                />
                                <Suspense fallback=move || view! { <div class="text-purple-400 font-mono text-xs">"Searching..."</div> }>
                                    {move || match aur_search_resource.get().map(|w| (*w).clone()) {
                                        Some(Ok(packages)) => {
                                            view! {
                                                <div class="space-y-2 max-h-[300px] overflow-y-auto">
                                                    {packages.into_iter().map(|pkg| {
                                                        view! {
                                                            <div class="bg-[#0d0e15] border border-[#2d204d] p-3 rounded flex justify-between items-center text-xs">
                                                                <div class="flex flex-col">
                                                                    <span class="font-bold text-[#00ffcc]">{pkg.name}</span>
                                                                    <span class="text-purple-300">"v" {pkg.version}</span>
                                                                </div>
                                                            </div>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        },
                                        Some(Err(e)) => view! { <div class="text-[#ff007f] text-xs">"Error: " {e}</div> }.into_any(),
                                        None => view! { <div class="text-purple-400 text-xs">"Enter search term..."</div> }.into_any(),
                                    }}
                                </Suspense>
                            </div>
                        </div>
                    }.into_any().into_view(),
                    
                    SubgridPage::Projects => view! {
                        <div class="animate-fadeIn space-y-6">
                            <div class="flex items-center justify-between border-b border-[#2d204d] pb-4">
                                <div>
                                    <h2 class="text-2xl font-bold text-[#ff007f] drop-shadow-[0_0_8px_rgba(255,0,127,0.2)]">"Project Inventory Matrix"</h2>
                                    <p class="text-sm text-purple-300 font-mono mt-1">"Live asynchronous upstreams and infrastructure pipeline tracking"</p>
                                </div>
                                <div class="text-xs font-mono text-[#00ffcc] bg-[#161925] px-3 py-1.5 rounded border border-[#2d204d]">
                                    "TARGET NODE: darkstardevx"
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                                <div class="lg:col-span-2 space-y-6">
                                    <div class="bg-[#161925] border border-[#2d204d] p-5 rounded shadow-md">
                                        <h3 class="text-md font-mono font-bold text-white mb-3 flex items-center gap-2">
                                            <span class="text-green-400 animate-pulse">"●"</span> "Live GitHub Upstreams"
                                        </h3>
                                        <Suspense fallback=move || view! { <div class="text-purple-400 font-mono text-sm py-4 animate-pulse">"Querying upstream network nodes..."</div> }>
                                            {move || match gh_repos_resource.get().map(|wrapper| (*wrapper).clone()) {
                                                None => view! { <div class="text-purple-400 font-mono text-xs italic py-2">"Initializing data streams..."</div> }.into_any(),
                                                Some(Err(err)) => view! { <div class="text-[#ff007f] font-mono text-xs py-2">{format!("Connection Error: {}", err)}</div> }.into_any(),
                                                Some(Ok(repos)) => {
                                                    if repos.is_empty() {
                                                        view! { <div class="text-purple-400 font-mono text-xs py-2">"No open repositories returned."</div> }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="space-y-3">
                                                                {repos.into_iter().map(|repo| view! {
                                                                    <div class="bg-[#0d0e15] border border-[#2d204d] p-4 rounded flex flex-col sm:flex-row sm:items-center justify-between gap-2 hover:border-purple-500/40 transition-all">
                                                                        <div>
                                                                            <a href=repo.html_url target="_blank" class="text-sm font-mono font-bold text-[#00ffcc] hover:underline">{repo.name}</a>
                                                                            <div class="text-xs text-purple-300 font-mono mt-1 max-w-xl">
                                                                                {repo.description.unwrap_or_else(|| "No tracking layout description provided.".to_string())}
                                                                            </div>
                                                                        </div>
                                                                        <span class="text-[10px] font-mono text-purple-400 bg-[#161925] px-2 py-0.5 rounded border border-[#2d204d] self-start sm:self-center">
                                                                            {format!("Updated: {}", repo.updated_at.get(0..10).unwrap_or("Unknown"))}
                                                                        </span>
                                                                    </div>
                                                                }).collect_view()}
                                                            </div>
                                                        }.into_any()
                                                    }
                                                }
                                            }}
                                        </Suspense>
                                    </div>

                                    <div class="bg-[#161925] border border-[#2d204d] p-5 rounded shadow-md mt-6">
                                        <h3 class="text-md font-mono font-bold text-white mb-4 flex items-center gap-2">
                                            <span class="text-blue-400">"🔍"</span> "AUR Package Search"
                                        </h3>
                                        <input 
                                            type="text" 
                                            placeholder="Search AUR by package name..."
                                            class="w-full bg-[#0d0e15] border border-[#2d204d] p-2 rounded text-sm text-white mb-4 focus:border-[#00ffcc]"
                                            on:input=move |e| set_aur_search_query.set(event_target_value(&e))
                                        />
                                        <Suspense fallback=move || view! { <div class="text-purple-400 font-mono text-xs">"Searching..."</div> }>
                                            {move || match aur_search_resource.get().map(|w| (*w).clone()) {
                                                Some(Ok(packages)) => {
                                                    view! {
                                                        <div class="space-y-2 max-h-[300px] overflow-y-auto">
                                                            {packages.into_iter().map(|pkg| {
                                                                view! {
                                                                    <div class="bg-[#0d0e15] border border-[#2d204d] p-3 rounded flex justify-between items-center text-xs">
                                                                        <div class="flex flex-col">
                                                                            <span class="font-bold text-[#00ffcc]">{pkg.name}</span>
                                                                            <span class="text-purple-300">"v" {pkg.version}</span>
                                                                        </div>
                                                                        <div class="text-right text-[10px] text-purple-400">
                                                                            <div>"Votes: " {pkg.num_votes}</div>
                                                                            <div>"Pop: " {format!("{:.2}", pkg.popularity)}</div>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_any()
                                                },
                                                Some(Err(e)) => view! { <div class="text-[#ff007f] text-xs">"Error: " {e}</div> }.into_any(),
                                                None => view! { <div class="text-purple-400 text-xs">"Enter search term..."</div> }.into_any(),
                                            }}
                                        </Suspense>
                                    </div>
                                </div>
                                
                                <div class="bg-[#161925] border border-[#2d204d] p-5 rounded shadow-md flex flex-col justify-between h-fit">
                                    <div>
                                        <h3 class="text-md font-mono font-bold text-white mb-3 flex items-center gap-2">
                                            <span class="text-orange-400 animate-pulse">"▲"</span> "AUR Deployments"
                                        </h3>
                                        <Suspense fallback=move || view! { <div class="text-purple-400 font-mono text-xs py-4 animate-pulse">"Polling upstream AUR RPC metadata..."</div> }>
                                            {move || match aur_packages_resource.get().map(|wrapper| (*wrapper).clone()) {
                                                None => view! { <div class="text-purple-400 font-mono text-xs italic py-2">"Mapping index streams..."</div> }.into_any(),
                                                Some(Err(err)) => view! { <div class="text-[#ff007f] font-mono text-xs py-2">{format!("RPC Telemetry Error: {}", err)}</div> }.into_any(),
                                                Some(Ok(packages)) => {
                                                    if packages.is_empty() {
                                                        view! { <div class="text-purple-400 font-mono text-xs py-2">"No packages detected under maintainer identity."</div> }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="space-y-4">
                                                                {packages.into_iter().map(|pkg| {
                                                                    let aur_link = format!("https://aur.archlinux.org/packages/{}", pkg.name);
                                                                    view! {
                                                                        <div class="bg-[#0d0e15] border border-[#2d204d] p-4 rounded space-y-3 hover:border-orange-500/30 transition-all">
                                                                            <div class="flex items-center justify-between">
                                                                                <a href=aur_link target="_blank" class="text-sm font-mono font-bold text-purple-300 hover:text-orange-400 hover:underline">
                                                                                    {pkg.name}
                                                                                </a>
                                                                                <span class="text-xs bg-[#221c38] px-2 py-0.5 rounded text-orange-400 font-mono font-semibold">
                                                                                    {pkg.version}
                                                                                </span>
                                                                            </div>
                                                                            <p class="text-[11px] text-purple-200 font-mono leading-normal line-clamp-2">
                                                                                {pkg.description.unwrap_or_else(|| "No static AUR blueprint description provided.".to_string())}
                                                                            </p>
                                                                            <div class="flex gap-4 text-[10px] font-mono text-purple-400 pt-1">
                                                                                <span>"Votes: " {pkg.num_votes}</span>
                                                                                <span>"Popularity: " {format!("{:.2}", pkg.popularity)}</span>
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                }).collect_view()}
                                                            </div>
                                                        }.into_any()
                                                    }
                                                }
                                            }}
                                        </Suspense>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any().into_view(),

                    SubgridPage::Codebase => {
                        view! {
                            <div class="animate-fadeIn space-y-6">
                                <div class="border-b border-[#2d204d] pb-4">
                                    <h2 class="text-2xl font-bold text-[#b829c2] drop-shadow-[0_0_8px_rgba(184,41,194,0.2)]">"Codebase Asset Explorer"</h2>
                                    <p class="text-xs text-purple-300 font-mono mt-1">"Analyze static abstractions, verify dynamic API loops, and inject custom workspace sandboxes."</p>
                                </div>

                                <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                                    <div class="space-y-4 lg:col-span-1 flex flex-col gap-4">
                                        <div class="bg-[#161925] border border-[#2d204d] p-4 rounded shadow-md flex-1">
                                            <h3 class="text-xs font-bold font-mono tracking-wider text-purple-400 uppercase mb-3">"📁 Snippet Vault"</h3>
                                            <div class="space-y-2 max-h-[320px] overflow-y-auto pr-1">
                                                {move || snippets.get().iter().enumerate()
                                                    .filter(|(_, snip)| snip.lang == selected_lang.get())
                                                    .map(|(index, snip)| {
                                                        let is_active = active_snippet_index.get() == index;
                                                        view! {
                                                            <button on:click=move |_| set_active_snippet_index.set(index) class=move || format!("w-full text-left p-3 rounded border font-mono transition-all flex flex-col gap-1 {}", if is_active { "bg-[#1f2335] border-[#b829c2] shadow-[0_0_10px_rgba(184,41,194,0.15)]" } else { "bg-[#0d0e15] border-[#2d204d] hover:border-purple-500/40" })>
                                                                <span class=move || format!("text-xs font-bold {}", if is_active { "text-[#ff007f]" } else { "text-[#00ffcc]" })>{snip.name.clone()}</span>
                                                                <span class="text-[11px] text-purple-300 line-clamp-2 leading-normal">{snip.description.clone()}</span>
                                                            </button>
                                                        }
                                                    }).collect_view()
                                                }
                                            </div>
                                        </div>
                                    </div>

                                    <div class="lg:col-span-2 flex flex-col space-y-4">
                                        <div class="bg-[#161925] border border-[#2d204d] rounded shadow-lg overflow-hidden flex-1 flex flex-col min-h-[450px]">
                                            <div class="bg-[#1d2132] px-4 py-2 border-b border-[#2d204d] flex items-center justify-between font-mono text-xs">
                                                <div class="flex items-center space-x-2">
                                                    <span class="w-3 h-3 rounded-full bg-[#ff007f] animate-pulse"></span>
                                                    <span class="text-purple-200 font-bold">"LIVE PLAYGROUND LABS"</span>
                                                </div>
                                                
                                                <div class="flex items-center gap-2">
                                                    <button class="bg-[#2d204d] text-white px-3 py-1 rounded text-xs font-bold hover:bg-[#3d306d]" on:click=move|_| {
                                                        let code = live_code.get();
                                                        let ext = match selected_lang.get() {
                                                            CodeLanguage::Rust => "rs",
                                                            CodeLanguage::Lua => "lua",
                                                            CodeLanguage::Bash => "sh",
                                                            CodeLanguage::Python => "py",
                                                            CodeLanguage::JavaScript => "js",
                                                            CodeLanguage::TypeScript => "ts",
                                                            CodeLanguage::Java => "java",
                                                            CodeLanguage::CSharp => "cs",
                                                            CodeLanguage::Cpp => "cpp",
                                                            CodeLanguage::C => "c",
                                                            CodeLanguage::Go => "go",
                                                            CodeLanguage::Sql => "sql",
                                                            CodeLanguage::Html => "html",
                                                            CodeLanguage::Css => "css",
                                                            CodeLanguage::Sass => "sass",
                                                            CodeLanguage::Scss => "scss",
                                                            CodeLanguage::R => "r",
                                                            CodeLanguage::Kotlin => "kt",
                                                            CodeLanguage::Asm => "asm",
                                                        };
                                                        let filename = format!("main.{}", ext);

                                                        let win = web_sys::window().unwrap();
                                                        let doc = win.document().unwrap();
                                                        let a = doc.create_element("a").unwrap();
                                                        let _ = a.set_attribute("href", &format!("data:text/plain;charset=utf-8,{}", js_sys::encode_uri_component(&code)));
                                                        let _ = a.set_attribute("download", &filename);
                                                        let _ = a.unchecked_into::<HtmlElement>().click();
                                                    }>"⬇ Download"</button>
                                                    
                                                    <button class="text-purple-300 hover:text-white" title="Reset" on:click=move|_| {
                                                        if let Some(snip) = snippets.get().get(active_snippet_index.get()) {
                                                            set_live_code.set(snip.source_code.clone());
                                                        }
                                                    }>"↺"</button>

                                                    <button class="text-purple-300 hover:text-white" title="Settings" on:click=move|_| set_show_settings.set(true)>"⚙"</button>

                                                    <select 
                                                        class="bg-[#0d0e15] border border-[#2d204d] text-purple-300 p-1 rounded font-mono text-[10px] focus:outline-none focus:border-[#ff007f] hover:border-purple-500"
                                                        on:change=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            let lang = match val.as_str() {
                                                                "rust-1.78" => CodeLanguage::Rust,
                                                                "lua-5.4" => CodeLanguage::Lua,
                                                                "bash-5.0" => CodeLanguage::Bash,
                                                                "python-3.12" => CodeLanguage::Python,
                                                                "js-es6" => CodeLanguage::JavaScript,
                                                                "ts-5.0" => CodeLanguage::TypeScript,
                                                                "java-21" => CodeLanguage::Java,
                                                                "csharp-12" => CodeLanguage::CSharp,
                                                                "cpp-20" => CodeLanguage::Cpp,
                                                                "c-17" => CodeLanguage::C,
                                                                "go-1.22" => CodeLanguage::Go,
                                                                "sql-ansi" => CodeLanguage::Sql,
                                                                "html-5" => CodeLanguage::Html,
                                                                "css-3" => CodeLanguage::Css,
                                                                "sass" => CodeLanguage::Sass,
                                                                "scss" => CodeLanguage::Scss,
                                                                "r-4.3" => CodeLanguage::R,
                                                                "kotlin-1.9" => CodeLanguage::Kotlin,
                                                                "asm-x86" => CodeLanguage::Asm,
                                                                _ => CodeLanguage::Rust,
                                                            };
                                                            set_selected_lang.set(lang);
                                                            set_selected_compiler.set(val);
                                                        }
                                                    >
                                                        <option value="rust-1.78">"Rust 1.78"</option>
                                                        <option value="lua-5.4">"Lua 5.4"</option>
                                                        <option value="bash-5.0">"Bash 5.0"</option>
                                                        <option value="python-3.12">"Python 3.12"</option>
                                                        <option value="js-es6">"JavaScript"</option>
                                                        <option value="ts-5.0">"TypeScript"</option>
                                                        <option value="java-21">"Java 21"</option>
                                                        <option value="csharp-12">"C# (.NET 8)"</option>
                                                        <option value="cpp-20">"C++ 20"</option>
                                                        <option value="c-17">"C 17"</option>
                                                        <option value="go-1.22">"Go 1.22"</option>
                                                        <option value="sql-ansi">"SQL (ANSI)"</option>
                                                        <option value="html-5">"HTML 5"</option>
                                                        <option value="css-3">"CSS 3"</option>
                                                        <option value="sass">"SASS"</option>
                                                        <option value="scss">"SCSS"</option>
                                                        <option value="r-4.3">"R 4.3"</option>
                                                        <option value="kotlin-1.9">"Kotlin 1.9"</option>
                                                        <option value="asm-x86">"ASM (x86)"</option>
                                                    </select>
                                                    <button 
                                                        class="bg-[#ff007f] text-white px-3 py-1 rounded text-[10px] font-bold hover:bg-[#d6006a]" 
                                                        on:click=move |_| {
                                                            let code = live_code.get();
                                                            let compiler = selected_compiler.get();
                                                            spawn_local(async move {
                                                                set_output_text.set(format!("Compiling with {}...", compiler));
                                                                match compile_remote(code, compiler).await {
                                                                    Ok(res) => set_output_text.set(res),
                                                                    Err(e) => set_output_text.set(format!("Error: {}", e)),
                                                                }
                                                            });
                                                        }
                                                    >
                                                        "RUN"
                                                    </button>
                                                </div>
                                            </div>
                                            <div class="flex-1 relative font-mono text-sm p-1 bg-[#0d0e15]">
                                                <textarea class="w-full h-full min-h-[400px] bg-transparent text-[#e2e8f0] p-4 font-mono focus:outline-none resize-none leading-relaxed selection:bg-[#ff007f]/30" 
                                                    style=move|| format!("font-size: {}px; tab-size: {}; white-space: {};", font_size.get(), tab_width.get(), if word_wrap.get() { "pre-wrap" } else { "pre" })
                                                    prop:value=live_code on:input=move |ev| set_live_code.set(event_target_value(&ev)) spellcheck="false"/>
                                            </div>
                                        </div>
                                        
                                        <div class="bg-[#0d0e15] border border-[#2d204d] rounded overflow-hidden">
                                            <div class="bg-[#1d2132] px-4 py-2 border-b border-[#2d204d] font-mono text-xs text-[#00ffcc]">"STDOUT_TERMINAL"</div>
                                            <pre class="p-4 text-sm font-mono text-gray-300 whitespace-pre-wrap">{move || output_text.get()}</pre>
                                        </div>
                                    </div>

                                    <div class="space-y-4 lg:col-span-1">
                                        <div class="bg-[#161925] border border-[#2d204d] p-4 rounded shadow-md flex flex-col h-full">
                                            <h4 class="text-xs font-mono font-bold text-[#00ffcc] mb-1">"Active Endpoint Spec"</h4>
                                            <div class="bg-[#0d0e15] border border-[#2d204d] p-3 rounded font-mono text-[11px] text-purple-200 overflow-x-auto whitespace-pre-wrap leading-normal">
                                                {move || {
                                                    let idx = active_snippet_index.get();
                                                    let current = snippets.get();
                                                    current.get(idx).map(|s| s.api_docs.clone()).unwrap_or_else(|| "No docs mapped to active slot.".to_string())
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any().into_view()
                    },

                    SubgridPage::Documentation => view! {
                        <div class="animate-fadeIn space-y-4 h-[calc(100vh-12rem)] flex flex-col">
                            <div class="flex flex-col sm:flex-row sm:items-center justify-between border-b border-[#2d204d] pb-3 gap-2 shrink-0">
                                <h2 class="text-2xl font-bold text-[#00ffcc]">"Technical Reference Manual"</h2>
                            </div>
                            <div class="flex-1 bg-[#161925] border border-[#2d204d] rounded overflow-hidden shadow-2xl relative">
                                <iframe src="/book/index.html" class="w-full h-full border-none bg-[#161925]" title="Subgrid Technical Documentation Engine"/>
                            </div>
                        </div>
                    }.into_any().into_view(),
                }}
            </main>
        </div>
    }
}

fn main() {
    leptos::prelude::mount_to_body(App);
}
