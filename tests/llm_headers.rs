use rustmem::{config::LlmConfig, extract::extract_facts};

#[tokio::test]
async fn rejects_invalid_custom_headers_without_exposing_their_values() {
    for provider in ["openai", "anthropic"] {
        for (name, value, expected) in [
            ("bad\r\nname", "secret-value", "Invalid LLM extra header name"),
            ("x-session", "secret-value\r\ninjected: yes", "Invalid LLM extra header value"),
        ] {
            let mut config = LlmConfig {
                provider: provider.into(),
                base_url: "http://127.0.0.1:0".into(),
                ..LlmConfig::default()
            };
            config.extra_headers.insert(name.into(), value.into());
            let error = extract_facts(&config, "I prefer local tools.").await.unwrap_err();
            assert_eq!(error.to_string(), expected);
            assert!(!format!("{error:#}").contains("secret-value"));
        }
    }
}
