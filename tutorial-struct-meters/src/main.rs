struct Meters(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }

    fn value(&self) -> f64 {
        self.0
    }

    fn add(&self, other: &Meters) -> Meters {
        Meters(self.0 + other.0)
    }

    fn subtract(&self, other: &Meters) -> Meters {
        Meters(self.0 - other.0)
    }

    fn powi(&self, exponent: i32) -> Meters {
        Meters(self.0.powi(exponent))
    }

    fn sqrt(&self) -> Meters {
        Meters(self.0.sqrt())
    }
}

struct Kilometers(f64);

impl Kilometers {
    fn new(value: f64) -> Self {
        Kilometers(value)
    }

    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1000.0)
    }
}

fn calculate_area(radius: &Meters) -> MetersSquared {
    let area = std::f64::consts::PI * radius.0.powi(2);
    MetersSquared(area)
}

fn calculate_volume(radius: &Meters) -> MetersCubed {
    let volume = std::f64::consts::PI * radius.0.powi(3);
    MetersCubed(volume)
}

struct MetersCubed(f64);
struct MetersSquared(f64);

fn main() {
    let distance_in_meters = Meters::new(1500.0);

    println!("Distance in meters: {:.2} m with value", distance_in_meters.value());

    let distance_in_kilometers = distance_in_meters.to_kilometers();
    println!("Distance in kilometers: {:.2} km", distance_in_kilometers.0);

    let distance_in_km = Kilometers::new(2.5);
    let distance_in_m = distance_in_km.to_meters();
    println!("Distance in meters: {:.2} m", distance_in_m.0);

    let radius = Meters::new(5.0);
    let area = calculate_area(&radius);
    println!("Area of circle with radius 5m: {:.2} m²", area.0);

    let volume = calculate_volume(&radius);
    println!("Volume of sphere with radius 5m: {:.2} m³", volume.0);

    let length1 = Meters::new(300.0);
    let length2 = Meters::new(450.0);
    let total_length = length1.add(&length2);
    println!("Total length: {:.2} m", total_length.0);

    let length_diff = length2.subtract(&length1);
    println!("Length difference: {:.2} m", length_diff.0);


    let squared_length = length1.powi(2);
    println!("Length squared: {:.2} m²", squared_length.0);

    let sqrt_length = length2.sqrt();
    println!("Square root of length2: {:.2} m", sqrt_length.0);

}