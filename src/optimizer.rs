use crate::frontend::{self, ir::IRFunction};

mod const_prop;

pub fn optimize(ir: &mut Vec<IRFunction>) {
    println!("Unoptimizied");
    frontend::ir::pretty_print(&ir);

    // Do some optimization
    for func in ir.iter_mut() {
        const_prop::propagate(func);
    }

    println!("Optimized");
    frontend::ir::pretty_print(&ir);
}
