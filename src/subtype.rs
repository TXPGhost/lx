use crate::ir::*;

pub trait TypeRelation {
    fn is_subtype_of(&self, other: &Self) -> bool;
    fn is_supertype_of(&self, other: &Self) -> bool {
        other.is_subtype_of(self)
    }
    fn is_type_equal(&self, other: &Self) -> bool {
        self.is_subtype_of(other) && self.is_supertype_of(other)
    }
}

impl TypeRelation for Expr {
    fn is_subtype_of(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Struct(lhs), Expr::Struct(rhs)) => lhs.borrow().is_subtype_of(&rhs.borrow()),
            (Expr::Prim(lhs), Expr::Prim(rhs)) => lhs.is_subtype_of(rhs),
            (Expr::Func(lhs), Expr::Func(rhs)) => lhs.is_subtype_of(rhs),
            _ => false,
        }
    }
}

impl TypeRelation for Prim {
    fn is_subtype_of(&self, other: &Self) -> bool {
        match (self, other) {
            (Prim::I32(lhs), Prim::I32(rhs)) => lhs == rhs,
            (Prim::String(lhs), Prim::String(rhs)) => lhs == rhs,
            (Prim::Char(lhs), Prim::Char(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl TypeRelation for Struct {
    fn is_subtype_of(&self, other: &Self) -> bool {
        // must have same or more number of fields
        if self.fields.len() < other.fields.len() {
            return false;
        }

        // each field of `other` must exist in `self`, and it must be a subtype
        for (ident, other_expr) in &other.fields {
            match self.fields.get(ident) {
                Some(self_expr) => {
                    if !self_expr.is_subtype_of(other_expr) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }
}

impl TypeRelation for Func {
    fn is_subtype_of(&self, other: &Self) -> bool {
        // arguments must match
        if self.args.borrow().args.len() != other.args.borrow().args.len() {
            return false;
        }

        for ((_, (self_mut, self_arg)), (_, (other_mut, other_arg))) in self
            .args
            .borrow()
            .args
            .iter()
            .zip(other.args.borrow().args.iter())
        {
            // `(I32)String` is a subtype of `(*I32)String`, but
            // `(*I32)String` is _not_ a subtype of `(I32)String`
            if *self_mut && !other_mut {
                return false;
            }

            // function arguments are contravariant
            // so `(T)() <: (U)()` requires that `T :> U`
            if !self_arg.is_supertype_of(other_arg) {
                return false;
            }
        }

        // function body is covariant
        self.body.is_subtype_of(&other.body)
    }
}
