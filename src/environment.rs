use std::collections::HashMap;

use crate::token_type::LiteralType;

struct Environment {
    map: HashMap<String, Option<LiteralType>>,
}

impl Environment {
    fn define(&mut self, name: String, value: Option<LiteralType>) -> () {
        self.map.insert(name, value);
    }
}
