use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{nodes::FunctionDeclarationNode, values::Value};

pub type ScopePtr = Rc<RefCell<Scope>>;

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopePtr>,
    fn_table: HashMap<String, Rc<RefCell<FunctionDeclarationNode>>>,
    var_table: HashMap<String, Value>,
}

impl Scope {
    pub fn new(parent: ScopePtr) -> ScopePtr {
        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            fn_table: HashMap::new(),
            var_table: HashMap::new(),
        }))
    }
    pub fn from(
        var_table: HashMap<String, Value>,
        fn_table: HashMap<String, Rc<RefCell<FunctionDeclarationNode>>>,
    ) -> ScopePtr {
        Rc::new(RefCell::new(Scope {
            parent: None,
            fn_table,
            var_table,
        }))
    }
    pub fn set_parent(&mut self, parent: ScopePtr) {
        self.parent = Some(parent);
    }

    pub fn set_symbol(&mut self, key: &String, value: Value) {
        if self.var_table.contains_key(key) {
            self.var_table.insert(key.to_owned(), value);
        } else if let Some(_parent) = self.parent.clone() {
            _parent.borrow_mut().set_symbol(key, value);
        } else {
            panic!("Symbol '{key}' not found")
        }
    }

    pub fn declare_symbol(&mut self, key: String, value: Value) {
        self.var_table.insert(key, value);
    }

    pub fn get_symbol(&self, key: &String) -> Value {
        match self.var_table.get(key) {
            Some(val) => val.clone(),
            None => {
                if let Some(_parent) = self.parent.clone() {
                    return _parent.borrow().get_symbol(key).clone();
                } else {
                    panic!("Symbol '{key}' not found")
                }
            }
        }
    }

    pub fn declare_function(&mut self, key: String, value: Rc<RefCell<FunctionDeclarationNode>>) {
        self.fn_table.insert(key, value);
    }

    pub fn get_function(&self, key: &String) -> Rc<RefCell<FunctionDeclarationNode>> {
        match self.fn_table.get(key) {
            Some(val) => val.clone(),
            None => {
                if let Some(_parent) = self.parent.clone() {
                    return _parent.borrow().get_function(key).clone();
                } else {
                    panic!("Function '{key}' not found")
                }
            }
        }
    }
}
