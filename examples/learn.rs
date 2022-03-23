// A type to represent a path, split into its component parts
#[derive(Debug)]
struct Path {
    parts: Vec<String>,
}
impl Path {
    pub fn new(path: &str) -> Path {
        Path {
            parts: path
                .to_string()
                .split("/")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

// A recursive type to represent a directory tree.
// Simplification: If it has children, it is considered
// a directory, else considered a file.
struct Dir {
    name:     String,
    children: Vec<Dir>,
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name:     name.to_string(),
            children: Vec::<Dir>::new(),
        }
    }

    fn find_child(&mut self, name: &str) -> Option<&mut Dir> {
        for c in self.children.iter_mut() {
            if c.name == name {
                return Some(c);
            }
        }
        None
    }

    fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<Dir>, {
        self.children.push(leaf.into());
        self
    }
}

fn dir(val: &str) -> Dir {
    Dir::new(val)
}

fn main() {
    // Form our INPUT:  a list of paths.
    let paths = vec![
        Path::new("introduction.md"),
        Path::new("python_list/methods/append.md"),
        Path::new("python_list/methods/extend.md"),
        Path::new("python_list/methods/append/python_bool.md"),
        Path::new("python_list/methods/README.md"),
        Path::new("python_list/macros/list_macro.md"),
        Path::new("python_list/macros/README.md"),
        Path::new("python_list/README.md"),
        Path::new("python_string/README.md"),
        Path::new("python_string/salutare/readme.md"),
        Path::new("python_string/salutare/asd/readme.md"),
    ];
    // println!("Input Paths:\n{:#?}\n", paths);

    // Transformation:
    // A recursive algorithm that converts the list of paths
    // above to Dir (tree) below.
    // ie: paths --> dir
    let mut top = dir("root");
    for path in paths.iter() {
        build_tree(&mut top, &path.parts, 0);
    }

    // println!(
    //     "Intermediate Representation of Dirs:\n{:#?}\n\nOutput Tree Format:\n",
    //     top
    // );

    // Output:  textual `tree` format
    print_dir(&top, 0);
}

fn build_tree(node: &mut Dir, parts: &Vec<String>, depth: usize) {
    if depth < parts.len() {
        let item = &parts[depth];

        let mut dir = match node.find_child(&item) {
            Some(d) => d,
            None => {
                let d = Dir::new(&item);
                node.add_child(d);
                match node.find_child(&item) {
                    Some(d2) => d2,
                    None => panic!("Got here!"),
                }
            },
        };
        build_tree(&mut dir, parts, depth + 1);
    }
}

// A function to print a Dir in format similar to unix `tree` command.
fn print_dir(dir: &Dir, depth: usize) {
    if depth != 0 {
        let indent = (depth - 1) * 4;
        let tabs = " ".repeat(indent);
        println!("{}- [{}]()", tabs, dir.name);
    }

    for child in dir.children.iter() {
        print_dir(child, depth + 1)
    }
}
