use std::any::type_name;

fn main() {
    let a = closure_builder(1);
    let b = closure_builder(2);
    let c = closure_builder(3);
}

fn closure_builder(divisor: usize) -> impl Fn(usize) -> bool {
    let my_closure = {
        move |x: usize| if x % divisor == 0 { true } else { false }
    };
    my_closure
}