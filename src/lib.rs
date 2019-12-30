//! Determine if a type does implement a logical trait
//! expression<sup>[**?**](#logical-trait-expression)</sup>.
//!
//! This library defines the [`does_impl!`], a macro<sup>[**?**](#macro)</sup>
//! that returns a [`bool`] indicating whether a type implements a boolean-like
//! expression over a set of traits<sup>[**?**](#trait)</sup>.
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! assert!(does_impl!(String: Clone & !Copy & Send & Sync));
//! ```
//!
//! See [examples](#examples) for detailed use cases.
//!
//! # Index
//!
//! - [Usage](#usage)
//! - [Vocabulary](#vocabulary)
//!   - [Macro](#macro)
//!   - [Trait](#trait)
//!   - [Logical Trait Expression](#logical-trait-expression)
//! - [Examples](#examples)
//!   - [Constant Evaluation](#constant-evaluation)
//!   - [Precedence and Nesting](#precedence-and-nesting)
//!   - [Mutual Exclusion](#mutual-exclusion)
//!   - [Reference Types](#reference-types)
//!   - [Unsized Types](#unsized-types)
//!   - [Generic Types](#generic-types)
//!   - [Lifetimes](#lifetimes)
//! - [License](#license)
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! does_impl = "1"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! # #[allow(unused_imports)]
//! #[macro_use]
//! extern crate does_impl;
//! # fn main() {}
//! ```
//!
//! When using [Rust 2018 edition][2018], the following import can help if
//! having `#[macro_use]` is undesirable.
//!
//! ```edition2018
//! use does_impl::does_impl;
//! ```
//!
//! # Vocabulary
//!
//! This documentation uses jargon that may be new to inexperienced Rust users.
//! This section exists to make these terms easier to understand. Feel free to
//! skip this section if these are already familiar to you.
//!
//! ## Macro
//!
//! In Rust, macros are functions over the [abstract syntax tree (AST)][AST].
//! They map input tokens to output tokens by performing some operation over
//! them through a set of rules. Because of this, only their outputs are ever
//! type-checked.
//!
//! If you wish to learn about implementing macros, I recommend:
//! - [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
//! - ["Macros" - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-06-macros.html)
//! - ["Macros" - The Rust Reference](https://doc.rust-lang.org/stable/reference/macros.html)
//! - ["Macros By Example" - The Rust Reference](https://doc.rust-lang.org/stable/reference/macros-by-example.html)
//!
//! To use this crate, you do not need to know how macros are defined.
//!
//! ## Trait
//!
//! In Rust, traits are a way of defining a generalized property. They should be
//! thought of expressing what a type is capable of doing. For example: if a
//! type implements [`Into`] for some type `T`, then we know it can be converted
//! into `T` by just calling the `.into()` method on it.
//!
//! If you wish to learn about traits in detail, I recommend:
//! - ["Traits: Defining Shared Behavior" - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html)
//! - ["Traits" - The Rust Reference](https://doc.rust-lang.org/stable/reference/items/traits.html)
//!
//! ## Logical Trait Expression
//!
//! In this crate, traits should be thought of as [`bool`]s where the condition
//! is whether the given type implements the trait or not.
//!
//! An expression can be formed from these trait operations:
//!
//! - And (`&`): also known as [logical disjunction], this returns `true` if
//!   **both** operands are `true`. This is usually defined in Rust via the
//!   [`BitAnd`] trait.
//!
//! - Or (`|`): also known as [logical conjunction], this returns `true` if
//!   **either** of two operands is `true`. This is usually defined in Rust via
//!   the [`BitOr`] trait.
//!
//! - Exclusive-or (`^`): also known as [exclusive disjunction], this returns
//!   `true` if **only one** of two operands is `true`. This is usually defined
//!   in Rust via the [`BitXor`] trait.
//!
//! - Not (`!`): a negation that returns `false` if the operand is `true`, or
//!   `true` if the operand is `false`. This is usually defined in Rust via the
//!   [`Not`] trait.
//!
//! See ["Precedence and Nesting"](#precedence-and-nesting) for information
//! about the order in which these operations are performed.
//!
// IMPORTANT: These examples are copy and pasted directly from `does_impl!`
//! # Examples
//!
//! This macro works in every type context. See below for use cases.
//!
//! ## Constant Evaluation
//!
//! Because types are [compile-time] constructs, the result of this macro can be
//! used as a `const` value:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! const DOES_IMPL: bool = does_impl!(u8: From<u32>);
//! ```
//!
//! Using [`static_assertions`], we can fail to compile if the trait expression
//! evaluates to `false`:
//!
//! ```compile_fail
//! # #[macro_use] extern crate does_impl;
//! # macro_rules! const_assert {
//! #     ($x:expr) => { let _: [(); 1] = [(); $x as usize]; }
//! # }
//! const_assert!(does_impl!(*const u8: Send | Sync));
//! ```
//!
//! ## Precedence and Nesting
//!
//! Trait operations abide by [Rust's expression precedence][precedence]. To
//! define a custom order of operations (e.g. left-to-right), simply nest the
//! expressions with parentheses.
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! let pre = does_impl!(u64:   From<u8> | From<u16>  ^ From<u32>  & From<u64>);
//! let ltr = does_impl!(u64: ((From<u8> | From<u16>) ^ From<u32>) & From<u64>);
//!
//! assert_eq!(pre, true | true ^ true & true);
//! assert_ne!(pre, ltr);
//! ```
//!
//! ## Mutual Exclusion
//!
//! Because exclusive-or (`^`) is a trait operation, we can check that a type
//! implements one of two traits, but not both:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! struct T;
//!
//! trait Foo {}
//! trait Bar {}
//!
//! impl Foo for T {}
//!
//! assert!(does_impl!(T: Foo ^ Bar));
//! ```
//!
//! ## Reference Types
//!
//! Something that's surprising to many Rust users is that [`&mut T`] _does not_
//! implement [`Copy`] _nor_ [`Clone`]:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! assert!(does_impl!(&mut u32: !Copy & !Clone));
//! ```
//!
//! Surely you're thinking now that this macro must be broken, because you've
//! been able to reuse `&mut T` throughout your lifetime with Rust. This works
//! because, in certain contexts, the compiler silently adds "re-borrows"
//! (`&mut *ref`) with a shorter lifetime and shadows the original. In reality,
//! `&mut T` is a move-only type.
//!
//! ## Unsized Types
//!
//! There's a variety of types in Rust that don't implement [`Sized`]:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! // Slices store their size with their pointer.
//! assert!(does_impl!(str:  !Sized));
//! assert!(does_impl!([u8]: !Sized));
//!
//! // Trait objects store their size in a vtable.
//! trait Foo {}
//! assert!(does_impl!(dyn Foo: !Sized));
//!
//! // Wrappers around unsized types are also unsized themselves.
//! struct Bar([u8]);
//! assert!(does_impl!(Bar: !Sized));
//! ```
//!
//! ## Generic Types
//!
//! When called from a generic function, the returned value is based on the
//! constraints of the generic type:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! use std::cell::Cell;
//!
//! struct Value<T> {
//!     // ...
//! #    value: T
//! }
//!
//! impl<T: Send> Value<T> {
//!     fn do_stuff() {
//!         assert!(does_impl!(Cell<T>: Send));
//!         // ...
//!     }
//! }
//! ```
//!
//! Keep in mind that this can result in false negatives:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! const fn is_copy<T>() -> bool {
//!     does_impl!(T: Copy)
//! }
//!
//! assert_ne!(is_copy::<u32>(), does_impl!(u32: Copy));
//! ```
//!
//! [precedence]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
//! [`static_assertions`]: https://docs.rs/static_assertions
//!
//! ## Lifetimes
//!
//! Traits with lifetimes are also supported:
//!
//! ```
//! # #[macro_use] extern crate does_impl;
//! trait Ref<'a> {}
//! impl<'a, T: ?Sized> Ref<'a> for &'a T {}
//! impl<'a, T: ?Sized> Ref<'a> for &'a mut T {}
//!
//! assert!(does_impl!(&'static str:      Ref<'static>));
//! assert!(does_impl!(&'static mut [u8]: Ref<'static>));
//! assert!(does_impl!(String:           !Ref<'static>));
//! ```
//!
//! # License
//!
//! This project is released under either:
//!
//! - [MIT License](https://github.com/nvzqz/does_impl/blob/master/LICENSE-MIT)
//! - [Apache License (Version 2.0)](https://github.com/nvzqz/does_impl/blob/master/LICENSE-APACHE)
//!
//! at your choosing.
//!
//! [compile-time]: https://en.wikipedia.org/wiki/Compile_time
//!
//! [`&mut T`]: https://doc.rust-lang.org/std/primitive.reference.html
//! [`Clone`]:  https://doc.rust-lang.org/std/marker/trait.Clone.html
//! [`Copy`]:   https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [`Sized`]:  https://doc.rust-lang.org/std/marker/trait.Sized.html
//!
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [`does_impl!`]: macro.does_impl.html
//! [2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
//! [crate]: https://crates.io/crates/does_impl
//!
//! [`BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
//! [`BitOr`]:  https://doc.rust-lang.org/std/ops/trait.BitOr.html
//! [`BitXor`]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
//! [`bool`]:   https://doc.rust-lang.org/std/primitive.bool.html
//! [`Into`]:   https://doc.rust-lang.org/std/convert/trait.Into.html
//! [`Not`]:    https://doc.rust-lang.org/std/ops/trait.Not.html
//!
//! [AST]:                   https://en.wikipedia.org/wiki/Abstract_syntax_tree
//! [exclusive disjunction]: https://en.wikipedia.org/wiki/Exclusive_disjunction
//! [logical conjunction]:   https://en.wikipedia.org/wiki/Logical_conjunction
//! [logical disjunction]:   https://en.wikipedia.org/wiki/Logical_disjunction

