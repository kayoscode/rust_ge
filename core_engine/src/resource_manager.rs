use std::collections::HashMap;

/// Structure responsible for managing the resources of a specific type T.
/// This type can be anything, but the engine should give whatever types it supports
/// to clints during the loading process.
#[derive(Default)]
pub struct ResourceManager<T> {
    resource_type_name: String,
    registries: HashMap<String, T>
}

impl<T> ResourceManager<T> {
    pub fn new(resource_type_name: &str) -> Self {
        ResourceManager {
            resource_type_name: resource_type_name.to_string(),
            registries: HashMap::<String, T>::default()
        }
    }

    pub fn add_registry(&mut self, name: &str, registry: T) {
        self.registries.insert(name.to_string(), registry);
    }

    pub fn get_registry(&self, name: &str) -> Option<&T> {
        self.registries.get(name)
    }

    pub fn get_name(&self) -> &str {
        &self.resource_type_name
    }
}