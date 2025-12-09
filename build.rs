fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_path = std::path::Path::new(&manifest_dir).join("assets").join("icon.ico");
        
        // Only set icon if the file exists
        if icon_path.exists() {
            res.set_icon(icon_path.to_str().unwrap());
            // Ignore compilation errors for now
            let _ = res.compile();
        }
    }
}
