use std::{cell::RefCell, rc::Rc};
use std::fmt::Display;

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

    /// Sets pwd to listed child folder.
    /// If folder doesn't exist, creates it.
    pub fn cd_child(&mut self, folder_name: &String) {
        let exists = self.pwd.borrow().exists_folder(&folder_name);
        if exists {
            let child = self.pwd.borrow().get_child(&folder_name)
                .expect("Could not find child folder despite checking for its existence");
            self.pwd = child;
        } else {
            self.ls_folder(folder_name);
            self.cd_child(folder_name);
        }
    }

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

    /// Checks if child folder exists in pwd. If it doesn't, add folder.
    pub fn ls_folder(&mut self, folder_name: &String) {
        let mut pwd = self.pwd.borrow_mut();

        if pwd.exists_file(&folder_name) {
            return;
        } else {
            pwd.add_child(
                &Folder::new(folder_name.clone(), &self.pwd)
            );
        }
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

    fn exists_file(&self, name: &String) -> bool {
        for file in &self.files {
            if &file.name == name {
                return true;
            }
        }

        false
    }

    fn exists_folder(&self, name: &String) -> bool {
        for folder in &self.children {
            if &folder.borrow().name == name {
                return true;
            }
        }

        false
    }

    fn get_child(&self, name: &String) -> Option<Rc<RefCell<Folder>>> {
        for folder in &self.children {
            if &folder.borrow().name == name {
                return Some(Rc::clone(folder));
            }
        }
        None
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.size)
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.name.clone();
        
        for child in &self.children {
            let child_fmt = format!("{}", child.borrow());

            let mut lines = child_fmt.lines();
            s.push_str("\n├───");

            s.push_str(lines.next().unwrap());

            for line in lines {
                s.push_str("\n│   ");
                s.push_str(line);
            }
        }

        for file in &self.files {
            s.push_str(format!("\n├───{}", file).as_str());
        }
        write!(f, "{}", s)
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.borrow())
    }
}

#[cfg(test)]
mod filesystem {
    use super::*;

    #[test]
    fn display() {
        let mut fs = Filesystem::new();
        fs.ls_folder(&"folder_1".into());
        fs.ls_folder(&"folder_2".into());
        fs.ls_file("file_1".into(), 10);
        fs.ls_file("file_2".into(), 20);
        fs.ls_file("file_3".into(), 30);

        fs.cd_child(&"folder_3".into());
        fs.ls_file("file_a".into(), 100);
        fs.ls_file("file_b".into(), 200);
        
        fs.cd_parent();
        fs.cd_child(&"folder_1".into());
        fs.ls_file("file_10".into(), 111);
        fs.ls_file("file_11".into(), 222);
        println!("{}", fs);
    }
}