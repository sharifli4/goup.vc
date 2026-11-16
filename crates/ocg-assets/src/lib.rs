//! Static asset embedding and build metadata for OCG.

use rust_embed::Embed;

/// Static file embedder using `rust-embed`.
#[derive(Embed)]
#[folder = "../../ocg-server/dist/static"]
pub struct StaticFile;

/// Current application commit SHA embedded at build time.
pub const COMMIT_SHA: &str = env!("OCG_COMMIT_SHA");

#[cfg(test)]
mod tests {
    use super::StaticFile;

    #[test]
    fn embedded_styles_include_compiled_tailwind_utilities() {
        let styles_path = StaticFile::iter()
            .find(|path| {
                path.starts_with("css/styles.")
                    && !path.starts_with("css/styles.src.")
                    && path.ends_with(".css")
            })
            .expect("compiled hashed styles.css should be embedded");
        let styles = StaticFile::get(styles_path.as_ref()).expect("styles asset bytes");
        let css = std::str::from_utf8(styles.data.as_ref()).expect("styles should be UTF-8");

        assert!(
            css.contains(".btn-primary") && css.contains(".flex") && css.contains(".hidden"),
            "compiled CSS ({} bytes) is missing Tailwind utilities scanned from templates",
            css.len()
        );
    }
}
