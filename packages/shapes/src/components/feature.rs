use std::fmt;

// derive equality for features
#[derive(Clone, Debug, PartialEq)]
pub enum Feature {
    Area,
    Perimeter,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Feature::Area => write!(f, "Area"),
            Feature::Perimeter => write!(f, "Perimeter"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod unit {
        use super::*;
        #[test]
        fn display_area() {
            let feature = Feature::Area;
            assert_eq!(format!("{}", feature), "Area");
        }

        #[test]
        fn display_perimeter() {
            let feature = Feature::Perimeter;
            assert_eq!(format!("{}", feature), "Perimeter");
        }
        #[test]
        fn values() {
            assert_eq!(0, Feature::Area as u8);
            assert_eq!(1, Feature::Perimeter as u8);
        }
    }
}
