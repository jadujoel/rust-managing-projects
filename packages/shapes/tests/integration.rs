use rand::prelude::*;
use shapes::*;

fn generate_two_random_numbers() -> (f64, f64) {
    let width = generate_random_number();
    let height = generate_random_number();
    (width, height)
}

fn generate_random_number() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..100.0)
}

#[test]
fn integration_random_rectangle() {
    let (width, height) = generate_two_random_numbers();
    let rect = Rectangle::new(width, height);

    let area = rect.get_feature(Feature::Area);
    assert_eq!(area, width * height);

    let perimeter = rect.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * width + 2.0 * height);
}

#[test]
fn integration_random_circle() {
    let radius = generate_random_number();
    let circ = Circle::new(radius);

    let area = circ.get_feature(Feature::Area);
    assert_eq!(area, std::f64::consts::PI * radius * radius);

    let perimeter = circ.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * std::f64::consts::PI * radius);
}
