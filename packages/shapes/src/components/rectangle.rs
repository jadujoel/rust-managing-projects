use super::Feature;

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter(),
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod unit {
        use super::*;
        #[test]
        fn new() {
            let rect = Rectangle::new(1.0, 2.0);
            assert_eq!(rect.width, 1.0);
            assert_eq!(rect.height, 2.0);
        }
        #[test]
        fn get_feature_area() {
            let rect = Rectangle::new(1.0, 2.0);
            let area = rect.get_feature(Feature::Area);
            assert_eq!(area, 2.0);
        }

        #[test]
        fn get_feature_perimiter() {
            let rect = Rectangle::new(1.0, 2.0);
            let perimeter = rect.get_feature(Feature::Perimeter);
            assert_eq!(perimeter, 6.0);
        }

        #[test]
        fn calc_area() {
            let rect = Rectangle::new(1.0, 2.0);
            let area = rect.calc_area();
            assert_eq!(area, 2.0);
        }
        #[test]
        fn calc_circumference() {
            let rect = Rectangle::new(1.0, 2.0);
            let perimeter = rect.calc_perimeter();
            assert_eq!(perimeter, 6.0);
        }
    }
}
