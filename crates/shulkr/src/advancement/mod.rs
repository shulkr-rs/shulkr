#![allow(unused)]

use crate::{item::ItemStack, text::TextComponent, util::Key};

#[derive(Debug, Clone)]
pub struct Advancement {
    id: Key,
    parent: Option<Key>,

    title: TextComponent,
    description: TextComponent,
    icon: ItemStack,
    frame: AdvancementFrame,
    background: Option<Key>,
    toast: bool,
    hidden: bool,
    x: f32,
    y: f32,

    aquired: bool,
    sends_telemetry_data: bool,
}

impl Advancement {
    pub fn builder(id: impl Into<Key>) -> AdvancementBuilder {
        AdvancementBuilder::new(id.into())
    }

    pub fn new(id: impl Into<Key>, parent: Option<&Advancement>) -> Self {
        let mut builder = Self::builder(id);
        if let Some(parent) = parent {
            builder = builder.with_parent(parent);
        }
        builder.build()
    }

    pub fn key(&self) -> &Key {
        &self.id
    }

    pub fn parent(&self) -> Option<&Key> {
        self.parent.as_ref()
    }

    pub fn set_parent(&mut self, parent: &Advancement) {
        self.parent = Some(parent.key().clone())
    }

    pub fn title(&self) -> &TextComponent {
        &self.title
    }

    pub fn description(&self) -> &TextComponent {
        &self.description
    }

    pub fn icon(&self) -> &ItemStack {
        &self.icon
    }

    pub fn frame(&self) -> AdvancementFrame {
        self.frame
    }

    pub fn has_toast(&self) -> bool {
        self.toast
    }

    pub fn show_toast(&mut self, value: bool) {
        self.toast = value;
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = value;
    }

    pub(crate) fn background(&self) -> Option<&Key> {
        self.background.as_ref()
    }

    pub(crate) fn flags(&self) -> i32 {
        let mut flags = 0;
        if self.background.is_some() {
            flags |= 0x01;
        }
        if self.has_toast() {
            flags |= 0x02;
        }
        if self.is_hidden() {
            flags |= 0x04;
        }
        flags
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn sends_telemetry_data(&self) -> bool {
        self.sends_telemetry_data
    }
}

pub struct AdvancementBuilder {
    id: Key,
    parent: Option<Key>,
    title: TextComponent,
    description: TextComponent,
    icon: ItemStack,
    frame: AdvancementFrame,
    background: Option<Key>,
    toast: bool,
    hidden: bool,
    x: f32,
    y: f32,
    telemetry: bool,
}

impl AdvancementBuilder {
    pub fn new(id: Key) -> Self {
        Self {
            id,
            parent: None,
            title: TextComponent::EMPTY,
            description: TextComponent::EMPTY,
            icon: ItemStack::EMPTY,
            frame: AdvancementFrame::Task,
            background: None,
            toast: false,
            hidden: false,
            x: 0.,
            y: 0.,
            telemetry: false,
        }
    }

    pub fn with_parent(mut self, parent: &Advancement) -> Self {
        self.parent = Some(parent.key().clone());
        self
    }

    pub fn with_title(mut self, title: impl Into<TextComponent>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<TextComponent>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_icon(mut self, icon: ItemStack) -> Self {
        self.icon = icon;
        self
    }

    pub fn with_frame(mut self, frame: AdvancementFrame) -> Self {
        self.frame = frame;
        self
    }

    pub fn toast(mut self, value: bool) -> Self {
        self.toast = value;
        self
    }

    pub fn hidden(mut self, value: bool) -> Self {
        self.hidden = value;
        self
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn build(self) -> Advancement {
        Advancement {
            id: self.id,
            parent: self.parent,
            title: self.title,
            description: self.description,
            icon: self.icon,
            frame: self.frame,
            background: self.background,
            toast: self.toast,
            hidden: self.hidden,
            x: self.x,
            y: self.y,
            aquired: false,
            sends_telemetry_data: self.telemetry,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdvancementFrame {
    Task,
    Challenge,
    Goal,
}

#[derive(Debug, Clone)]
pub struct AdvancementTree {
    background: Key,
    children: Vec<Advancement>,
}

impl AdvancementTree {
    pub fn new(background: Key) -> Self {
        Self {
            background,
            children: vec![],
        }
    }

    pub fn add(&self, advancement: Advancement) {}

    pub fn remove(&self, advancement: Advancement) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        advancement::{Advancement, AdvancementTree},
        util::Key,
    };

    #[test]
    fn test_advanements() {
        let tree = AdvancementTree::new(Key::new("a", "b"));

        let root = Advancement::builder("id")
            .hidden(true)
            .with_x(5.)
            .with_y(5.)
            .build();

        let advancement = Advancement::new("xy", Some(&root));

        tree.add(root);
        tree.add(advancement);
    }
}
