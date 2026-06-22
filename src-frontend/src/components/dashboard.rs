use leptos::prelude::*;
use crate::models::NewsItem;
use crate::api::{fetch_arch_news, fetch_aur_news};

#[component]
pub fn Dashboard() -> impl IntoView {
    let arch_news = LocalResource::new(|| async { fetch_arch_news().await });
    let aur_news = LocalResource::new(|| async { fetch_aur_news().await });

    view! {
        <div class="space-y-6 p-6">
            // Ticker Bar
            <div class="bg-[#161925] border border-[#ff007f] p-2 overflow-hidden flex font-mono text-[10px] text-[#ff007f]">
                <span class="animate-marquee whitespace-nowrap">"// SYSTEM STATUS: ALL NODES ONLINE // SECURITY: OPTIMAL // UPTIME: 1440H // MEMORY: NOMINAL //"</span>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                // Column 1: Feeds
                <div class="space-y-4">
                    <FeedWidget title="ARCH NEWS" items=arch_news color="border-[#00ffcc]" />
                    <FeedWidget title="AUR UPDATES" items=aur_news color="border-[#ff007f]" />
                </div>
                
                // Column 2: Activity
                <div class="bg-[#161925] border border-[#2d204d] p-4 rounded h-full">
                    <h3 class="text-xs font-bold text-white mb-4">"LIVE GITHUB ACTIVITY"</h3>
                    <div class="space-y-2">
                         <div class="text-[11px] text-purple-400">"Push event: subgrid-core main"</div>
                         <div class="text-[11px] text-purple-400">"Pull request: #142 merged"</div>
                    </div>
                </div>

                // Column 3: System Vitals
                <div class="bg-[#161925] border border-[#2d204d] p-4 rounded flex flex-col items-center justify-center text-center">
                    <div class="text-4xl font-bold text-[#00ffcc] mb-2">"88%"</div>
                    <div class="text-[10px] text-purple-300 font-mono">"CPU LOAD"</div>
                    <div class="w-32 h-2 bg-[#0d0e15] mt-4 rounded-full border border-[#2d204d] overflow-hidden">
                        <div class="h-full bg-[#ff007f] w-[88%]"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeedWidget(title: &'static str, items: LocalResource<Vec<NewsItem>>, color: &'static str) -> impl IntoView {
    view! {
        <div class={format!("bg-[#161925] border-l-4 {} p-4", color)}>
            <h3 class="text-xs font-bold text-white mb-3">{title}</h3>
            <Suspense fallback=move || view! { <div class="text-[10px] text-purple-500">"Fetching..."</div> }>
                <div class="space-y-2">
                    {move || items.get().unwrap_or_default().into_iter().map(|item| view! {
                        <div class="text-[11px] text-purple-200 border-b border-[#2d204d] pb-1">
                            <span class="text-[#00ffcc]">"[" {item.timestamp} "]"</span> " " {item.title}
                        </div>
                    }).collect_view()}
                </div>
            </Suspense>
        </div>
    }
}
