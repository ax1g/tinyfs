#[derive(Debug)]
enum Node {
    File {
        name: String,
    },

    Directory {
        name: String,
        children: Vec<Box<Node>>,
    },
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Self::Directory { name, .. } => name,
            Self::File { name } => name,
        }
    }

    fn add_file(&mut self, name: &str) {
        match self {
            Self::Directory { children, .. } => {
                let file = Self::File {
                    name: name.to_string(),
                };

                children.push(Box::new(file));
            }

            Self::File { .. } => {
                println!("cannot add a file inside a file");
            }
        }
    }

    fn add_dir(&mut self, name: &str) {
        match self {
            Self::Directory { children, .. } => {
                let dir = Self::Directory {
                    name: name.to_string(),
                    children: vec![],
                };

                children.push(Box::new(dir));
            }

            Self::File { .. } => {}
        }
    }

    fn child_mut(&mut self, index: usize) -> Option<&mut Node> {
        match self {
            Self::Directory { children, .. } => {
                if index >= children.len() {
                    return None;
                }

                let child_box = &mut children[index];

                let child_node = child_box.as_mut();

                Some(child_node)
            }

            Self::File { .. } => None,
        }
    }

    fn child_named_mut(&mut self, wanted: &str) -> Option<&mut Node> {
        match self {
            Self::Directory { children, .. } => {
                for child in children {
                    if child.name() == wanted {
                        return Some(child.as_mut());
                    }
                }

                None
            }

            Self::File { .. } => None,
        }
    }

    fn ls(&self) {
        match self {
            Self::Directory { children, .. } => {
                for child in children {
                    match child.as_ref() {
                        Self::Directory { name, .. } => {
                            println!("{}/", name);
                        }

                        Self::File { name } => {
                            println!("{}", name);
                        }
                    }
                }
            }

            Self::File { .. } => {
                println!("not a directory");
            }
        }
    }

    fn tree(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}{}", indent, self.name());

        match self {
            Self::Directory { children, .. } => {
                for child in children {
                    child.tree(depth + 1);
                }
            }

            Self::File { .. } => {}
        }
    }

    fn find(&self, wanted: &str) -> Option<&Node> {
        if self.name() == wanted {
            return Some(self);
        }

        match self {
            Self::Directory { children, .. } => {
                for child in children {
                    if let Some(result) = child.find(wanted) {
                        return Some(result);
                    }
                }

                None
            }

            Self::File { .. } => None,
        }
    }
}

fn main() {
    let mut root = Node::Directory {
        name: "/".to_string(),
        children: vec![],
    };

    root.add_dir("home");

    let home = root.child_named_mut("home").unwrap();
    home.add_dir("ag");

    let ag = home.child_named_mut("ag").unwrap();
    ag.add_dir("workspace");
    ag.add_dir("notes");

    let workspace = ag.child_named_mut("workspace").unwrap();
    workspace.add_file("hello.rs");

    workspace.add_dir("rust");
    let rust = workspace.child_named_mut("rust").unwrap();
    rust.add_dir("src");

    let src = rust.child_named_mut("src").unwrap();
    src.add_file("main.rs");

    let notes = ag.child_named_mut("notes").unwrap();
    notes.add_file("rust.txt");

    // println!("{root:#?}");
    root.tree(0);

    println!("{:#?}", root.find("src"));
}
