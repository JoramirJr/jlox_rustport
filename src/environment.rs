use std::collections::HashMap;

use crate::expr::ExpressionType;

struct Environment {
    map: HashMap<String, ExpressionType>,
}
