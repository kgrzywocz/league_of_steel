extern crate piston_window;
use piston_window::*;

pub struct ImageWindow {
    window: PistonWindow,
    picture: G2dTexture,
}

impl ImageWindow {
    pub fn new(width: u32, height: u32, file: &str) -> Self {
        super::resolution::set_resolution(width, height);
        let mut window = Self::create_window(width, height);
        let picture = Self::create_picture(&mut window, file);
        window.set_lazy(false);

        let mut iw = Self { window, picture };
        iw.run_till_on_screen();

        super::resolution::refocus();
        iw.run_till_on_screen();
        iw
    }

    fn create_window(width: u32, height: u32) -> PistonWindow {
        WindowSettings::new("League of Legends (TM) Client", [width, height])
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .expect("Unable to create window")
    }
    fn create_picture(window: &mut PistonWindow, file: &str) -> G2dTexture {
        Texture::from_path(
            &mut window.create_texture_context(),
            file,
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Unable to open picture")
    }

    fn window_draw(&mut self, event: &piston_window::Event) {
        let picture = &self.picture;
        self.window.draw_2d(event, |context, graphics, _device| {
            image(picture, context.transform, graphics);
        });
    }
    fn run_till_on_screen(&mut self) {
        while let Some(e) = self.window.next() {
            self.window_draw(&e);
            if let piston_window::Event::Loop(Loop::AfterRender(_)) = e {
                break;
            }
        }
    }
}

pub struct ImageDisplay {
    handle: std::thread::JoinHandle<()>,
    stop_tx: std::sync::mpsc::Sender<()>,
}

impl ImageDisplay {
    pub fn new(width: u32, height: u32, file: &'static str) -> Self {
        let (start_tx, start_rx) = std::sync::mpsc::channel();
        let (stop_tx, stop_rx) = std::sync::mpsc::channel();

        let handle = std::thread::spawn(move || {
            let mut window = ImageWindow::new(width, height, file);
            start_tx.send("ready").unwrap();
            loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                window.run_till_on_screen();
            }
        });
        start_rx.recv().unwrap();
        Self { handle, stop_tx }
    }
    pub fn stop(self) {
        self.stop_tx.send(()).unwrap();
        self.handle.join().unwrap();
    }
}
