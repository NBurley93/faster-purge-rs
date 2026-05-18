fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", "FASTER Purge");
        res.set("ProductName", "FASTER Purge");
        res.set("LegalCopyright", "Copyright 2026");
        // Uncomment once you have an icon:
        res.set_icon("icon.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
