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
        window.set_lazy(true);

        let mut iw = Self { window, picture };
        iw.run_till_on_screen();
        
        super::resolution::refocus();
        iw.run_till_on_screen();
        iw
    }
    pub fn next(&mut self) -> Option<piston_window::Event> {
        self.window.next()
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

    pub fn window_draw(&mut self, event: &piston_window::Event) {
        let picture = &self.picture;
        self.window.draw_2d(event, |context, graphics, _device| {
            image(picture, context.transform, graphics);
        });
    }
    pub fn run_till_on_screen(&mut self) {
        while let Some(e) = self.window.next() {
            self.window_draw(&e);
            if let piston_window::Event::Loop(Loop::AfterRender(_)) = e {
                break;
            }
        }
    }
}
