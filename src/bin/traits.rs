struct Cube {
    width: f32,
    height: f32,
    depth: f32,
}

struct Sphere {
    radius: f32,
}

struct Cone {
    base_radius: f32,
    height: f32,
}

trait Volume {
    fn volume(&self) -> f32 {
        0.0
    }
}

impl Volume for Cube {
    fn volume(&self) -> f32 {
        self.width * self.height * self.depth
    }
}

impl Volume for Sphere {
    fn volume(&self) -> f32 {
        self.radius.powi(3) * 3.141592 * 4.0 / 3.0
    }
}

impl Volume for Cone {}

fn report_volume(object: &impl Volume) {
    println!("Volume is {}", object.volume());
}

fn exclaim_volume<T: Volume>(object: &T) {
    println!("Volume is {}!!!", object.volume());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let cube = Cube {
        width: 6.0,
        height: 6.0,
        depth: 6.0,
    };
    println!("cube.volume() == {}", cube.volume());

    let sphere = Sphere { radius: 3.0 };
    println!("sphere.volume() == {}", sphere.volume());

    // cone.volume falls back to default implementation.
    let cone = Cone {
        base_radius: 3.0,
        height: 10.0,
    };
    println!("cone.volume() == {}", cone.volume());

    // Report
    report_volume(&cube);
    report_volume(&sphere);
    report_volume(&cone);

    // Exclaim!
    exclaim_volume(&cube);
    exclaim_volume(&sphere);
    exclaim_volume(&cone);

    // find largest numbers.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
