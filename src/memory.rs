use std::collections::HashMap;
use crate::types::VasoType;

#[derive(Clone, Debug)]
pub struct MemoryStack {
    scopes: Vec<HashMap<String, VasoType>>,
}

impl MemoryStack {
    pub fn new() -> Self {
        MemoryStack { scopes: vec![HashMap::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    // --- ESTA ES LA PARTE QUE ARREGLA EL ROLLBACK ---
    pub fn set(&mut self, name: String, val: VasoType) {
        // 1. Buscamos si la variable ya existe en algún scope superior (hacia atrás)
        // Si existe (como el saldo global), la actualizamos ahí mismo.
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, val);
                return; // ¡Actualizada! Salimos de la función.
            }
        }
        
        // 2. Si el bucle termina y no la encontramos, es una variable nueva.
        // La creamos en el scope actual.
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, val);
        }
    }
    // ------------------------------------------------

    // Mantenemos tu función global por seguridad
    pub fn set_global(&mut self, name: String, val: VasoType) {
        if let Some(scope) = self.scopes.first_mut() {
            scope.insert(name, val);
        }
    }

    pub fn get(&self, name: &str) -> Option<&VasoType> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val);
            }
        }
        None
    }
}