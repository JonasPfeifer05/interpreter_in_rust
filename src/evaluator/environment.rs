use std::collections::HashMap;
use nanoid::nanoid;
use crate::evaluator::object::{OwnerShip};

#[derive(Default)]
pub struct Environment {
    stack: Stack,
    heap: Heap
}

impl Environment {
    pub fn get(&self, identifier: &str) -> Option<&OwnerShip> {
        let stack_obj = self.stack.get(identifier);
        if stack_obj.is_some() { return stack_obj }

        self.heap.get(identifier)
    }

    pub fn get_mut(&mut self, identifier: &str) -> Option<&mut OwnerShip> {
        let stack_obj = self.stack.get_mut(identifier);
        if stack_obj.is_some() { return stack_obj }

        self.heap.get_mut(identifier)
    }


    pub fn stack(&self) -> &Stack {
        &self.stack
    }
    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }
}

pub struct Stack {
    values: Vec<(String, OwnerShip)>,
    scope_pointers: Vec<usize>,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            values: vec![],
            scope_pointers: vec![0],
        }
    }
}

impl Stack {
    pub fn create_scope(&mut self) {
        self.scope_pointers.push(self.values.len());
    }
    pub fn drop_scope(&mut self) {
        unsafe { self.values.set_len(self.scope_pointers.pop().unwrap()); }
    }

    pub fn add(&mut self, identifier: String, value: OwnerShip) {
        self.values.push((identifier, value));
    }

    pub fn get(&self, identifier: &str) -> Option<&OwnerShip> {
        self.values.iter()
            .rev()
            .find(|x| x.0 == identifier)
            .map(|x| &x.1)
    }

    pub fn get_mut(&mut self, identifier: &str) -> Option<&mut OwnerShip> {
        self.values.iter_mut()
            .rev()
            .find(|x| x.0 == identifier)
            .map(|x| &mut x.1)
    }
}

#[derive(Default)]
pub struct Heap {
    memory: HashMap<String, OwnerShip>,
}

impl Heap {
    pub fn get(&self, address: &str) -> Option<&OwnerShip> {
        self.memory.get(address)
    }

    pub fn get_mut(&mut self, address: &str) -> Option<&mut OwnerShip> {
        self.memory.get_mut(address)
    }

    pub fn set(&mut self, value: OwnerShip) -> String {
        let address = nanoid!(20);
        self.memory.insert(address.clone(), value);
        address
    }

    pub fn deallocate(&mut self, address: &str) {
        self.memory.remove(address);
    }
}

// pub struct Stack {
//     scopes: Vec<Scope>,
// }
//
// impl Stack {
//     pub fn create_scope(&mut self) { self.scopes.push(Scope::default()) }
//     pub fn drop_scope(&mut self) { self.scopes.pop(); }
//
//     pub fn add(&mut self, identifier: String, value: OwnerShip) {
//         self.scopes.last_mut().unwrap().set(identifier, value);
//     }
//
//     pub fn get(&self, identifier: &str) -> Option<&OwnerShip> {
//         self.scopes.iter()
//             .rev()
//             .filter(|x| x.get(identifier).is_some())
//             .map(|x| x.get(identifier).unwrap())
//             .next()
//     }
//
//     pub fn get_mut(&mut self, identifier: &str) -> Option<&mut OwnerShip> {
//         self.scopes.iter_mut()
//             .rev()
//             .filter(|x| x.get(identifier).is_some())
//             .map(|x| x.get_mut(identifier).unwrap())
//             .next()
//     }
// }
//
// impl Default for Stack {
//     fn default() -> Self {
//         Self {
//             scopes: vec![Scope::default()]
//         }
//     }
// }
//
// #[derive(Default)]
// pub struct Scope {
//     identifiers: HashMap<String, OwnerShip>,
// }
//
// impl Scope {
//     pub fn get(&self, identifier: &str) -> Option<&OwnerShip> {
//         self.identifiers.get(identifier)
//     }
//
//     pub fn get_mut(&mut self, identifier: &str) -> Option<&mut OwnerShip> {
//         self.identifiers.get_mut(identifier)
//     }
//
//     pub fn set(&mut self, identifier: String, value: OwnerShip) {
//         self.identifiers.insert(identifier, value);
//     }
// }