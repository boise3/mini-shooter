use raylib::prelude::*;

pub struct Button {
    pub position: Rectangle, // x y width height
    pub text: String,
    pub font_size: i32,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str, font_size: i32) -> Self {
        Self {
            position: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            font_size,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let mouse_pos = d.get_mouse_position();
        let is_hovered = self.position.check_collision_point_rec(mouse_pos);

        let bg_color = if is_hovered { Color::GRAY } else { Color::DARKGRAY };


        d.draw_rectangle_rec(self.position, bg_color);
        d.draw_rectangle_lines_ex(self.position, 2.0, Color::WHITE);


        let text_width = d.measure_text(&self.text, self.font_size);
        let text_x = self.position.x + (self.position.width - text_width as f32) / 2.0;
        let text_y = self.position.y + (self.position.height - self.font_size as f32) / 2.0;

        d.draw_text(&self.text, text_x as i32, text_y as i32, self.font_size, Color::WHITE);
    }

    pub fn is_clicked(&self, mut rl: &mut RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        let is_hovered = self.position.check_collision_point_rec(mouse_pos);

        is_hovered && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }
}