struct HomoPoint<T> {
    x: T,
    y: T,
}

impl<T> HomoPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl HomoPoint<f32> {
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct HeteroPoint<T, V> {
    x: T,
    y: V,
}

impl<T, V> HeteroPoint<T, V> {
    fn mix<A, B>(self, other: HeteroPoint<A, B>) -> HeteroPoint<T, B> {
        HeteroPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let int_point = HomoPoint { x: 5, y: 10 };
    let float_point = HomoPoint { x: 1.0, y: 4.0 };
    println!("int_point.x == {}", int_point.x);
    println!("float_point.magnitude == {}", float_point.magnitude());

    let int_and_float = HeteroPoint { x: 1, y: 4.0 };
    let mixed = int_and_float.mix(HeteroPoint { x: 50.5, y: 20 });
    println!("mixed == {}, {}", mixed.x, mixed.y);
}
