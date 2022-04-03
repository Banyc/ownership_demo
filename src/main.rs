struct S {
    x: i32,
}

fn fn_immut_move_prim(a: i32) -> i32 {
    // pass `a` by value
    // a = a;  // consider making this binding mutable: `mut a`
    a
}
fn fn_immut_move_obj(a: S) -> S {
    // pass `a` by value or reference
    // a.x = a.x;  // consider changing this to be mutable: `mut a`
    // a = a;  // consider making this binding mutable: `mut a`
    a
}
fn fn_mut_move_prim(mut a: i32) -> i32 {
    // pass `a` by value
    a = a;
    a
}
fn fn_mut_move_obj(mut a: S) -> S {
    // pass `a` by value or reference
    a.x = a.x;
    a = a;
    a
}
fn fn_immut_borrow_prim(a: &i32) -> i32 {
    // a = a;  // consider making this binding mutable: `mut a`
    *a
}
fn fn_immut_borrow_obj(a: &S) -> &S {
    // a.x = a.x;  // consider changing this to be a mutable reference: `&mut S`
    // a = a;  // consider making this binding mutable: `mut a`
    a
}
fn fn_mut_borrow_prim(a: &mut i32) -> i32 {
    // a = a;  // consider making this binding mutable: `mut a`
    *a
}
fn fn_mut_borrow_obj(a: &mut S) -> &S {
    a.x = a.x;
    // a = a;  // consider making this binding mutable: `mut a`
    a
}

fn main() {
    println!("Hello, world!");

    {
        let immut_prim = 1;
        let _ = fn_immut_move_prim(immut_prim);  // move: don't care parameter mutability; pass by value
        let _ = fn_mut_move_prim(immut_prim);  // move: don't care parameter mutability; pass by value
        let _ = fn_immut_borrow_prim(&immut_prim);
        // let _ = fn_mut_borrow_prim(&mut immut_prim);  // consider changing this to be mutable: `mut immut_prim`
    }
    {
        {
            let immut_obj = S { x: 1 };
            let _ = fn_immut_move_obj(immut_obj);  // move: don't care parameter mutability; pass by value or reference
        }
        {
            let immut_obj = S { x: 1 };
            let _ = fn_mut_move_obj(immut_obj);  // move: don't care parameter mutability; pass by value or reference
        }
        {
            let immut_obj = S { x: 1 };
            let _ = fn_immut_borrow_obj(&immut_obj);
            // let _ = fn_mut_borrow_obj(&mut immut_obj);  // consider changing this to be mutable: `mut immut_obj`
        }
    }
    {
        let mut mut_prim = 1;
        let _ = fn_immut_move_prim(mut_prim);  // move: don't care parameter mutability; pass by value
        let _ = fn_mut_move_prim(mut_prim);  // move: don't care parameter mutability; pass by value
        let _ = fn_immut_borrow_prim(&mut_prim);
        let _ = fn_mut_borrow_prim(&mut mut_prim);
    }
    {
        {
            let mut mut_obj = S { x: 1 };  // warn: variable does not need to be mutable
            let _ = fn_immut_move_obj(mut_obj);  // move: don't care parameter mutability; pass by value or reference
        }
        {
            let mut mut_obj = S { x: 1 };  // warn: variable does not need to be mutable
            let _ = fn_mut_move_obj(mut_obj);  // move: don't care parameter mutability; pass by value or reference
        }
        {
            let mut mut_obj = S { x: 1 };
            let _ = fn_immut_borrow_obj(&mut_obj);
            let _ = fn_mut_borrow_obj(&mut mut_obj);
        }
    }
}
