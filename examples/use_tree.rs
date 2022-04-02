use std::str::Split;
use core_dev::collections::TreeNode;
use core_dev::traits::StringExtended;


// https://stackoverflow.com/questions/60479260/how-to-convert-a-flat-list-of-directory-paths-to-hierarchical-struct-in-rust

fn main() {
    let mut head = TreeNode::new(
        String::from("Python List"),
        String::from("./python_list/README.md"),
        None,
    );


    let mut leaves = Vec::new();
    for int in (0..5i32).rev() {
        leaves.push(TreeNode {
            name:  int.to_string() + "luigi",
            path:  format!("./{}/README.md", int.to_string() + "luigi"),
            nodes: None,
        });
    }

    // head.print_children();

    head.nodes = Some(leaves);

    // head.print_children();


    let mut leaves = Vec::new();
    for int in (0..5i32).rev() {
        leaves.push(TreeNode {
            name:  int.to_string() + "mario",
            path:  format!("./{}/README.md", int.to_string() + "luigi"),
            nodes: None,
        });
    }
    head.add_node(TreeNode::new(
        "saluatre".to_string(),
        String::from("./salutare/readme.md"),
        None,
    ));


    head.add_node(TreeNode::new(
        "inner".to_string(),
        String::from("./inner/readme.md"),
        Some(leaves),
    ));

    // head.print_children(0);


    // let allch = head.get_all_children();
    // println!("{:?}", allch);

    head.generate_summary(0);
    TreeNode::sort_nodes_alphabetically(&mut head);
    head.generate_summary(0);


    // python_list
    //     methods
    //         readme.md
    //         append.md
    //         extend.md
    //     macros
    //         readme.md
    //         list

    // python_string
    //     salutare
    //         readme.md
    //     asd
    //         readme.md


    // astea trebuie neaparat sa fie in ordinea astsa
    // fara / primele
    // dupa toate readme in ordine
    // si dupa ce a mai ramas
    let lines = vec![
        // TODO find a method to insert all the items with non slash on their original positions
        String::from("introduction.md"),
        // dai collect la toate si le sortezi dupa depth of slash
        String::from("python_list/methods/extend.md"),
        String::from("python_list/methods/append/python_bool.md"),
        String::from("python_list/macros/list_macro.md"),
        String::from("python_list/README.md"),
        String::from("python_list/macros/README.md"),
        String::from("introduction.md"),
        String::from("python_list/methods/append/python_bool2.md"),
        String::from("python_list/methods/append/python_bool3.md"),
        String::from("python_list/methods/README.md"),
        String::from("python_list/methods/append/README.md"),
        String::from("python_string/README.md"),
        String::from("introduction.md"),
    ];


    let mut all_readmes = vec![];
    for line in lines.iter() {
        if line.contains("README.md") {
            all_readmes.push(line.clone());
        }
    }

    all_readmes
        .sort_by_key(|item| item.split("/").collect::<Vec<&str>>().len());

    for line in lines.iter() {
        if line.contains("README.md") {
            continue;
        }
        if line.contains("/") {
            all_readmes.push(line.clone());
        } else {
            // all_readmes.insert(line.clone());
        }
    }


    for readme in all_readmes.iter() {
        println!("{}", readme);
    }

    fn build_tree(root: &mut TreeNode, items: &Vec<String>, index: usize) {
        if index < items.len() {
            let item = &items[index];
            if item == "README.md" {
                return;
            }

            let mut new_node = match root.find_node(&item) {
                Some(found_node) => found_node,
                None => {
                    let new_node = TreeNode::new(
                        item.clone(),
                        "./".to_string() + &items.join("/"),
                        Some(vec![]),
                    );
                    root.add_node(new_node);
                    root.find_node(&item).unwrap()
                },
            };
            build_tree(&mut new_node, items, index + 1);
        }
    }

    let mut root =
        TreeNode::new("head".to_string(), "---".to_string(), Some(vec![]));


    for line in all_readmes {
        if line.contains("/") {
            let items = line.split_to_vec_string("/");
            build_tree(&mut root, &items, 0);
        } else {
            if let Some(nodes) = &mut root.nodes {
                let title =
                    line.split(".").collect::<Vec<&str>>()[0].capitalize();

                nodes.push(TreeNode::new(title, line.clone(), None));
            }
        }
    }

    fn format_names(root: &mut TreeNode) {
        if let Some(nodes) = &mut root.nodes {
            for node in nodes {
                let item = &node.name;
                let formatted_name = if item.contains(".") {
                    item.split(".").collect::<Vec<&str>>()[0].capitalize()
                } else if item.contains("_") {
                    item.split("_")
                        .map(|s| s.capitalize())
                        .collect::<Vec<String>>()
                        .join(" ")
                } else {
                    item.capitalize().clone()
                };
                node.name = formatted_name;

                if node.nodes.is_some() {
                    format_names(node);
                }
            }
        }
    }

    format_names(&mut root);

    let summ = root.generate_summary_string();
    println!("{}", summ);
}
