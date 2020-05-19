use backend_interface::*;

pub struct BarsPosition {
    hud_scale: f32,
}
impl BarsPosition {
    pub fn new() -> Self {
        Self { hud_scale: 1.0 }
    }
    pub fn get(&self, disp_mode: &BackendScreenResolution) -> BackendCaptureRect {
        let mut pos = BackendCaptureRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        //682x1031 1094x1065 on 1920x1080
        //455x688 729x709 on 1280x720
        //415x985 748x1011 on 1280x1024
        //729x1038 1071x1067 on 1920x1080 - hud 50% -> 0.83
        //776x1047 1049x1069 on 1920x1080 - hud 0%(0.01) -> 0.66
        //381x744 575x761 on 1024x768 - hud 0%(0.01) -> 0.66
        //245x572 474x591 on 800x600
        //297x581 449x594 on 800x600 hud 0
        //314x732 607x757 on 1024x768
        //354x824 684x851 on 1152x864

        let mut ratio = (disp_mode.width as f32 / disp_mode.height as f32) / 1.77777777777;
        ratio = (ratio + 2.0) / 3.0;

        pos.left = (ratio * 0.356 * (disp_mode.width as f32)) as i32;
        pos.right = (ratio * 0.57 * (disp_mode.width as f32)) as i32; //bar len
        pos.right = ((disp_mode.width as f32)
            - (ratio * (disp_mode.width as i32 - pos.right) as f32)) as i32; //space on rigth
        pos.bottom = ((1.0 - 0.015 * ratio) * (disp_mode.height as f32)) as i32 - 1;
        pos.top = pos.bottom - (ratio * ratio * 0.025 * (disp_mode.height as f32)) as i32 + 1;

        self.rescale_for_hud_scaling(&mut pos, disp_mode);

        return pos;
    }
    pub fn set_hud_scaling(&mut self, hud_global_scale: f32) {
        self.hud_scale = 0.333 * hud_global_scale + 0.666;
    }

    fn rescale_for_hud_scaling(
        &self,
        pos: &mut BackendCaptureRect,
        disp_mode: &BackendScreenResolution,
    ) {
        let mid_width = disp_mode.width / 2;
        pos.left =
            (mid_width as f32 - self.hud_scale * (mid_width as f32 - pos.left as f32)) as i32;
        pos.right =
            (mid_width as f32 + self.hud_scale * (pos.right as f32 - mid_width as f32)) as i32;
        pos.top = (disp_mode.height as f32
            - self.hud_scale * (disp_mode.height as f32 - pos.top as f32)) as i32;
        pos.bottom = (disp_mode.height as f32
            - self.hud_scale * (disp_mode.height as f32 - pos.bottom as f32))
            as i32;
    }
}
