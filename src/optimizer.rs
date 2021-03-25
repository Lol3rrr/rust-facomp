use crate::frontend::{self, ir::IRFunction};

mod const_prop;

pub fn optimize(ir: &mut std::collections::HashMap<String, IRFunction>) {
    // Do some optimization
    for (_, func) in ir.iter_mut() {
        const_prop::propagate(func);
    }

    println!("Optimized");
    frontend::ir::pretty_print(&ir);
}
