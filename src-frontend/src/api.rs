use crate::models::NewsItem;

pub async fn fetch_arch_news() -> Vec<NewsItem> {
    // Simulated fetch: In production, call your backend proxy
    vec![
        NewsItem { title: "Kernel 6.13 Update", source: "Arch Linux", timestamp: "1h ago", link: "#" },
        NewsItem { title: "Plasma 6.2 Deployment", source: "Arch Linux", timestamp: "4h ago", link: "#" },
    ]
}

pub async fn fetch_aur_news() -> Vec<NewsItem> {
    vec![
        NewsItem { title: "AUR RPC Protocol Update", source: "AUR", timestamp: "2d ago", link: "#" },
    ]
}
