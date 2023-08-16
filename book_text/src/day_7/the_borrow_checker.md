# The **Borrow Checker** and the Shared Mutable Reference Problem

I have hinted that Rust's **borrow checker** will make today's challenge harder than expected. Before we talk about why, let's review what the borrow checker does.

The borrow checker enforces the following two rules _at compiletime_:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

The purpose of these rules are described well in the [Rust Book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), among other places. But why does it make today's challenge hard?

Let's think about what a filesystem needs to do. First, we need to model `File`'s and `Folder`'s. The `Folder`'s hold `File`'s and the `File`'s hold file sizes. Both types also contain their name.

```rust
struct File {
    name: String,
    size: usize,
}

struct Folder {
    name: String,
    files: Vec<File>,
}
```

Folders can be nested, so we'll need to add a "child folders" item to our `Folder`.

```rust
struct Folder {
    name: String,
    files: Vec<File>,
    children: Vec<Folder>,
}
```

From the borrow checker's perspective, each `Folder` _owns_ each of its child `Folder`'s. Modelled like this, we will have no issue moving from parent to child, modifying each folder as we go. (In fact, what we've created is not very different from a [singly-linked list](https://rust-unofficial.github.io/too-many-lists/first-layout.html)).

To demonstrate this, we could write a function `get_mut_child`, which gets a mutable reference to a child folder, like so:

```rust
impl Folder {
    fn get_mut_child(&mut self, index: usize) -> &mut Self {
        &mut self.child_folders[index]
    }
}
```

One pattern our input uses looks like `cd "folder_name"`, which will prompt us to begin populating the folder `folder_name` with files and child folders. Getting a mutable reference to a child folder with `get_mut_child` means that we can modify the child as we wish, including adding new grandchild folders and files.

But what if our input gives us the command `cd ..`? How can we get a mutable reference to a parent folder? We may try holding a storing a reference to the parent folder in each child folder, like so:

```rust
struct Folder {
    name: String,
    files: Vec<File>,
    child_folders: Vec<Folder>,
    parent_folder: &Folder,
}
```
Will it build?
```bash
aoc/day_7 $ cargo build
    Compiling day_7 v0.1.0 (~/aoc/day_7)
    error[E0106]: missing lifetime specifier
    --> day_7/src/main.rs:10:20
    |
    10 |     parent_folder: &Folder,
    |                    ^ expected named lifetime parameter
    |
    help: consider introducing a named lifetime parameter
    |
    6  ~ struct Folder<'a> {
    7  |     name: String,
    8  |     files: Vec<File>,
    9  |     child_folders: Vec<Folder>,
    10 ~     parent_folder: &'a Folder,
    |
```
Oof. The borrow checker has thrown its first punch. A lifetime issue. Let's apply the fix suggested by the compiler and try again.
```rust
struct Folder<'a> {
    name: String,
    files: Vec<File>,
    child_folders: Vec<Folder<'a>>,
    parent_folder: &'a Folder<'a>,
}

impl<'a> Folder<'a> {/*...*/}
```
This compiles fine, but can we implement a `get_mut_parent` function? Let's give it a go.

```rust
impl<'a> Folder<'a> {
    /*...*/

    fn get_mut_parent(&mut self) -> &mut Self {
        self.parent_folder
    }
}
```
Try to build this, however, and you will see it fail again.
```bash
aoc/day_7 $ cargo check
        Checking day_7 v0.1.0 (~/aoc/day_7)
    error[E0596]: cannot borrow data in a `&` reference as mutable
    --> day_7/src/main.rs:19:9
    |
    19 |         &mut self.parent_folder
    |         ^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable

    For more information about this error, try `rustc --explain E0596`.
    error: could not compile `day_7` (bin "day_7") due to previous error
```
Whoops. Let's store a mutable reference to the parent folder.
```rust
struct Folder<'a> {
    name: String,
    files: Vec<File>,
    child_folders: Vec<Folder<'a>>,
    parent_folder: &'a mut Folder<'a>,
}
```
The build looks good. But can we use it in practice? Unfortunately, no. Our root folder doesn't have a parent, but trying to solve that with some `Option` trickery opens up a whole host of lifetime and ownership issues.

As it turns out, our whole approach is fundamentally flawed. Remember the borrow checker's first rule:

> At any given time, you can have either **one mutable reference** or any number of immutable references.

This means that our child folders can't _ever_ pass around mutable references and hold on to them at the same time. The compiler won't allow it. The borrow checker has thrown its second punch: _ownership_.

You can try all manner of trickery to open up this sort of behavior. However, it turns out we only really have two options. We can pass around raw pointers with `unsafe` Rust, or we can take advantage of the `RefCell<Rc>` pattern. For this challenge, we will be using the latter.

> **Doubly linked lists**
> The structure we're trying to create is very similar to the doubly-linked list, which gets extensive treatment in _[Learning Rust through Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/fourth.html)_. 