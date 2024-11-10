use leptos::*;

pub struct AppConfig {
    pub leptos_options: LeptosOptions,
}

impl AppConfig {
    pub async fn new() -> std::io::Result<Self> {
        let conf = get_configuration(None).await.unwrap();
        Ok(Self {
            leptos_options: conf.leptos_options,
        })
    }
}