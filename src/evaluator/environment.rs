use std::collections::HashMap;
use crate::evaluator::object::Object;

pub struct Environment {
    scopes: Vec<Scope>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::default()]
        }
    }
}

impl Environment {
    pub fn create_scope(&mut self) { self.scopes.push(Scope::default()) }
    pub fn drop_scope(&mut self) { self.scopes.pop(); }

    pub fn put(&mut self, identifier: String, value: Object) {
        self.scopes.last_mut().unwrap().set(identifier, value);
    }

    pub fn search(&self, identifier: &str) -> Option<&Object> {
        self.scopes.iter()
            .rev()
            .filter(|x| x.get(identifier).is_some())
            .map(|x| x.get(identifier).unwrap())
            .next()
    }

    pub fn search_mut(&mut self, identifier: &str) -> Option<&mut Object> {
        self.scopes.iter_mut()
            .rev()
            .filter(|x| x.get(identifier).is_some())
            .map(|x| x.get_mut(identifier).unwrap())
            .next()
    }
}

#[derive(Default)]
pub struct Scope {
    identifiers: HashMap<String, Object>,
}

impl Scope {
    pub fn get(&self, identifier: &str) -> Option<&Object> {
        self.identifiers.get(identifier)
    }

    pub fn get_mut(&mut self, identifier: &str) -> Option<&mut Object> {
        self.identifiers.get_mut(identifier)
    }

    pub fn set(&mut self, identifier: String, value: Object) {
        self.identifiers.insert(identifier, value);
    }
}