// use default_args::default_args;


type NodesOption = Option<Vec<TreeNode>>;

pub struct TreeNode {
    pub name:  String,
    pub path:  String,
    pub nodes: NodesOption,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl TreeNode {
    pub fn new(
        name: String,
        path: String,
        nodes: NodesOption,
    ) -> Self {
        Self {
            name,
            path,
            nodes,
        }
    }

    pub fn find_node(&mut self, name: &str) -> Option<&mut TreeNode> {
        if let Some(nodes) = &mut self.nodes {
            for node in nodes.iter_mut() {
                if node.name == name {
                    return Some(node);
                }
            }
        }
        None
    }

    pub fn print_children(&self, level: u32) {
        let indent = "    ".repeat(level as usize);
        println!("{}Node:", indent);
        println!("{} value: {}", indent, self.name);
        println!("{} children:", indent);
        if let Some(children) = &self.nodes {
            for ch in children {
                println!("    {}", ch.name);
                match &ch.nodes {
                    Some(nodes) => {
                        for node in nodes {
                            node.print_children(level + 2);
                        }
                    },
                    None => {},
                }
            }
        }
    }

    pub fn add_node(&mut self, node: TreeNode) {
        if let Some(children) = &mut self.nodes {
            children.push(node);
        }
    }

    pub fn add_nodes(&mut self, nodes: Vec<TreeNode>) {
        if let Some(children) = &mut self.nodes {
            for (ch, node) in
                children.iter_mut().zip(nodes.into_iter())
            {
                match &mut ch.nodes {
                    Some(ch_nodes) => {
                        ch_nodes.push(node);
                    },
                    None => {},
                }
            }
        }
    }

    pub fn sort_nodes_alphabetically(head: &mut TreeNode) {
        if let Some(nodes) = &mut head.nodes {
            for node in nodes.iter_mut() {
                TreeNode::sort_nodes_alphabetically(node);
            }
            nodes.sort_by(|n1, n2| n1.name.cmp(&n2.name))
        }
    }

    fn _generate_summary(&self, level: usize) -> Vec<String> {
        let mut result = Vec::new();
        let indent = "    ".repeat(level);
        let line =
            format!("{}- [{}]({})", indent, self.name, self.path);
        // println!("{}", line);
        result.push(line);

        if let Some(nodes) = &self.nodes {
            for node in nodes {
                let indent = "    ".repeat(level + 1);
                let line = format!(
                    "{}- [{}]({})",
                    indent, node.name, node.path
                );
                // println!("{}", line);
                result.push(line);

                match &node.nodes {
                    Some(nodes) => {
                        for node in nodes {
                            result.extend(
                                node._generate_summary(level + 2),
                            );
                        }
                    },
                    None => {},
                }
            }
        }
        result
    }

    pub fn generate_summary(&self, level: usize) -> Vec<String> {
        let mut summary_vec =
            vec!["# Summary".to_string(), "".to_string()];
        if let Some(nodes) = &self.nodes {
            for node in nodes {
                summary_vec.extend(node._generate_summary(level));
            }
        }
        summary_vec
    }

    pub fn generate_summary_string(&self) -> String {
        let mut summary_string = String::new();
        let summary = self.generate_summary(0);
        for line in summary.iter() {
            let ll = line.clone() + "\n";
            summary_string.push_str(&ll);
        }
        summary_string
    }
}
