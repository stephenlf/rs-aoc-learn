use std::{cell::RefCell, rc::Rc};

pub struct Filesystem {
    root: Rc<RefCell<Folder>>,
    pwd: Rc<RefCell<Folder>>,
}

impl Filesystem {
    /// Creates a new Filesystem. Creates new root folder and sets pwd to root.
    pub fn new() -> Self {
        let root_folder = Folder::new_root();
        Filesystem { 
            root: Rc::clone(&root_folder), 
            pwd: Rc::clone(&root_folder)
        }
    }

    /// Sets pwd to root.
    pub fn cd_root(&mut self) {
        self.pwd = Rc::clone(&self.root);
    }

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

    /// Checks if file exists in pwd. If it doesn't, adds file.
    pub fn ls_file(&self, file_name: String, file_size: usize) {
        todo!()
    }

    /// Checks if child folder exists in pwd. If it doesn't, add folder.
    pub fn ls_folder(&self, folder_name: String) {
        todo!()
    }
}

struct File {
    name: String,
    size: usize,
}

struct Folder {
    name: String,
    size: Option<usize>,
    files: Vec<File>,
    children: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

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

    /// Adds file to self.files
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    /// Takes a reference to a child Folder and adds it to list of `children`
    fn add_child(&mut self, folder: &Rc<RefCell<Folder>>) {
        self.children.push(Rc::clone(folder));
    }
}