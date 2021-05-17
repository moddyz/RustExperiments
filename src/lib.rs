mod world;

#[cfg(test)]
#[path = "./lib_test.rs"]
mod test;

pub fn find_mermaids() {
    crate::world::ocean::find_mermaids();
    world::ocean::find_mermaids();
}
