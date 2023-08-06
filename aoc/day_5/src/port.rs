pub struct Port(Vec<Dock>);

impl Port {
    /// Creates a new port from ...some input???
    pub fn new() -> Self {
        todo!()
    }

    /// Moves `num_crates` number of crates from the top of stack `origin`
    /// to the top of stack `destination`. Crates are moved one at a time.
    pub fn arrange(&mut self, num_crates: u32, origin: usize, dest: usize) {
        todo!()
    }

    /// Prints top crate of each Dock to stout.
    pub fn print(&self) {
        println!("{}", todo!());
    }
}

struct Dock(Vec<char>);