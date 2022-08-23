use std::collections::HashMap;

/// Essentially a drop trait, but it's called right the game resources are destroyed.
/// This way we can have joint ownership over graphics objects such as textures, framebuffers, shaders, and models.
/// Each destroyable resource should be given this trait.
/// Must have a mutible reference to the object and this should never be called manually.
/// Add each resource to the global resources hashmap, and it will handle this.
pub trait ResourceDestroy {
    fn destroy(&mut self);
}

/// Structure responsible for managing the resources of a specific type T.
/// This type can be anything, but the engine should give whatever types it supports
/// to clints during the loading process.
pub struct ResourceManager<T: ResourceDestroy> {
    resource_type_name: String,
    registries: HashMap<String, T>
}

/// When the resources are destroyed, call the resource destroy function on each loaded object.
impl<T: ResourceDestroy> Drop for ResourceManager<T> {
    fn drop(&mut self) {
        for (_, registry) in self.registries.iter_mut() {
            registry.destroy();
        }
    }
}

impl<T: ResourceDestroy> ResourceManager<T> {
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
        match self.registries.get(name) {
            Some(registry) => Some(registry),
            None => {
                println!("Unable to find registry ({}) in {}", name, self.resource_type_name);
                None
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.resource_type_name
    }
}