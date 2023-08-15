# A Simulated Filesystem

Let's try creating our filesystem again, the right way. This will be a bit of code, so let's put it all in a new module.

```rust
// aoc/day_7/src/main.rs
mod filesystem;

fn main() {}
```
We can create our `File` object as we did before. We can also define the `name` and `files` members in our `Folder` object. At some point, we'll also want to store the `size` of each `Folder` as well, so let's define that as an optional data member.
```rust
// aoc/day_7/src/filesystem.rs  <--DIFFERENT
struct File {
    name: String,
    size: usize,
}

struct Folder {
    name: String,
    size: Option<usize>,
    files: Vec<File>,
}
```
Instead of having each parent folder own its child folders, let's have the parents own an `Rc` pointing to their child folders. That will allow the parent to pass references around and give us flexibility to travel from parent to child and vice versa.
```rust
// aoc/day_7/src/filesystem.rs
use std::{cell::RefCell, rc::Rc};
// ..
struct Folder {
    name: String,
    size: Option<usize>,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Folder>>>,
}
```
Now we can implement a `parent` datamember as well, which is a pointer to another folder. We'll make this member optional, since our root folder won't have a parent. This becomes super easy with `Rc<RefCell<_>>`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
struct Folder {
    name: String,
    size: Option<usize>,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}
```
And we're done! See how easy it is with smart pointers? No messing with lifetimes. And as long as we don't try to mutably alter each folder more than once at a time, we won't have to mess with the borrow checker either.

## Building our filesystem

Let's start defining functions to build our virtual filesystem from inputs. We'll start by defining the following functions. See if you can't implement them yourself from their definitions and docscript. Once you've given it a solid go, come back and we'll walk through it together.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl Folder {
    /// Instantiates a new `Folder` named "/" with no parent folder.
    fn new_root() -> Rc<RefCell<Self>> {
        todo!()
    }

    /// Creates a new folder with given name and parent.
    /// Takes in reference to pointer to parent, clones it, and stores it.
    fn new(name: String, parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        todo!()
    }

    /// Adds file to self.files
    fn add_file(&mut self, file: File) {
        todo!()
    }

    /// Takes a reference to a child Folder and adds it to list of `children`
    fn add_child(&mut self, folder: &Rc<RefCell<Folder>>) {
        todo!()
    }
}
```
Good luck! I'll even put a big bar here as a signal to stop and try these implementations before moving on.

---

Did you try it? Was it tricky? If you did, _great work_! Let's see how we can implement this together.

The `new_root` function is fairly simple. We can stick with `Default`'s for pretty much every datamember.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl Folder {
    /// Instantiates a new `Folder` named "/" with no parent folder.
    fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name: "/".to_string(),
                size: None,
                files: Vec::new(),
                children: Vec::new(),
                parent: None
            }
        ))
    }
// ..
}
```
Note that since we are only dealing with Folders wrapped in `Rc<RefCell<_>>`, we choose to return a smart pointer to the root folder right away, rather than trying to remember to wrap our root in a smart pointer at instantiation. 

The `new` function is similar. However, we need to save the pointer referenced by the `parent` parameter. We can do that with `Rc::clone`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl Folder {
// ..
    /// Creates a new folder with given name and parent.
    /// Takes in reference to pointer to parent, clones it, and stores it.
    fn new(name: String, parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name,
                size: None,
                files: Vec::new(),
                children: Vec::new(),
                parent: Some(Rc::clone(parent)),
            }
        ))
    }
// ..
}
```
Not too bad! Adding new files is a simple matter of pushing the input file onto `self.files`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl Folder {
// ..
    /// Adds file to self.files
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
// ..
}
```
We can use `Rc::clone` to add child folders in `add_child`, just as we did to add a parent folder in `new`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl Folder {
// ..
    /// Takes a reference to a child Folder and adds it to list of `children`
    fn add_child(&mut self, folder: &Rc<RefCell<Folder>>) {
        self.children.push(Rc::clone(folder));
    }
}
```
Very nice! We now have the beginnings of a functional virtual filesystem.

## A Filesystem Crawler

Our input will give us four types of commands to parse: `cd child_folder`, `cd ..`, `cd /`, and `ls`. Each of these (except `cd /`) will have different behavior depending on the "working directory" the command is called from. If we are in the root folder, then `ls` will list all of the files and folders in the root folder. But if we're in a child folder, then `ls` will list all of the files and folders in that child folder. Similar constraints apply to the `cd` commands. 

How should we keep track of this working directory? One solution is to create a new struct, `Filesystem`, which has two datamembers: a pointer to the working directory (`pwd`) and a pointer to the root. Let's write that out.

```rust
// aoc/day_7/src/filesystem.rs
// ..

pub struct Filesystem {
    root: Rc<RefCell<Folder>>,
    pwd: Rc<RefCell<Folder>>,
}

// ..
```

From our `Filesystem` struct, we can implement the high-level methods that our input will provide. Structured this way, the `Filesystem` can also serve as the sole public interface for this module. Let's write out what these functions might look like. As before, try to give these functions a go before reading the solution below.

Keep in mind that when we implement the `ls` command, we will kick off a loop that calls `ls_file` or `ls_folder` for every line of output. For now, we only need to create implementations for a single line of the `ls` output.

