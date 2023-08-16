# A Filesystem Crawler

We now have all the pieces we need to parse today's input. Let's write some code in our main file that can parse input strings as `filesystem` API commands. This is a nontrivial problem, but one you should be well equipped to handle if you've done the other challenges. I will include some hints below. And as always, check out this book's source code for a full implementation.

With our filesystem initialized, its time to figure out how big each folder is, then we can solve the puzzle. One way to solve this is with a simple recursive crawler. The algorithm looks something like this:

1) Attempt to calculate the size of `pwd` by adding up the sizes of all the child folders and files.
2) If a child folder is found that doesn't have a size, set `pwd` to that child and try again.
3) If the size can be calculated, add the value to the folders `size` and set `pwd` to the parent folder.
4) When the size of `root` is found, exit.

This algorithm works best as a `Filesystem` method, since it already has tools to move through the file system.

Here's what my implementation looks like:

```rust
// aoc/day_7/src/filesystem.rs
// .. 
impl Filesystem {
    // ..
    /// Updates the folder size of all folders
    pub fn update_all(&mut self) {
        let update = self.pwd.borrow_mut().try_update();
        match update {
            None => return,
            Some(Ok(parent)) => self.pwd = parent,
            Some(Err(child)) => self.pwd = child,
        }
        self.update_all()
    }
}
// ..
impl Folder {
    // ..
    /// Trys to calculate and update the size of self.
    /// If calculation is successful, returns a pointer to parent.
    /// If calculation fails, returns a pointer to child that caused failure.
    /// If calculation succeeds and self is root, returns "None".
    fn try_update(&mut self) -> Option<Result<Rc<RefCell<Self>>, Rc<RefCell<Self>>>> {
        let mut size: usize = 0;
        for folder in &self.children {
            if let Some(child_size) = folder.borrow().size {
                size += child_size;
            } else {
                return Some(Err(Rc::clone(folder)));
            }
        }

        for file in &self.files {
            size += file.size;
        }

        self.size = Some(size);

        if let Some(parent) = &self.parent {
            Some(Ok(Rc::clone(parent)))
        } else {
            None
        }
    }
}
// ..
```
## Solving the puzzle
We now have a fully populated filesystem with updated folder sizes. It's time to solve the puzzle!

The puzzle question goes as follows:

> Find all of the directories with a total size of at most 100000. **What is the sum of the total sizes of those directories?**

We can solve this puzzle using a crawler similar to the one we used to populate folder sizes. We query each folder for its size, travelling recursively from parent to child (or vice versa), and add up the total. In fact, we can perform the update and the query in one step. Every time `try_update` returns a pointer to a parent folder, we know that all of its child folders have been accounted for. When this happens, we just add `pwd`'s size to some data member `total_size` on our filesystem within our `update_all` function.

Here's what that functionality might look like:

```rust
// aoc/day_7/src/filesystem.rs

pub struct Filesystem {
    root: Rc<RefCell<Folder>>,
    pwd: Rc<RefCell<Folder>>,
    pub total_pt_1: usize,      // <-- NEW
}

impl Filesystem {
    // ..
    /// Updates the folder size of all folders
    pub fn update_all(&mut self) {
        let update = self.pwd.borrow_mut().try_update();
        match update {
            None => return,
            
            // --------------------NEW-----------------------
            Some(Ok(parent)) => {
                let size = self.pwd.borrow().size
                    .expect("Size should have been updated");
                if size <= 100000 {
                    self.total_pt_1 += size;
                }
                self.pwd = parent;
            }
            // ----------------------------------------------
            Some(Err(child)) => self.pwd = child,
        }
        self.update_all()
    }
}
// ..
```
We can then print out `fs.total_pt_1` to get our solution. Good job!