#![deny(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/nvzqz/does_impl/assets/logo.svg?sanitize=true")]

#[doc(hidden)]
pub extern crate core as _core;

/// Type-level booleans.
///
/// This module and its contents are not for public consumption and are thus
/// exempt from semantic versioning.
#[doc(hidden)]
#[path = "bool.rs"]
pub mod _bool;

/// Returns `true` if a type does implement a logical trait expression.
///
// IMPORTANT: Update crate level docs when updating these examples!
/// # Examples
///
/// This macro works in every type context. See below for use cases.
///
/// ## Constant Evaluation
///
/// Because types are [compile-time] constructs, the result of this macro can be
/// used as a `const` value:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// const DOES_IMPL: bool = does_impl!(u8: From<u32>);
/// ```
///
/// Using [`static_assertions`], we can fail to compile if the trait expression
/// evaluates to `false`:
///
/// ```compile_fail
/// # #[macro_use] extern crate does_impl;
/// # macro_rules! const_assert {
/// #     ($x:expr) => { let _: [(); 1] = [(); $x as usize]; }
/// # }
/// const_assert!(does_impl!(*const u8: Send | Sync));
/// ```
///
/// ## Precedence and Nesting
///
/// Trait operations abide by [Rust's expression precedence][precedence]. To
/// define a custom order of operations (e.g. left-to-right), simply nest the
/// expressions with parentheses.
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// let pre = does_impl!(u64:   From<u8> | From<u16>  ^ From<u32>  & From<u64>);
/// let ltr = does_impl!(u64: ((From<u8> | From<u16>) ^ From<u32>) & From<u64>);
///
/// assert_eq!(pre, true | true ^ true & true);
/// assert_ne!(pre, ltr);
/// ```
///
/// ## Mutual Exclusion
///
/// Because exclusive-or (`^`) is a trait operation, we can check that a type
/// implements one of two traits, but not both:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// struct T;
///
/// trait Foo {}
/// trait Bar {}
///
/// impl Foo for T {}
///
/// assert!(does_impl!(T: Foo ^ Bar));
/// ```
///
/// ## Reference Types
///
/// Something that's surprising to many Rust users is that [`&mut T`] _does not_
/// implement [`Copy`] _nor_ [`Clone`]:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// assert!(does_impl!(&mut u32: !Copy & !Clone));
/// ```
///
/// Surely you're thinking now that this macro must be broken, because you've
/// been able to reuse `&mut T` throughout your lifetime with Rust. This works
/// because, in certain contexts, the compiler silently adds "re-borrows"
/// (`&mut *ref`) with a shorter lifetime and shadows the original. In reality,
/// `&mut T` is a move-only type.
///
/// ## Unsized Types
///
/// There's a variety of types in Rust that don't implement [`Sized`]:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// // Slices store their size with their pointer.
/// assert!(does_impl!(str:  !Sized));
/// assert!(does_impl!([u8]: !Sized));
///
/// // Trait objects store their size in a vtable.
/// trait Foo {}
/// assert!(does_impl!(dyn Foo: !Sized));
///
/// // Wrappers around unsized types are also unsized themselves.
/// struct Bar([u8]);
/// assert!(does_impl!(Bar: !Sized));
/// ```
///
/// ## Generic Types
///
/// When called from a generic function, the returned value is based on the
/// constraints of the generic type:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// use std::cell::Cell;
///
/// struct Value<T> {
///     // ...
/// #    value: T
/// }
///
/// impl<T: Send> Value<T> {
///     fn do_stuff() {
///         assert!(does_impl!(Cell<T>: Send));
///         // ...
///     }
/// }
/// ```
///
/// Keep in mind that this can result in false negatives:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// const fn is_copy<T>() -> bool {
///     does_impl!(T: Copy)
/// }
///
/// assert_ne!(is_copy::<u32>(), does_impl!(u32: Copy));
/// ```
///
/// [precedence]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
/// [`static_assertions`]: https://docs.rs/static_assertions
///
/// ## Lifetimes
///
/// Traits with lifetimes are also supported:
///
/// ```
/// # #[macro_use] extern crate does_impl;
/// trait Ref<'a> {}
/// impl<'a, T: ?Sized> Ref<'a> for &'a T {}
/// impl<'a, T: ?Sized> Ref<'a> for &'a mut T {}
///
/// assert!(does_impl!(&'static str:      Ref<'static>));
/// assert!(does_impl!(&'static mut [u8]: Ref<'static>));
/// assert!(does_impl!(String:           !Ref<'static>));
/// ```
///
/// [compile-time]: https://en.wikipedia.org/wiki/Compile_time
///
/// [`&mut T`]: https://doc.rust-lang.org/std/primitive.reference.html
/// [`Clone`]:  https://doc.rust-lang.org/std/marker/trait.Clone.html
/// [`Copy`]:   https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`Sized`]:  https://doc.rust-lang.org/std/marker/trait.Sized.html
#[macro_export(local_inner_macros)]
macro_rules! does_impl {
    ($type:ty: $($trait_expr:tt)+) => {
        _does_impl!($type: $($trait_expr)+)
    };
}

