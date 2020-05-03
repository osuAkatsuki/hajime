#[cfg(windows)]
extern crate winres;

fn main() {
    install_manifest();
}

fn install_manifest() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();

        res.set_icon("resources/icon.ico");
    
        res.compile().unwrap();
    }
}
