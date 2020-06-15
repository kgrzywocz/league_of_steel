use miniview;

pub struct ImageWindow {
    controls: miniview::MiniView,
}

impl ImageWindow {
    pub fn new(width: u32, height: u32, file: &str) -> Self {
        super::resolution::set_resolution(width, height);

        let config = miniview::ConfigBuilder::from_path(file)
            .set_fullscreen(true)
            .build();
        let controls = miniview::MiniView::show(config).unwrap();

        Self { controls }
    }
    pub fn stop(self) {
        self.controls.close().unwrap();
    }
}
