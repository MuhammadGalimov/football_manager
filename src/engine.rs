use std::collections::HashMap;
use std::rc::Rc;

pub struct Page {
    page_number: u32,
    w_number: u8,
    text_widgets: HashMap<u8, Text>,
    button_widgets: HashMap<u8, Button>,
}

impl Page {
    pub fn new(page_number: u32) -> Self {
        Page { 
            page_number, 
            w_number: 0, 
            text_widgets: HashMap::new(),
            button_widgets: HashMap::new(), 
        }
    }

    pub fn add_text_widget(&mut self, widget: Text) {
        self.text_widgets.insert(self.w_number, widget);
        self.w_number += 1;
    }

    pub fn add_button_widget(&mut self, widget: Button) {
        self.button_widgets.insert(self.w_number, widget);
        self.w_number += 1;
    }

    pub fn draw(&self) -> String {
        let mut s = String::from("");
        for (_, widget) in self.widgets.iter() {
            s.push_str(&(widget.draw() + "\n")[..]);
        }
        s
    }

    pub fn increase_index(&mut self) {

    }

    pub fn decrease_index(&mut self) {

    }
}

pub struct Book {
    pages: Vec<Page>,
    current_page_number: u32
}

impl Book {
    pub fn new(current_page_number: u32, pages: Vec<Page>) -> Self {
        Book { pages, current_page_number }
    }
}

pub struct Env { }

pub struct Text<'a> {
    text: &'a str,   
}

impl<'a> Text<'a> {
    fn draw(&self) -> String {
        self.text.to_string()
    }
    
    pub fn builder(text: &'a str) -> TextBuilder {
        TextBuilder::new(text)
    }
}

pub struct TextBuilder<'a> {
    text: Text<'a>
}

impl<'a> TextBuilder<'a> {
    pub fn new(text: &'a str) -> Self {
        TextBuilder { text: Text { text } }
    }

    pub fn build(self) -> Text<'a> {
        self.text
    }
}

pub struct Button<'a> {
    text: &'a str,
    tag: (&'a str, &'a str),
    tagged: bool,
    jump: fn(&mut Env) -> Option<u32>
}

impl<'a> Default for Button<'a> {
    fn default() -> Self {
        Button { 
            text: "", 
            tag: (">", "<"), 
            tagged: false, 
            jump: |_: &mut Env| { None } 
        }
    }
}

impl<'a> Button<'a> {
    fn draw(&self) -> String {
        if self.tagged {
            format!("{} {} {}", self.tag.0, self.text, self.tag.1)
        } else {
            format!("  {}  ", self.text)
        }
    }
    
    pub fn builder(text: &'a str) -> ButtonBuilder {
        ButtonBuilder::new(text)
    }
}

pub struct ButtonBuilder<'a> {
    button: Button<'a>
}

impl<'a> ButtonBuilder<'a> {
    pub fn new(text: &'a str) -> Self {
        ButtonBuilder { button: Button { text, ..Default::default() } }
    }

    pub fn jump(mut self, jump: fn(&mut Env) -> Option<u32>) -> Self {
        self.button.jump = jump;
        self
    }

    pub fn tag(mut self, tag: (&'a str, &'a str)) -> Self {
        self.button.tag = tag;
        self
    }

    pub fn tagged(mut self, tagged: bool) -> Self {
        self.button.tagged = tagged;
        self
    }

    pub fn build(self) -> Button<'a> {
        self.button
    }
}

// fn main() {
//     let mut page = Page::new(0);
//     page.add_widget(Box::new(Button { text: "New game", tagged: true, ..Default::default() }));
//     page.add_widget(Box::new(Button { text: "Continue", ..Default::default() }));
//     page.add_widget(Box::new(Button { text: "Exit", ..Default::default() }));

//     page.draw();
// }


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