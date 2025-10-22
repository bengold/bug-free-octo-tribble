use reqwest::blocking::Client;
use std::error::Error;
use url::Url;

pub struct WebFetcher {
    client: Client,
}

impl WebFetcher {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .user_agent("BrowserEngine/0.1")
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(WebFetcher { client })
    }

    /// Fetch HTML content from a URL
    pub fn fetch_html(&self, url: &str) -> Result<String, Box<dyn Error>> {
        println!("Fetching HTML from: {}", url);
        let response = self.client.get(url).send()?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        
        let html = response.text()?;
        Ok(html)
    }

    /// Fetch CSS content from a URL
    pub fn fetch_css(&self, url: &str) -> Result<String, Box<dyn Error>> {
        println!("Fetching CSS from: {}", url);
        let response = self.client.get(url).send()?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        
        let css = response.text()?;
        Ok(css)
    }

    /// Resolve a relative URL against a base URL
    pub fn resolve_url(base: &str, relative: &str) -> Result<String, Box<dyn Error>> {
        let base_url = Url::parse(base)?;
        let resolved = base_url.join(relative)?;
        Ok(resolved.to_string())
    }
}

/// Extract CSS links from HTML content
pub fn extract_css_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    
    // Simple extraction - look for <link rel="stylesheet" href="...">
    for line in html.lines() {
        if line.contains("rel=\"stylesheet\"") || line.contains("rel='stylesheet'") {
            if let Some(href_start) = line.find("href=\"").or_else(|| line.find("href='")) {
                let quote = if line[href_start..].starts_with("href=\"") { '"' } else { '\'' };
                let start = href_start + 6; // length of "href=\"" or "href='"
                if let Some(href_end) = line[start..].find(quote) {
                    let url = &line[start..start + href_end];
                    if url.ends_with(".css") {
                        links.push(url.to_string());
                    }
                }
            }
        }
    }
    
    links
}

