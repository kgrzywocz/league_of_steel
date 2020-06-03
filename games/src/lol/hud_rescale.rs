pub fn rescale(hud_scale: f32, value: u32, scale_point: u32) -> u32 {
    let hud_scale = 0.35 * hud_scale + 0.65;
    (scale_point as f32 - hud_scale * (scale_point as f32 - value as f32)) as u32
}
