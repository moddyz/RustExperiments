#[cfg(test)]
#[path = "./lib_test.rs"]
mod test;

mod world;
pub mod config;

pub fn find_mermaids() {
    crate::world::ocean::find_mermaids();
    world::ocean::find_mermaids();
}
