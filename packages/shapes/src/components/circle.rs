use super::Feature;

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_circumference(),
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod test {
    mod unit {
        use super::super::*;

        #[test]
        fn new() {
            let circ = Circle::new(3.0);
            assert_eq!(circ.radius, 3.0);
        }
        #[test]
        fn get_feature_area() {
            let circ = Circle::new(3.0);
            let area = circ.get_feature(Feature::Area);
            assert_eq!(area, 28.274333882308138);

            let circ = Circle::new(0.0);
            let area = circ.get_feature(Feature::Area);
            assert_eq!(area, 0.0);

            let circ = Circle::new(-3.0);
            let area = circ.get_feature(Feature::Area);
            assert_eq!(area, 28.274333882308138);
        }

        #[test]
        fn get_feature_perimiter() {
            let circ = Circle::new(3.0);
            let perimeter = circ.get_feature(Feature::Perimeter);
            assert_eq!(perimeter, 18.84955592153876);

            let circ = Circle::new(0.0);
            let perimeter = circ.get_feature(Feature::Perimeter);
            assert_eq!(perimeter, 0.0);

            let circ = Circle::new(-3.0);
            let perimeter = circ.get_feature(Feature::Perimeter);
            assert_eq!(perimeter, -18.84955592153876);
        }

        #[test]
        fn calc_area() {
            let circ = Circle::new(3.0);
            let area = circ.calc_area();
            assert_eq!(area, 28.274333882308138);

            let circ = Circle::new(0.0);
            let area = circ.calc_area();
            assert_eq!(area, 0.0);

            let circ = Circle::new(-3.0);
            let area = circ.calc_area();
            assert_eq!(area, 28.274333882308138);
        }
        #[test]
        fn calc_circumference() {
            let circ = Circle::new(3.0);
            let perimeter = circ.calc_circumference();
            assert_eq!(perimeter, 18.84955592153876);

            let circ = Circle::new(0.0);
            let perimeter = circ.calc_circumference();
            assert_eq!(perimeter, 0.0);

            let circ = Circle::new(-3.0);
            let perimeter = circ.calc_circumference();
            assert_eq!(perimeter, -18.84955592153876);
        }
    }
}
