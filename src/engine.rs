use std::collections::HashMap;

struct Node {
    id: String,
    parent_id: String,
    text: String,
    children: Vec<String>
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}\nparent_id: {}\ntext: {}\namount of children: {}",
            self.id, self.parent_id, self.text, self.children.len())
    }
}

impl Node {
    fn new(id: &str, parent_id: &str, text: &str) -> Self {
        Node { 
            id: id.to_string(),
            parent_id: parent_id.to_string(), 
            text: text.to_string(), 
            children: Vec::new() 
        }
    }

    fn add_child(&mut self, child_id: &str) {
        self.children.push(child_id.to_string());
    }

    fn children(&self) -> Vec<String> {
        self.children.clone()
    }

    fn text(&self) -> String {
        self.text.clone()
    }

    fn parent_id(&self) -> String {
        self.parent_id.clone()
    }
}

pub struct Glob {
    nodes: HashMap<String, Node>,
    point: String,
    index: usize,
}

impl Glob {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(String::from("root"), Node::new(
            "root",
            "",
            ""
        ));

        Glob { nodes, point: String::from("root"), index: 0}
    }

    fn current_node(&self) -> &Node {
        self.nodes.get(&self.point[..]).unwrap()
    }

    pub fn add_node(&mut self, parent_id: &str, id: &str, text: &str) -> Option<()> {
        match self.nodes.get_mut(parent_id) {
            Some(parent_node) => {
                parent_node.add_child(id);
                self.nodes.insert(id.to_string(), Node::new(id, parent_id, text));
                Some(())
            }
            None => { None }
        }
    }

    pub fn view(&self) -> String {
        let node = self.current_node();
        let mut output = String::new();
        for (i, child_id) in node.children().iter().enumerate() {
            let text = match self.nodes.get(child_id) {
                Some(child_node) => { child_node.text() },
                None => { return "error".to_string(); }
            };

            let brs = if i == self.index {
                (">", "<\n")
            } else {
                (" ", " \n")
            };
                    
            output.push_str(brs.0);
            output.push_str(&text[..]);
            output.push_str(brs.1);
        }

        output
    }

    pub fn increase_index(&mut self) {
        let max_index = self.current_node().children().len();
        
        if self.index + 1 == max_index {
            self.index = 0;
        } else {
            self.index = self.index + 1;
        }
    }

    pub fn decrease_index(&mut self) {
        let max_index = self.current_node().children().len();
        
        if self.index == 0 {
            self.index = max_index - 1;
        } else {
            self.index = self.index - 1;
        }
    }

    pub fn enter(&mut self) {
        let node = self.current_node();
        let need = node.children()[self.index].clone();
        self.point = need;
        self.index = 0;
    }

    pub fn back(&mut self) {
        let node = self.current_node();
        self.point = node.parent_id();
        self.index = 0;
    }
}