```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    /// Creates a new Filesystem. Creates new root folder and sets pwd to root.
    pub fn new() -> Self {
        todo!()
    }

    /// Sets pwd to root.
    pub fn cd_root(&mut self) {
        todo!()
    }

    /// Sets pwd to current pwd's parent folder, if it has one.
    pub fn cd_parent(&mut self) {
        todo!()
    }

    /// Sets pwd to listed child folder.
    /// If folder doesn't exist, creates it.
    pub fn cd_child(&self, folder_name: String) {
        todo!()
    }

    /// Checks if file exists in pwd. If it doesn't, adds file.
    pub fn ls_file(&self, file_name: String, file_size: usize) {
        todo!()
    }

    /// Checks if child folder exists in pwd. If it doesn't, add folder.
    pub fn ls_folder(&self, folder_name: String) {
        todo!()
    }
}
// ..
```
Ready to start implementing? Let's gooooooooo

Creating a new `Filesystem` is a simple matter of creating a new `root` folder and assigning its pointer to `self.root` and `self.pwd`
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Creates a new Filesystem. Creates new root folder and sets pwd to root.
    pub fn new() -> Self {
        let root_folder = Folder::new_root();
        Filesystem { 
            root: Rc::clone(&root_folder), 
            pwd: Rc::clone(&root_folder)
        }
    }
    // ..
}
```
The `cd_root` function works similarly, setting `self.pwd` to `self.root`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Sets pwd to root.
    pub fn cd_root(&mut self) {
        self.pwd = Rc::clone(&self.root);
    }
    // ..
}
```
`cd_parent` requires us to first grab a reference to `pwd`'s `parent folder`, then assign a copy of that pointer to `self.pwd`. If `pwd` has no parent folder, then we are already at root and we do nothing.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Sets pwd to current pwd's parent folder, if it has one.
    pub fn cd_parent(&mut self) {
        let parent_reference = match &self.pwd.borrow().parent {
            // pwd == root, do nothing.
            None => return,

            // pwd != root. Grab reference to pwd's parent folder.
            // parent has type &Rc<RefCell<Folder>>
            Some(parent) => Rc::clone(parent),
        };
        
        self.pwd = parent_reference;
    }
    // ..
}
```

For the `ls_file` command, we first check if the given file already exists in `pwd`. We can write a utility function `Folder::exists_file` to check if the given file exists. If it doesn't, then we create a new file and add it to `pwd`.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Checks if file exists in pwd. If it doesn't, adds file.
    pub fn ls_file(&self, file_name: String, file_size: usize) {
        let mut pwd = self.pwd.borrow_mut();
        if pwd.exists_file(&file_name) {
            return;
        } else {
            let new_file = File {
                name: file_name,
                size: file_size,
            };
            pwd.add_file(new_file);
        }
    }
    // ..
}
// ..
impl Folder {
    // ..
    fn exists_file(&self, name: &String) -> bool {
        for file in &self.files {
            if &file.name == name {
                return true;
            }
        }

        false
    }
}
```
The logic of `ls_folder` is equivalent to that of `ls_file`. We'll just have to be careful about our borrows.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Checks if child folder exists in pwd. If it doesn't, add folder.
    pub fn ls_folder(&self, folder_name: String) {
        let mut pwd = self.pwd.borrow_mut();

        if pwd.exists_file(&folder_name) {
            return;
        } else {
            pwd.add_child(
                &Folder::new(folder_name, &self.pwd)
            );
        }
    }
    // ..
}
// ..
impl Folder {
    // ..
    fn exists_folder(&self, name: &String) -> bool {
        for folder in &self.children {
            if &folder.borrow().name == name {
                return true;
            }
        }

        false
    }
}
```
`cd_child` is just a combination of `cd_parent` logic plus a call to `ls_folder` if the child folder doesn't exist. We define a `Folder::get_child` function to help us with this.
```rust
// aoc/day_7/src/filesystem.rs
// ..
impl FileSystem {
    // ..
    /// Sets pwd to listed child folder.
    /// If folder doesn't exist, creates it.
    pub fn cd_child(&self, folder_name: String) {
        let mut pwd = self.pwd.borrow_mut();
    }
    // ..
}
```

Nice! Let's run a few tests to see if things are working well.

## Testing and the `Display` trait

Before moving on, let's see if our code is working. We could set up some unit tests. However, I would have a better grasp of our code if I could visualize it. Instead of unit tests, let's build a visualization of our filesystem to see if it matches up with our expectation. 

We can implement the `std::fmt::Display` trait to define how our structs will get printed to the screen. I already talked about the `Display` trait in Day 5's tutorial, so I won't go into too much detail here. Instead, let's define what our visualization should look like.

The goal will be to create a visualization that matches the output of the `tree` CLI command, like below.

```bash
/
├───folder_1
│   └───file_1_1
├───folder_1
│   │───file_2_1
│   └───file_2_2
└───file_root_1
```

All of this is totally optional and only tangentially related to the puzzle, so I won't bore you too much with the details. In essence, we first define `Display` for `File`, which is just the file name and size, separated by a space. We then define `Display` for `Folder`, which prints its name, the recursively prints its child folders, and finally prints its files. Check out the implementation in the source code for this book.

## Parsing our input
We now have all the pieces we need to parse today's input. Let's write some code in our main file that can parse input strings as `filesystem` api commands.