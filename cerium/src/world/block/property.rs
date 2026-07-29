pub struct Property {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

impl Property {
    pub fn value_count(&self) -> u16 {
        self.values.len() as u16
    }
}
