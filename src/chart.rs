use serde_derive::Serialize;
use tera::Context;

#[derive(Clone, Debug)]
pub struct Chart {
    pub name: String,
    pub points: Vec<Point>,
    pub smooth: bool,
    pub colour: String,
    pub y_granularity: usize,
}

// Inspired by code from the Lorikeet Dashboard.
impl Chart {
    pub fn add_point(&mut self, x: usize, y: usize) {
        self.points.push(Point { x, y });
    }

    pub fn draw_svg(&self, width: usize, height: usize, markers: &[(usize, String)]) -> Context {
        let mut context = Context::new();

        let min_x = self.points[0].x;
        let max_x = self.points.last().unwrap().x;
        let min_y = 0;
        let max_value_y = self.points.iter().map(|point| point.y).max().unwrap();
        let max_y = if max_value_y % self.y_granularity == 0 {
            max_value_y
        } else {
            (max_value_y / self.y_granularity + 1) * self.y_granularity
        };

        let max_label = format!("{}", max_y + min_y);

        let p_left = max_label.len() as f64 * 9.0 + 5.0;
        let p_other = 50.0;

        let width = width as f64 - p_left - p_other;
        let height = height as f64 - p_other * 2.0;

        let p_first_y = (self.points[0].y - min_y) as f64 / max_y as f64 * (-1.0 * height)
            + p_other as f64
            + height as f64;

        let path = self
            .points
            .iter()
            .map(|point| {
                let x = (point.x - min_x) as f64 / max_x as f64 * width + p_left;
                let y = (point.y - min_y) as f64 / max_y as f64 * (-1.0 * height)
                    + p_other as f64
                    + height as f64;

                format!("L {} {}", x, y)
            })
            .collect::<Vec<String>>()
            .join("");

        context.insert("path", &path);
        context.insert("name", &self.name);
        context.insert("width", &(width));
        context.insert("height", &(height));
        context.insert("p_left", &p_left);
        context.insert("p_other", &p_other);
        context.insert("p_first_y", &p_first_y);
        context.insert("max_y", &max_y);
        context.insert("min_y", &min_y);
        context.insert("max_x", &max_x);
        context.insert("min_x", &min_x);
        context.insert("lines", &5);
        context.insert("colour", &self.colour);
        context.insert("markers_x", &markers.to_vec());
        context
    }
}

#[derive(Serialize, Clone, Debug, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
