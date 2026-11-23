use std::collections::HashMap;

pub struct AssetStorage<T, H> {
    assets: HashMap<u32, T>,
    next_id: u32,
    _marker: std::marker::PhantomData<H>,
}
#[allow(dead_code)]
impl<T, H> AssetStorage<T, H> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            next_id: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn insert(&mut self, asset: T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.assets.insert(id, asset);
        id
    }

    pub fn get(&self, id: u32) -> Option<&T> {
        self.assets.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.assets.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<T> {
        self.assets.remove(&id)
    }
}