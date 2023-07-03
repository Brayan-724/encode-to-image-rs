#[macro_export()]
macro_rules! impl_op {
    ($op_trait:ident, $op_fn:ident($self:ident: $self_t:ty, $rhs:ident:$rhs_t:ty) -> $out:ty $body:block) => {
        impl ::std::ops::$op_trait<$rhs_t> for $self_t {
            type Output = $out;

            fn $op_fn(self, $rhs: $rhs_t) -> Self::Output {
                let $self = self;
                $body
            }
        }
    };

    ($op_trait:ident, $op_fn:ident(mut $self:ident: $self_t:ty, $rhs:ident:$rhs_t:ty) $body:block) => {
        impl ::std::ops::$op_trait<$rhs_t> for $self_t {
            fn $op_fn(&mut self, $rhs: $rhs_t) {
                let mut $self = self;
                $body
            }
        }
    };

    (+ $($_:tt)+) => { $crate::impl_op!(Add, add $($_)+); };
    (- $($_:tt)+) => { $crate::impl_op!(Sub, sub $($_)+); };
    (* $($_:tt)+) => { $crate::impl_op!(Mul, mul $($_)+); };
    (/ $($_:tt)+) => { $crate::impl_op!(Div, div $($_)+); };
    (% $($_:tt)+) => { $crate::impl_op!(Rem, rem $($_)+); };
    (| $($_:tt)+) => { $crate::impl_op!(BitOr, bitor $($_)+); };
    (& $($_:tt)+) => { $crate::impl_op!(BitAnd, bitand $($_)+); };
    (^ $($_:tt)+) => { $crate::impl_op!(BitXor, bitxor $($_)+); };
    (>> $($_:tt)+) => { $crate::impl_op!(Shr, shr $($_)+); };
    (<< $($_:tt)+) => { $crate::impl_op!(Shl, shl $($_)+); };

    (+= $($_:tt)+) => { $crate::impl_op!(AddAssign, add_assign $($_)+); };
    (-= $($_:tt)+) => { $crate::impl_op!(SubAssign, sub_assign $($_)+); };
    (*= $($_:tt)+) => { $crate::impl_op!(MulAssign, mul_assign $($_)+); };
    (/= $($_:tt)+) => { $crate::impl_op!(DivAssign, div_assign $($_)+); };
    (%= $($_:tt)+) => { $crate::impl_op!(RemAssign, rem_assign $($_)+); };
    (|= $($_:tt)+) => { $crate::impl_op!(BitOrAssign, bitor_assign $($_)+); };
    (&= $($_:tt)+) => { $crate::impl_op!(BitAndAssign, bitand_assign $($_)+); };
    (^= $($_:tt)+) => { $crate::impl_op!(BitXorAssign, bitxor_assign $($_)+); };
    (>>= $($_:tt)+) => { $crate::impl_op!(ShrAssign, shr_assign $($_)+); };
    (<<= $($_:tt)+) => { $crate::impl_op!(ShlAssign, shl_assign $($_)+); };
}

#[macro_export()]
macro_rules! macro_if {
    (if ($($x:tt)+) {$($if_true:tt)+} else {$($false:tt)*}) => {
        $($if_true)+
    };
    (if ($($x:tt)+) else {$($false:tt)*}) => {
        $($x)+
    };
    (if () {$($true:tt)*} else {$($false:tt)+}) => {
        $($false)+
    };
    (if () else {$($false:tt)+}) => {
        $($false)+
    }
}

#[macro_export()]
macro_rules! impl_ops {
    ($op_trait:ident, $op_fn:ident($self:ident: $self_t:path , $rhs:ident) $(-> $out:ty)? $body:block) => {
        $crate::impl_op!($op_trait,
            $op_fn($self: $self_t, $rhs: $self_t) -> $crate::macro_if!(if ($($out)?) else { $self_t })
            $body
        );
        $crate::impl_op!($op_trait,
            $op_fn($self: $self_t, $rhs: &$self_t) -> $crate::macro_if!(if ($($out)?) else { $self_t })
            $body
        );
        $crate::impl_op!($op_trait,
            $op_fn($self: &$self_t, $rhs: &$self_t) -> $crate::macro_if!(if ($($out)?) else { $self_t })
            $body
        );
        $crate::impl_op!($op_trait,
            $op_fn($self: &$self_t, $rhs: $self_t) -> $crate::macro_if!(if ($($out)?) else { $self_t })
            $body
        );
    };

    ($op_trait:ident, $op_fn:ident($self:ident: &$self_t:path , $rhs:ident) $(-> $out:ty)? $body:block) => {
        compile_error!(stringify!(Consider remove the borrowing: $self: $self_t));
    };

    ($op_trait:ident, $op_fn:ident(mut $self:ident : &$self_t:path, $rhs:ident) $body:block) => {
        compile_error!(stringify!(Consider remove the borrowing: mut $self: $self_t));
    };

    ($op_trait:ident, $op_fn:ident(mut $self:ident : $self_t:path, $rhs:ident) $body:block) => {
        $crate::impl_op!($op_trait, $op_fn(mut $self: $self_t, $rhs: $self_t) $body);
        $crate::impl_op!($op_trait, $op_fn(mut $self: $self_t, $rhs: &$self_t) $body);
    };

    (+ $($_:tt)+) => { $crate::impl_ops!(Add, add $($_)+); };
    (- $($_:tt)+) => { $crate::impl_ops!(Sub, sub $($_)+); };
    (* $($_:tt)+) => { $crate::impl_ops!(Mul, mul $($_)+); };
    (/ $($_:tt)+) => { $crate::impl_ops!(Div, div $($_)+); };
    (% $($_:tt)+) => { $crate::impl_ops!(Rem, rem $($_)+); };
    (| $($_:tt)+) => { $crate::impl_ops!(BitOr, bitor $($_)+); };
    (& $($_:tt)+) => { $crate::impl_ops!(BitAnd, bitand $($_)+); };
    (^ $($_:tt)+) => { $crate::impl_ops!(BitXor, bitxor $($_)+); };
    (>> $($_:tt)+) => { $crate::impl_ops!(Shr, shr $($_)+); };
    (<< $($_:tt)+) => { $crate::impl_ops!(Shl, shl $($_)+); };

    (+= $($_:tt)+) => { $crate::impl_ops!(AddAssign, add_assign $($_)+); };
    (-= $($_:tt)+) => { $crate::impl_ops!(SubAssign, sub_assign $($_)+); };
    (*= $($_:tt)+) => { $crate::impl_ops!(MulAssign, mul_assign $($_)+); };
    (/= $($_:tt)+) => { $crate::impl_ops!(DivAssign, div_assign $($_)+); };
    (%= $($_:tt)+) => { $crate::impl_ops!(RemAssign, rem_assign $($_)+); };
    (|= $($_:tt)+) => { $crate::impl_ops!(BitOrAssign, bitor_assign $($_)+); };
    (&= $($_:tt)+) => { $crate::impl_ops!(BitAndAssign, bitand_assign $($_)+); };
    (^= $($_:tt)+) => { $crate::impl_ops!(BitXorAssign, bitxor_assign $($_)+); };
    (>>= $($_:tt)+) => { $crate::impl_ops!(ShrAssign, shr_assign $($_)+); };
    (<<= $($_:tt)+) => { $crate::impl_ops!(ShlAssign, shl_assign $($_)+); };
}
