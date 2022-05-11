pub struct Page {
    page_number: usize,
    w_number: usize,
    index: Option<usize>,
    text_widgets: Vec<OrdWidget<Text>>,
    button_widgets: Vec<OrdWidget<Button>>,
}

impl Page {
    pub fn new(page_number: usize) -> Self {
        Page { 
            page_number, 
            w_number: 0,
            index: None, 
            text_widgets: vec![],
            button_widgets: vec![], 
        }
    }

    pub fn add_text_widget(&mut self, widget: Text) {
        self.text_widgets.push(OrdWidget::new(self.w_number, widget));
        self.w_number += 1;
    }

    pub fn add_button_widget(&mut self, widget: Button) {
        self.button_widgets.push(OrdWidget::new(self.w_number, widget));

        if self.index == None {
            self.index = Some(self.w_number);
        }

        self.w_number += 1;
    }

    pub fn draw(&self) -> String {
        let mut s = String::from("");

        for i in 0..self.w_number {
            if let Some(w) = self.text_widgets
                .iter()
                .find(|&item| item.number == i) 
                {
                s.push_str(&(w.widget.draw() + "\n")[..]);
                continue;
            }
        }

        for i in 0..self.w_number {
            if let Some(w) = self.button_widgets
                .iter()
                .find(|&item| item.number == i) 
                {
                s.push_str(&(w.widget.draw() + "\n")[..]);
                continue;
            }
        }

        s
    }

    pub fn increase_index(&mut self) {
        if let Some(index) = self.index {
            if let Some(position) = self.button_widgets.iter().position(|item| item.number == index) {
                self.button_widgets[position].widget.tagged = false;
                
                if position + 1 == self.button_widgets.len() {
                    self.button_widgets[0].widget.tagged = true;
                    self.index = Some(self.button_widgets[0].number);
                } else {
                    self.button_widgets[position + 1].widget.tagged = true;
                    self.index = Some(self.button_widgets[position + 1].number);
                }
            }
        }   
    }

    pub fn decrease_index(&mut self) {
        if let Some(index) = self.index {
            if let Some(position) = self.button_widgets.iter().position(|item| item.number == index) {
                self.button_widgets[position].widget.tagged = false;
                
                if position == 0 {
                    if let Some(w) = self.button_widgets.iter_mut().last() {
                        w.widget.tagged = true;
                        self.index = Some(w.number);
                    };
                } else {
                    self.button_widgets[position - 1].widget.tagged = true;
                    self.index = Some(self.button_widgets[position - 1].number);
                }
            }
        }
    }
}

pub struct OrdWidget<T> {
    number: usize,
    widget: T
}

impl<T> OrdWidget<T> {
    fn new(number: usize, widget: T) -> Self {
        OrdWidget { number, widget }
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

pub struct Text {
    text: String,   
}

impl Text {
    fn draw(&self) -> String {
        self.text.clone()
    }
}

pub struct TextBuilder {
    text: String,
}

impl TextBuilder {
    pub fn new(text: &str) -> Self {
        TextBuilder { text: text.to_string() }
    }

    pub fn build(self) -> Text {
        Text { text: self.text }
    }
}

pub struct Button {
    text: String,
    tag: (String, String),
    tagged: bool,
    jump: fn(&mut Env) -> Option<u32>
}

impl Button {
    fn draw(&self) -> String {
        if self.tagged {
            format!("{} {} {}", self.tag.0, self.text, self.tag.1)
        } else {
            format!("  {}  ", self.text)
        }
    }
}

pub struct ButtonBuilder {
    text: String,
    tag: (String, String),
    tagged: bool,
    jump: fn(&mut Env) -> Option<u32>
}

impl ButtonBuilder {
    pub fn new(text: &str) -> Self {
        ButtonBuilder {
            text: text.to_string(),
            tag: (">".to_string(), "<".to_string()), 
            tagged: false, 
            jump: |_: &mut Env| { None }
        }
    }

    pub fn jump(mut self, jump: fn(&mut Env) -> Option<u32>) -> Self {
        self.jump = jump;
        self
    }

    pub fn tag(mut self, tag: (&str, &str)) -> Self {
        self.tag = (tag.0.to_string(), tag.1.to_string());
        self
    }

    pub fn tagged(mut self, tagged: bool) -> Self {
        self.tagged = tagged;
        self
    }

    pub fn build(self) -> Button {
        Button { 
            text: self.text, 
            tag: self.tag, 
            tagged: self.tagged, 
            jump: self.jump 
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let mut h: Vec<(u32, String)> = vec![];
        h.push((0, "one".to_string()));
        h.push((2, "two".to_string()));
        h.push((5, "three".to_string()));
        h.push((6, "four".to_string()));
        h.push((10, "five".to_string()));

        for (k, v) in h.iter() {
            println!("key: {}, value: {}", k, v);
        }

        println!("------");

        for (k, v) in h.iter().find(|item| item.0 == 5) {
            println!("key: {}, value: {}", k, v);
        }

    }
}