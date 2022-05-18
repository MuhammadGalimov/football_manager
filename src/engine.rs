use std::ops::{Deref, DerefMut};
use super::env::Env;

pub struct Page {
    page_number: usize,
    w_number: usize,
    index: Option<usize>,
    text_widgets: Vec<OrdWidget<Text>>,
    button_widgets: Vec<OrdWidget<Button>>,
}

// todo: add action button, without env
// todo: where should be save function?

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
                .find(|&item| item.number() == i) 
                {
                s.push_str(&(w.draw() + "\n")[..]);
                continue;
            }

            if let Some(w) = self.button_widgets
                .iter()
                .find(|&item| item.number() == i) 
                {
                s.push_str(&(w.draw() + "\n")[..]);
                continue;
            }
        }

        s
    }

    pub fn increase_index(&mut self) {
        if let Some(index) = self.index {
            if let Some(position) = self.button_widgets.iter().position(|item| item.number() == index) {
                self.button_widgets[position].tagged = false;
                
                if position + 1 == self.button_widgets.len() {
                    self.button_widgets[0].tagged = true;
                    self.index = Some(self.button_widgets[0].number());
                } else {
                    self.button_widgets[position + 1].tagged = true;
                    self.index = Some(self.button_widgets[position + 1].number());
                }
            }
        }   
    }

    pub fn decrease_index(&mut self) {
        if let Some(index) = self.index {
            if let Some(position) = self.button_widgets.iter().position(|item| item.number() == index) {
                self.button_widgets[position].tagged = false;
                
                if position == 0 {
                    if let Some(w) = self.button_widgets.iter_mut().last() {
                        w.tagged = true;
                        self.index = Some(w.number());
                    }
                } else {
                    self.button_widgets[position - 1].tagged = true;
                    self.index = Some(self.button_widgets[position - 1].number());
                }
            }
        }
    }

    pub fn enter(&self, env: &mut Env) -> Option<usize> {
        if let Some(index) = self.index {
            if let Some(position) = self.button_widgets.iter().position(|item| item.number() == index) {
                let new_page_number = (self.button_widgets[position].jump)(env);
                return new_page_number;
            } else {
                return None;
            }
        } else {
            return None;
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

    fn number(&self) -> usize {
        self.number
    }
}

impl<T> Deref for OrdWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl<T> DerefMut for OrdWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

pub struct Book {
    pages: Vec<Page>,
    current_page_number: usize,
}

impl Book {
    pub fn new(current_page_number: usize, pages: Vec<Page>) -> Self {
        Book { pages, current_page_number }
    }

    fn get_page(&self, page_number: usize) -> Option<&Page> {
        if let Some(position) = self.pages
            .iter()
            .position(|item| item.page_number == page_number) {
                return Some(&self.pages[position])
        } else {
            return None;
        }
    }

    fn get_page_mut(&mut self, page_number: usize) -> Option<&mut Page> {
        if let Some(position) = self.pages
            .iter()
            .position(|item| item.page_number == page_number) {
                return Some(&mut self.pages[position])
        } else {
            return None;
        }
    }

    pub fn draw(&self) -> String {
        self.get_page(self.current_page_number).unwrap().draw()
    }

    pub fn increase_index(&mut self) {
        self.get_page_mut(self.current_page_number).unwrap().increase_index();
    }

    pub fn decrease_index(&mut self) {
        self.get_page_mut(self.current_page_number).unwrap().decrease_index();
    }

    pub fn enter(&mut self, env: &mut Env) {
        let new_page_number = self.get_page_mut(self.current_page_number).unwrap().enter(env);

        match new_page_number {
            Some(n) => {
                self.current_page_number = n;
            },
            None => {}
        }
    }
}

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
    jump: fn(&mut Env) -> Option<usize>
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
    jump: fn(&mut Env) -> Option<usize>
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

    pub fn jump(mut self, jump: fn(&mut Env) -> Option<usize>) -> Self {
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

pub trait ToButton {
    fn to_button(&self, jump: fn(&mut Env) -> Option<usize>) -> Button;
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