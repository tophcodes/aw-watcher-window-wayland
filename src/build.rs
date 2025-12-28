fn main() {
    #[cfg(feature = "wlr-protocols")]
    generate_protocols();
}

#[cfg(feature = "wlr-protocols")]
fn generate_protocols() {
    extern crate wayland_scanner;
    use wayland_scanner::{Side, generate_code};

    // Location of the xml file, relative to the `Cargo.toml`
    // (xmlsrc, outsrc)
    let protocols = vec!(
        ("./protocols/wlr-foreign-toplevel-management-unstable-v1.xml",
         "./src/protocols/wlr-foreign-toplevel-management.rs"),
        ("./protocols/idle.xml",
         "./src/protocols/idle.rs"),
        ("./protocols/ext-idle-notify-v1.xml",
         "./src/protocols/ext-idle-notify-v1.rs"),
    );

    // Create "./src/protocols" folder for generated bindings
    std::fs::create_dir_all("./src/protocols").unwrap();

    for protocol in protocols {
        let (xmlsrc, outsrc) = protocol;
        generate_code(
            xmlsrc, outsrc,
            Side::Client, // Replace by `Side::Server` for server-side code
        );
    }
}