/// Handles the dirty work of `does_impl`.
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! _does_impl {
    // ONE: Turn `$trait` into `true` or `false` based on whether `$type`
    // implements it.
    ($type:ty: $(! !)* $trait:path) => {{
        // Do not import types in order to prevent trait name collisions.

        /// Fallback trait with `False` for `DOES_IMPL` if the type does not
        /// implement the given trait.
        trait DoesNotImpl {
            const DOES_IMPL: $crate::_bool::False = $crate::_bool::False;
        }
        impl<T: ?Sized> DoesNotImpl for T {}

        /// Concrete type with `True` for `DOES_IMPL` if the type does implement
        /// the given trait. Otherwise, it falls back to `DoesNotImpl`.
        struct Wrapper<T: ?Sized>($crate::_core::marker::PhantomData<T>);

        #[allow(dead_code)]
        impl<T: ?Sized + $trait> Wrapper<T> {
            const DOES_IMPL: $crate::_bool::True = $crate::_bool::True;
        }

        <Wrapper<$type>>::DOES_IMPL.value()
    }};

    // NOT
    ($type:ty: $(! !)* !$trait:path) => {
        !_does_impl!($type: $trait)
    };

    // PAREN
    ($type:ty: $(! !)* ($($trait_expr:tt)+)) => {
        _does_impl!($type: $($trait_expr)+)
    };
    // PAREN+NOT
    ($type:ty: $(! !)* !($($trait_expr:tt)+)) => {
        !_does_impl!($type: $($trait_expr)+)
    };
    // PAREN+OR
    ($type:ty: $(! !)* ($($t1:tt)+) | $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        |
        _does_impl!($type: $($t2)+)
    };
    // PAREN+OR+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) | $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        |
        _does_impl!($type: $($t2)+)
    };
    // PAREN+AND
    ($type:ty: $(! !)* ($($t1:tt)+) & $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        &
        _does_impl!($type: $($t2)+)
    };
    // PAREN+AND+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) & $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        &
        _does_impl!($type: $($t2)+)
    };
    // PAREN+XOR
    ($type:ty: $(! !)* ($($t1:tt)+) ^ $($t2:tt)+) => {
        _does_impl!($type: $($t1)+)
        ^
        _does_impl!($type: $($t2)+)
    };
    // PAREN+XOR+NOT
    ($type:ty: $(! !)* !($($t1:tt)+) ^ $($t2:tt)+) => {
        !_does_impl!($type: $($t1)+)
        ^
        _does_impl!($type: $($t2)+)
    };

    // OR: Any.
    ($type:ty: $(! !)* $t1:path | $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        |
        _does_impl!($type: $($t2)+)
    }};
    // OR+NOT: Any.
    ($type:ty: $(! !)* !$t1:path | $($t2:tt)+) => {{
        !_does_impl!($type: $t1)
        |
        _does_impl!($type: $($t2)+)
    }};

    // AND: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* $t1:ident & $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* !$t1:ident & $($t2:tt)+) => {{
        !_does_impl!($type: $t1)
        &
        _does_impl!($type: $($t2)+)
    }};

    // AND: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        !$t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        &
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        &
        _does_impl!($type: $($t2)+)
    }};

    // AND: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_generic),+ >)
        &
        _does_impl!($type: $($t2)+)
    }};
    // AND+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        !$t1:ident < $($t1_generic:ty),+ $(,)? >
        &
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_generic),+ >)
        &
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* $t1:ident ^ $($t2:tt)+) => {{
        _does_impl!($type: $t1)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 0 lifetimes, 0 generics.
    ($type:ty: $(! !)* !$t1:ident ^ $($t2:tt)+) => {{
        ! _does_impl!($type: $t1)
        ^
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 1+ lifetimes, 0+ generics.
    (
        $type:ty: $(! !)*
        ! $t1:ident < $($t1_lifetime:lifetime),+ $(, $t1_generic:ty)* $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        !_does_impl!($type: $t1 < $($t1_lifetime),+ $(, $t1_generic)* >)
        ^
        _does_impl!($type: $($t2)+)
    }};

    // XOR: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        _does_impl!($type: $t1 < $($t1_generic),+ >)
        ^
        _does_impl!($type: $($t2)+)
    }};
    // XOR+NOT: 0 lifetimes, 1+ generics.
    (
        $type:ty: $(! !)*
        ! $t1:ident < $($t1_generic:ty),+ $(,)? >
        ^
        $($t2:tt)+
    ) => {{
        ! _does_impl!($type: $t1 < $($t1_generic),+ >)
        ^
        _does_impl!($type: $($t2)+)
    }};
}

// Declare after macros in order to be able to use them.
#[cfg(test)]
mod tests;