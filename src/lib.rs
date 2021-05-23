#[cfg(test)]
#[path = "./lib_test.rs"]
mod test;

#[cfg(test)]
#[path = "./minigrep_test.rs"]
mod minigrep_test;

pub mod doctest;
pub mod minigrep;
mod world;

pub fn find_mermaids() {
    crate::world::ocean::find_mermaids();
    world::ocean::find_mermaids();
}
