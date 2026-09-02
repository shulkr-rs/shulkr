use crate::{
    event::Event,
    util::{HashMap, RwLock},
};
use std::any::{Any, TypeId};

type Listener = Box<dyn Fn(&mut dyn Any) + Send + Sync>;

pub struct Events {
    listeners: RwLock<HashMap<TypeId, Vec<Listener>>>,
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::default()),
        }
    }

    pub fn subscribe<E, F>(&self, f: F) -> &Self
    where
        E: Event + 'static,
        F: Fn(&mut E) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let mut listeners = self.listeners.write();
        let listeners = listeners.entry(type_id).or_default();

        let wrapper = Box::new(move |event: &mut dyn Any| {
            if let Some(concrete_event) = event.downcast_mut::<E>() {
                f(concrete_event);
            }
        });

        listeners.push(wrapper);
        self
    }

    pub fn fire<E>(&self, event: &mut E)
    where
        E: Event + 'static,
    {
        let type_id = TypeId::of::<E>();
        let listeners = &self.listeners.read();
        if let Some(listeners) = listeners.get(&type_id) {
            for listener in listeners {
                listener(event);
            }
        }
    }
}
