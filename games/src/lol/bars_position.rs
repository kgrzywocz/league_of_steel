use super::hud_rescale::rescale;

pub struct BarsPosition {
    hud_scale: f32,
    width: u32,
    height: u32,

    health: u32,
    mana: u32,
    range: (u32, u32),
}
impl BarsPosition {
    pub fn new(width: u32, height: u32, hud_scale: f32) -> Self {
        let mut ratio = (width as f32 / height as f32) / 1.77777777777;
        ratio = (ratio + 2.0) / 3.0;

        let range = Self::calculate_range(width, ratio);
        let mana = ((1.0 - 0.015 * ratio) * (height as f32)) as u32 - 1;
        let health = mana - (ratio * ratio * 0.025 * (height as f32)) as u32 + 1;

        let mut res = Self {
            hud_scale,
            width,
            height,
            health,
            mana,
            range,
        };
        res.rescale_for_hud_scaling();
        res
    }

    pub fn get_range(&self) -> (u32, u32) {
        self.range
    }
    pub fn get_health_height(&self) -> u32 {
        self.health
    }
    pub fn get_mana_height(&self) -> u32 {
        self.mana
    }

    fn calculate_range(width: u32, ratio: f32) -> (u32, u32) {
        let range_left = (ratio * 0.356 * (width as f32)) as u32;
        let mut range_right = (ratio * 0.57 * (width as f32)) as u32;
        range_right = ((width as f32) - (ratio * (width - range_right) as f32)) as u32;
        (range_left, range_right)
    }

    fn rescale_for_hud_scaling(&mut self) {
        self.range.0 = rescale(self.hud_scale, self.range.0, self.width / 2);
        self.range.1 = rescale(self.hud_scale, self.range.1, self.width / 2);
        self.health = rescale(self.hud_scale, self.health, self.height);
        self.mana = rescale(self.hud_scale, self.mana, self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_position_1920x1080_hud_1() {
        let sut = BarsPosition::new(1920, 1080, 1.0);

        assert_in_range(sut.get_health_height(), 1030, 1045);
        assert_in_range(sut.get_mana_height(), 1050, 1065);
        assert_eq_with_tolerance(sut.get_range().0, 682);
        assert_eq_with_tolerance(sut.get_range().1, 1094);
    }
    #[test]
    fn test_bar_position_1280x720_hud_1() {
        let sut = BarsPosition::new(1280, 720, 1.0);

        assert_in_range(sut.get_health_height(), 686, 696);
        assert_in_range(sut.get_mana_height(), 700, 709);
        assert_eq_with_tolerance(sut.get_range().0, 455);
        assert_eq_with_tolerance(sut.get_range().1, 730);
    }
    #[test]
    fn test_bar_position_1280x1024_hud_1() {
        let sut = BarsPosition::new(1280, 1024, 1.0);

        assert_in_range(sut.get_health_height(), 983, 996);
        assert_in_range(sut.get_mana_height(), 999, 1011);
        assert_eq_with_tolerance(sut.get_range().0, 416);
        // assert_eq_with_tolerance(sut.get_range().1, 748);
    }
    #[test]
    fn test_bar_position_1024x768_hud_1() {
        let sut = BarsPosition::new(1024, 768, 1.0);

        assert_in_range(sut.get_health_height(), 732, 742);
        assert_in_range(sut.get_mana_height(), 746, 756);
        // assert_eq_with_tolerance(sut.get_range().0, 314);
        // assert_eq_with_tolerance(sut.get_range().1, 608);
    }
    #[test]
    fn test_bar_position_1152x864_hud_1() {
        let sut = BarsPosition::new(1152, 864, 1.0);

        assert_in_range(sut.get_health_height(), 824, 835);
        assert_in_range(sut.get_mana_height(), 840, 851);
        // assert_eq_with_tolerance(sut.get_range().0, 353);
        // assert_eq_with_tolerance(sut.get_range().1, 683);
    }
    #[test]
    fn test_bar_position_800x600_hud_1() {
        let sut = BarsPosition::new(800, 600, 1.0);

        assert_in_range(sut.get_health_height(), 572, 580);
        assert_in_range(sut.get_mana_height(), 583, 591);
        // assert_eq_with_tolerance(sut.get_range().0, 275);
        // assert_eq_with_tolerance(sut.get_range().1, 474);
    }

    #[test]
    fn test_bar_position_1920x1080_hud_50() {
        let sut = BarsPosition::new(1920, 1080, 0.5);

        assert_in_range(sut.get_health_height(), 1038, 1050);
        assert_in_range(sut.get_mana_height(), 1055, 1067);
        assert_eq_with_tolerance(sut.get_range().0, 727);
        assert_eq_with_tolerance(sut.get_range().1, 1071);
    }
    #[test]
    fn test_bar_position_1920x1080_hud_001() {
        let sut = BarsPosition::new(1920, 1080, 0.01);

        assert_in_range(sut.get_health_height(), 1047, 1056);
        assert_in_range(sut.get_mana_height(), 1060, 1069);
        assert_eq_with_tolerance(sut.get_range().0, 776);
        assert_eq_with_tolerance(sut.get_range().1, 1049);
    }
    #[test]
    fn test_bar_position_1024x768_hud_001() {
        let sut = BarsPosition::new(1024, 768, 0.01);

        assert_in_range(sut.get_health_height(), 744, 751);
        assert_in_range(sut.get_mana_height(), 754, 760);
        // assert_eq_with_tolerance(sut.get_range().0, 381);
        // assert_eq_with_tolerance(sut.get_range().1, 575);
    }
    #[test]
    fn test_bar_position_800x600_hud_001() {
        let sut = BarsPosition::new(800, 600, 0.01);

        assert_in_range(sut.get_health_height(), 581, 586);
        assert_in_range(sut.get_mana_height(), 589, 593);
        // assert_eq_with_tolerance(sut.get_range().0, 273);
        // assert_eq_with_tolerance(sut.get_range().1, 449);
    }
    #[test]
    fn test_bar_position_1600x1024_hud_001() {
        let sut = BarsPosition::new(1600, 1024, 0.01);

        assert_in_range(sut.get_health_height(), 992, 1001);
        assert_in_range(sut.get_mana_height(), 1005, 1013);
        // assert_eq_with_tolerance(sut.get_range().0, 625);
        // assert_eq_with_tolerance(sut.get_range().1, 884);
    }
    #[test]
    fn test_bar_position_1680x1050_hud_001() {
        let sut = BarsPosition::new(1680, 1050, 0.01);

        assert_in_range(sut.get_health_height(), 1017, 1026);
        assert_in_range(sut.get_mana_height(), 1030, 1039);
        // assert_eq_with_tolerance(sut.get_range().0, 660);
        // assert_eq_with_tolerance(sut.get_range().1, 926);
    }

    fn assert_in_range(real_val: u32, min: u32, max: u32) {
        assert!(real_val >= min);
        assert!(real_val <= max);
    }
    fn assert_eq_with_tolerance(real_val: u32, expected_val: u32) {
        let eps = 10;
        assert_in_range(real_val, expected_val - eps, expected_val + eps);
    }
}
