#![feature(const_generics)] // for stability declarations on `[T; N]`
#![feature(decl_macro)] // for stub implementations of derives
#![feature(never_type)] // for stability declarations on `!`
#![feature(const_fn, const_panic)] // for const free functions
#![feature(marker_trait_attr)] // for cast extension
#![cfg_attr(doc, feature(staged_api))] // for `unstable` attribute
#![feature(optin_builtin_traits)] // for `mem` gadgets demo
#![feature(vec_into_raw_parts)] // for vec casting demo
#![feature(impl_trait_in_bindings)]
#![allow(unused_unsafe, incomplete_features)]
#![deny(missing_docs)]

//! Rustdoc for the API surface proposed by the [safer transmute RFC](https://github.com/rust-lang/project-safe-transmute/pull/5).
//!
//! New modules and new items within existing modules are marked with **🌟**. Items associated with RFC extensions are marked unstable.
//!
//! <style>h2#macros, h2#macros + table, a[href="#macros"] { display: none!important; }</style>

/// Additions to libcore.
pub mod core {

    /// Traits for conversions between types.
    pub mod convert {
        #[doc(no_inline)]
        pub use core::convert::*;

        use transmute::*;

        /// **🌟** **Bit-preserving conversions.**
        ///
        /// Transmutation is the act of reinterpreting the bytes corresponding to a value of one type as if they corresponded to a different type. A transmutation of a `Src` to a `Dst` type is similar to defining a union with two variants:
        /// ```rust
        /// #![feature(untagged_unions)]
        /// union Transmute<Src, Dst> {
        ///     src: ManuallyDrop<Src>,
        ///     dst: ManuallyDrop<Dst>,
        /// }
        /// ```
        /// And instantiating that union with its `src` variant, then reading `dst` back out. The [TransmuteFrom] and [TransmuteInto] traits are implemented *automatically* for all `Src` and `Dst` types where it is *sound*, *safe*, and *stable* to do this conversion:
        ///  - A transmutation is ***sound*** if the mere act of transmuting a value from one type to another is not compiler undefined behavior.
        ///  - A sound transmutation is ***safe*** if *using* the transmuted value cannot violate memory safety.
        ///  - A safe transmutation is ***stable*** if the authors of the source type and destination types [have indicated that the layouts of those types is part of their libraries' stability guarantees][stability].
        ///
        /// For more information on these concepts [**see here**](https://github.com/jswrenn/project-safe-transmute/blob/rfc/rfcs/0000-safe-transmute.md#concepts-in-depth).
        ///
        /// ## Example
        /// Given:
        /// ```rust
        /// use core::convert::transmute::{
        ///     TransmuteInto,
        ///     stability::{PromiseTransmutableInto, PromiseTransmutableFrom},
        /// };
        ///
        /// #[derive(PromiseTransmutableInto, PromiseTransmutableFrom)]
        /// #[repr(C)]
        /// pub struct Foo(pub u8, pub u16);
        /// //                    ^ there's a padding byte here, between these fields
        /// ```
        /// This transmutation accepted:
        /// ```rust
        /// let _ : Foo = 64u32.transmute_into(); // Alchemy Achieved!
        /// //                  ^^^^^^^^^^^^^^ provided by the `TransmuteInto` trait
        /// ```
        /// But this transmutation is rejected:
        /// ```compile_fail
        /// let _ : u32 = Foo(16, 12).transmute_into();
        /// // error[E0277]: the trait bound `u32: TransmuteFrom<Foo, _>` is not satisfied
        /// //   --> src/demo.rs:15:27
        /// //    |
        /// // 15 | let _ : u32 = Foo(16, 12).transmute_into();
        /// //    |                           ^^^^^^^^^^^^^^ the trait `TransmuteFrom<foo::Foo, _>` is not implemented for `u32`
        /// //    |
        /// //   = note: required because of the requirements on the impl of `TransmuteInto<u32, _>` for `foo::Foo`
        /// ```
        pub mod transmute {
            use {options::*, stability::*};

            /// Reinterprets the bits of a value of one type as another type, safely.
            #[inline(always)]
            pub const fn safe_transmute<Src, Dst, Neglect>(src: Src) -> Dst
            where
                Src: TransmuteInto<Dst, Neglect>,
                Neglect: SafeTransmuteOptions
            {
                unimplemented!()
            }

            /// Reinterprets the bits of a value of one type as another type, potentially unsafely.
            #[inline(always)]
            pub const unsafe fn unsafe_transmute<Src, Dst, Neglect>(src: Src) -> Dst
            where
                Src: TransmuteInto<Dst, Neglect>,
                Neglect: TransmuteOptions
            {
                unimplemented!()
            }

            /// Reinterpret the bits of `Self` as a type `Dst`.
            ///
            /// The reciprocal of [TransmuteFrom].
            ///
            /// ***This trait is implemented automatically by the compiler for combinations of types where a transmutation is valid.***
            pub unsafe trait TransmuteInto<Dst: ?Sized, Neglect = ()>
            where
                Neglect: TransmuteOptions,
            {
                /// Reinterpret the bits of a value of one type as another type, safely.
                fn transmute_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: SafeTransmuteOptions;

                /// Reinterpret the bits of a value of one type as another type, potentially unsafely.
                ///
                /// The onus is on you to ensure that calling this method is safe.
                unsafe fn unsafe_transmute_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: TransmuteOptions;
            }

            unsafe impl<Src, Dst, Neglect> TransmuteInto<Dst, Neglect> for Src
            where
                Src: ?Sized,
                Dst: ?Sized + TransmuteFrom<Src, Neglect>,
                Neglect: TransmuteOptions,
            {
                #[inline(always)]
                fn transmute_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: SafeTransmuteOptions,
                {
                    Dst::transmute_from(self)
                }

                #[inline(always)]
                unsafe fn unsafe_transmute_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: TransmuteOptions,
                {
                    unsafe { Dst::unsafe_transmute_from(self) }
                }
            }

            /// Reinterpret the bits of `Src` as a type `Self`.
            ///
            /// The reciprocal of [TransmuteFrom].
            ///
            /// ***This trait is implemented automatically by the compiler for combinations of types where a transmutation is valid.***
            pub unsafe trait TransmuteFrom<Src: ?Sized, Neglect = ()>
            where
                Neglect: TransmuteOptions,
            {
                /// Reinterpret the bits of a value of one type as another type, safely.
                #[inline(always)]
                fn transmute_from(src: Src) -> Self
                where
                    Src: Sized,
                    Self: Sized,
                    Neglect: SafeTransmuteOptions,
                {
                    use core::{mem, ptr};
                    unsafe {
                        let dst = ptr::read(&src as *const Src as *const Self);
                        mem::forget(src);
                        dst
                    }
                }

                /// Reinterpret the bits of a value of one type as another type, potentially unsafely.
                ///
                /// The onus is on you to ensure that calling this function is safe.
                #[inline(always)]
                unsafe fn unsafe_transmute_from(src: Src) -> Self
                where
                    Src: Sized,
                    Self: Sized,
                    Neglect: TransmuteOptions,
                {
                    use core::{mem, ptr};
                    unsafe {
                        let dst = ptr::read_unaligned(&src as *const Src as *const Self);
                        mem::forget(src);
                        dst
                    }
                }
            }

            #[doc(hidden)]
            unsafe impl<T> TransmuteFrom<T, NeglectStability> for T {}

            /// A type `Dst` is [stably][stability] transmutable from `Src` if:
            ///  - `Dst` implements [PromiseTransmutableFrom][trait@PromiseTransmutableFrom], 
            ///  - `Src` implements [PromiseTransmutableInto][trait@PromiseTransmutableInto], and
            ///  - The [PromiseTransmutableFrom::Archetype] of `Dst` is safely transmutable from the [PromiseTransmutableInto::Archetype] of `Src`.
            unsafe impl<Src, Dst> TransmuteFrom<Src> for Dst
            where
                Src: PromiseTransmutableInto,
                Dst: PromiseTransmutableFrom,
                <Dst as PromiseTransmutableFrom>::Archetype:
                    TransmuteFrom<
                        <Src as PromiseTransmutableInto>::Archetype,
                        NeglectStability
                    >
            {}

            /// Traits for declaring the SemVer stability of a type's layout.
            ///
            /// Since the soundness and safety of a transmutation is affected by the layouts of the source and destination types, changes to those types' layouts may cause code which previously compiled to produce errors. In other words, transmutation causes a type's layout to become part of that type's API for the purposes of SemVer stability.
            /// 
            /// To promise that all transmutations which are currently safe for your type will remain so in the future, simply annotate your type with:
            /// ```rust
            /// #[derive(PromiseTransmutableFrom, PromiseTransmutableInto)]
            /// #[repr(C)]
            /// pub struct Foo(pub Bar, pub Baz);
            /// ```
            ///
            /// For more information on stability, [**see here**](https://github.com/jswrenn/project-safe-transmute/blob/rfc/rfcs/0000-safe-transmute.md#-when-is-a-transmutation-stable).
            pub mod stability {

                use super::{TransmuteFrom, TransmuteInto, options::NeglectStability};

                /// Promise that a type may be stably transmuted *into* other types.
                ///
                /// To promise that all safe transmutations from your type into other `PromiseTransmutableFrom` types will remain safe in the future, simply annotate your type with `#[derive(PromiseTransmutableFrom)]`; e.g.:
                /// ```rust
                /// #[derive(PromiseTransmutableFrom)]
                /// #[repr(C)]
                /// pub struct Foo(pub Bar, pub Baz);
                /// ```
                pub trait PromiseTransmutableInto
                {
                    /// A type which exemplifies the greatest extent to which `Self` might change in non-breaking crate releases, insofar that those changes might affect converting `Self` into another type via transmutation. 
                    type Archetype
                        : TransmuteFrom<Self, NeglectStability>
                        + PromiseTransmutableInto;
                }

                /// Promise that a type may be stably transmuted *from* other types.
                ///
                /// To promise that all transmutations of any `PromiseTransmutableInto` type into your type that are currently safe will remain so in the future, simply annotate your type with `#[derive(PromiseTransmutableFrom)]`.
                /// 
                /// For instance, this:
                /// ```rust
                /// #[derive(PromiseTransmutableFrom)]
                /// #[repr(C)]
                /// pub struct Foo(pub Bar, pub Baz);
                /// ```
                /* #[lang = "promise_transmutable_from"] */
                pub trait PromiseTransmutableFrom
                {
                    /// A type which exemplifies the greatest extent to which `Self` might change in non-breaking crate releases, insofar that those changes might affect instantiating `Self` via transmutation. 
                    type Archetype
                        : TransmuteInto<Self, NeglectStability>
                        + PromiseTransmutableFrom;
                }

                #[doc(hidden)]
                mod macros {
                    use super::*;

                    /// Derive macro generating an impl of the trait [PromiseTransmutableFrom][trait@PromiseTransmutableFrom].
                    ///
                    /// To promise that all safe transmutations from your type into other `PromiseTransmutableFrom` types will remain safe in the future, simply annotate your type with `#[derive(PromiseTransmutableFrom)]`.
                    /// 
                    /// For instance, this:
                    /// ```rust
                    /// #[derive(PromiseTransmutableFrom)]
                    /// #[repr(C)]
                    /// pub struct Foo(pub Bar, pub Baz);
                    /// ```
                    /// will expand to this:
                    /// ```rust
                    /// /// Generated `PromiseTransmutableInto` for `Foo`
                    /// const _: () = {
                    ///     use core::convert::transmute::stability::PromiseTransmutableInto;
                    /// 
                    ///     #[repr(C)]
                    ///     pub struct TransmutableIntoArchetype(
                    ///         pub <Bar as PromiseTransmutableInto>::Archetype,
                    ///         pub <Baz as PromiseTransmutableInto>::Archetype,
                    ///     );
                    /// 
                    ///     impl PromiseTransmutableInto for TransmutableIntoArchetype { type Archetype = Self };
                    /// 
                    ///     impl PromiseTransmutableInto for Foo {
                    ///         type Archetype = TransmutableIntoArchetype;
                    ///     }
                    /// };
                    /// ```
                    pub macro PromiseTransmutableInto($item:item) {
                        /* compiler built-in */
                    }

                    /// Derive macro generating an impl of the trait [PromiseTransmutableFrom][trait@PromiseTransmutableFrom].
                    ///
                    /// To promise that all transmutations of any `PromiseTransmutableInto` type into your type that are currently safe will remain so in the future, simply annotate your type with `#[derive(PromiseTransmutableFrom)]`.
                    /// 
                    /// For instance, this:
                    /// ```rust
                    /// #[derive(PromiseTransmutableFrom)]
                    /// #[repr(C)]
                    /// pub struct Foo(pub Bar, pub Baz);
                    /// ```
                    /// will expand to this:
                    /// ```rust
                    /// /// Generated `PromiseTransmutableFrom` for `Foo`
                    /// const _: () = {
                    ///     use core::convert::transmute::stability::PromiseTransmutableFrom;
                    /// 
                    ///     #[repr(C)]
                    ///     pub struct TransmutableFromArchetype(
                    ///         pub <Bar as PromiseTransmutableFrom>::Archetype,
                    ///         pub <Baz as PromiseTransmutableFrom>::Archetype,
                    ///     );
                    /// 
                    ///     impl PromiseTransmutableFrom for TransmutableFromArchetype { type Archetype = Self };
                    /// 
                    ///     impl PromiseTransmutableFrom for Foo {
                    ///         type Archetype = TransmutableFromArchetype;
                    ///     }
                    /// };
                    /// ```
                    pub macro PromiseTransmutableFrom($item:item) {
                        /* compiler built-in */
                    }

                    /// Derive macro generating impls of *both* [PromiseTransmutableFrom][trait@PromiseTransmutableFrom] and [PromiseTransmutableInto][trait@PromiseTransmutableInto].
                    ///
                    /// This is just a shorthand for deriving both [PromiseTransmutableFrom!] and [PromiseTransmutableInto!].
                    ///
                    /// For more information on this extension, [**see here**](https://github.com/jswrenn/project-safe-transmute/blob/rfc/rfcs/0000-safe-transmute.md#extension-promisetransmutable-shorthand).
                    #[cfg_attr(doc, unstable(feature = "stability_shorthand", issue = "none"))]
                    pub macro PromiseTransmutable($item:item) {
                        /* compiler built-in */
                    }
                }

                #[doc(inline)]
                pub use macros::{
                    PromiseTransmutableInto,
                    PromiseTransmutableFrom,
                    PromiseTransmutable,
                };

                impl PromiseTransmutableInto for     ! {type Archetype = Self;}
                impl PromiseTransmutableFrom for     ! {type Archetype = Self;}

                impl PromiseTransmutableInto for    () {type Archetype = Self;}
                impl PromiseTransmutableFrom for    () {type Archetype = Self;}

                impl PromiseTransmutableInto for   f32 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   f32 {type Archetype = Self;}
                impl PromiseTransmutableInto for   f64 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   f64 {type Archetype = Self;}

                impl PromiseTransmutableInto for    i8 {type Archetype = Self;}
                impl PromiseTransmutableFrom for    i8 {type Archetype = Self;}
                impl PromiseTransmutableInto for   i16 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   i16 {type Archetype = Self;}
                impl PromiseTransmutableInto for   i32 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   i32 {type Archetype = Self;}
                impl PromiseTransmutableInto for   i64 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   i64 {type Archetype = Self;}
                impl PromiseTransmutableInto for  i128 {type Archetype = Self;}
                impl PromiseTransmutableFrom for  i128 {type Archetype = Self;}
                impl PromiseTransmutableInto for isize {type Archetype = Self;}
                impl PromiseTransmutableFrom for isize {type Archetype = Self;}

                impl PromiseTransmutableInto for    u8 {type Archetype = Self;}
                impl PromiseTransmutableFrom for    u8 {type Archetype = Self;}
                impl PromiseTransmutableInto for   u16 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   u16 {type Archetype = Self;}
                impl PromiseTransmutableInto for   u32 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   u32 {type Archetype = Self;}
                impl PromiseTransmutableInto for   u64 {type Archetype = Self;}
                impl PromiseTransmutableFrom for   u64 {type Archetype = Self;}
                impl PromiseTransmutableInto for  u128 {type Archetype = Self;}
                impl PromiseTransmutableFrom for  u128 {type Archetype = Self;}
                impl PromiseTransmutableInto for usize {type Archetype = Self;}
                impl PromiseTransmutableFrom for usize {type Archetype = Self;}

                use core::marker::PhantomData;
                impl<T: ?Sized> PromiseTransmutableInto for PhantomData<T> { type Archetype = Self; }
                impl<T: ?Sized> PromiseTransmutableFrom for PhantomData<T> { type Archetype = Self; }


                impl<T, const N: usize> PromiseTransmutableInto for [T; N]
                where
                    T: PromiseTransmutableInto,
                    [T::Archetype; N]
                        : TransmuteFrom<Self, NeglectStability>
                        + PromiseTransmutableInto,
                {
                    type Archetype = [T::Archetype; N];
                }

                impl<T, const N: usize> PromiseTransmutableFrom for [T; N]
                where
                    T: PromiseTransmutableFrom,
                    [T::Archetype; N]
                        : TransmuteInto<Self, NeglectStability>
                        + PromiseTransmutableFrom,
                {
                    type Archetype = [T::Archetype; N];
                }


                impl<T: ?Sized> PromiseTransmutableInto for *const T
                where
                    T: PromiseTransmutableInto,
                    *const T::Archetype
                        : TransmuteFrom<Self, NeglectStability>
                        + PromiseTransmutableInto,
                {
                    type Archetype = *const T::Archetype;
                }

                impl<T: ?Sized> PromiseTransmutableFrom for *const T
                where
                    T: PromiseTransmutableFrom,
                    *const T::Archetype
                        : TransmuteInto<Self, NeglectStability>
                        + PromiseTransmutableFrom,
                {
                    type Archetype = *const T::Archetype;
                }


                impl<T: ?Sized> PromiseTransmutableInto for *mut T
                where
                    T: PromiseTransmutableInto,
                    *mut T::Archetype
                        : TransmuteFrom<Self, NeglectStability>
                        + PromiseTransmutableInto,
                {
                    type Archetype = *mut T::Archetype;
                }

                impl<T: ?Sized> PromiseTransmutableFrom for *mut T
                where
                    T: PromiseTransmutableFrom,
                    *mut T::Archetype
                        : TransmuteInto<Self, NeglectStability>
                        + PromiseTransmutableFrom,
                {
                    type Archetype = *mut T::Archetype;
                }


                impl<'a, T: ?Sized> PromiseTransmutableInto for &'a T
                where
                    T: PromiseTransmutableInto,
                    &'a T::Archetype
                        : TransmuteFrom<&'a T, NeglectStability>
                        + PromiseTransmutableInto,
                {
                    type Archetype = &'a T::Archetype;
                }

                impl<'a, T: ?Sized> PromiseTransmutableFrom for &'a T
                where
                    T: PromiseTransmutableFrom,
                    &'a T::Archetype
                        : TransmuteInto<&'a T, NeglectStability>
                        + PromiseTransmutableFrom,
                {
                    type Archetype = &'a T::Archetype;
                }

                impl<'a, T: ?Sized> PromiseTransmutableInto for &'a mut T
                where
                    T: PromiseTransmutableInto,
                    &'a mut T::Archetype
                        : TransmuteFrom<&'a mut T, NeglectStability>
                        + PromiseTransmutableInto,
                {
                    type Archetype = &'a mut T::Archetype;
                }

                impl<'a, T: ?Sized> PromiseTransmutableFrom for &'a mut T
                where
                    T: PromiseTransmutableFrom,
                    &'a mut T::Archetype
                        : TransmuteInto<&'a mut T, NeglectStability>
                        + PromiseTransmutableFrom,
                {
                    type Archetype = &'a mut T::Archetype;
                }
            }

            /// Static checks that may be neglected when determining if two types are transmutable.
            ///
            /// The default value of the `Neglect` parameter of [TransmuteFrom] and [TransmuteInto], `()`, statically forbids transmutes that are unsafe, unsound, or unstable. However, you may explicitly opt-out of some static checks:
            /// 
            /// | Transmute Option    | Compromises | Usable With                                             |
            /// |---------------------|-------------|---------------------------------------------------------|
            /// | [NeglectStability]   | Stability   | `transmute_{from,into}`, `unsafe_transmute_{from,into}` |
            /// | [NeglectAlignment]  | Safety      | `unsafe_transmute_{from,into}`                          |
            /// | [NeglectValidity]   | Soundness   | `unsafe_transmute_{from,into}`                          |
            /// 
            /// The selection of multiple options is encoded by grouping them as a tuple; e.g., `(NeglectAlignment, NeglectValidity)` is a selection of both the [NeglectAlignment] and [NeglectValidity] options.
            pub mod options {
                use super::*;

                /// Options that may be used with safe transmutations.
                pub trait SafeTransmuteOptions: TransmuteOptions
                {}

                /// Options that may be used with unsafe transmutations.
                pub trait TransmuteOptions: private::Sealed
                {}

                impl SafeTransmuteOptions for () {}
                impl TransmuteOptions for () {}

                /// Neglect the static stability check.
                ///
                /// By default, [TransmuteFrom] and [TransmuteInto] require that the [layouts of the source and destination types are SemVer-stable][super::stability]. The [NeglectStability] option disables this requirement.
                ///
                /// Prior to the adoption of the [stability declaration traits][super::stability], crate authors documented the layout guarantees of their types with doc comments. The [TransmuteFrom] and [TransmuteInto] traits and methods may be used with these types by requesting that the stability check is neglected; for instance:
                /// 
                /// ```rust
                /// fn serialize<W: Write>(val : LibraryType, dst: W) -> std::io::Result<()>
                /// where
                ///     LibraryType: TransmuteInto<[u8; size_of::<LibraryType>()], NeglectStability>
                /// {
                ///     ...
                /// }
                /// ```
                /// 
                /// Neglecting stability over-eagerly cannot cause unsoundness or unsafety. For this reason, it is the only transmutation option available on the safe methods `transmute_from` and `transmute_into`. However, neglecting stability over-eagerly may cause your code to cease compiling if the authors of the source and destination types make changes that affect their layout.
                /// 
                /// By using the `NeglectStability` option to transmute types you do not own, you are committing to ensure that your reliance on these types' layouts is consistent with their documented stability guarantees.
                pub struct NeglectStability;

                // Uncomment this if/when constructibility is fully implemented:
                impl SafeTransmuteOptions for NeglectStability {}
                impl TransmuteOptions for NeglectStability {}

                /// Neglect the static alignment check.
                ///
                /// By default, [TransmuteFrom] and [TransmuteInto] are only implemented for references when the minimum alignment of the destination's referent type is no greater than the minimum alignment of the source's referent type. The `NeglectAlignment` option disables this requirement.
                /// 
                /// By using the `NeglectAlignment` option, you are committing to ensure that the transmuted reference satisfies the alignment requirements of the destination's referent type. For instance:
                /// ```rust
                /// /// Try to convert a `&T` into `&U`.
                /// ///
                /// /// This produces `None` if the referent isn't appropriately
                /// /// aligned, as required by the destination type.
                /// pub fn try_cast_ref<'t, 'u, T, U>(src: &'t T) -> Option<&'u U>
                /// where
                ///     &'t T: TransmuteInto<&'u U, NeglectAlignment>,
                /// {
                ///     if (src as *const T as usize) % align_of::<U>() != 0 {
                ///         None
                ///     } else {
                ///         // Safe because we dynamically enforce the alignment
                ///         // requirement, whose static check we chose to neglect.
                ///         Some(unsafe { src.unsafe_transmute_into() })
                ///     }
                /// }
                /// ```
                pub struct NeglectAlignment;
                impl TransmuteOptions for NeglectAlignment {}

                /// Partially neglect the static validity check.
                /// 
                /// By default, [TransmuteFrom] and [TransmuteInto]'s methods require that all instantiations of the source type are guaranteed to be valid instantiations of the destination type. This precludes transmutations which *might* be valid depending on the source value:
                /// ```rust
                /// #[derive(PromiseTransmutableFrom, PromiseTransmutableInto)]
                /// #[repr(u8)]
                /// enum Bool {
                ///     True = 1,
                ///     False = 0,
                /// }
                /// 
                /// /* ⚠️ This example intentionally does not compile. */
                /// let _ : Bool  = some_u8_value.transmute_into(); // Compile Error!
                /// ```
                /// The [NeglectValidity] option disables this check.
                /// 
                /// By using the [NeglectValidity] option, you are committing to ensure dynamically source value is a valid instance of the destination type. For instance:
                /// ```rust
                /// #[derive(PromiseTransmutableFrom, PromiseTransmutableInto)]
                /// #[repr(u8)]
                /// enum Bool {
                ///     True = 1,
                ///     False = 0,
                /// }
                /// 
                /// pub trait TryIntoBool
                /// {
                ///     fn try_into_bool(self) -> Option<Bool>;
                /// }
                /// 
                /// impl<T> TryIntoBool for T
                /// where
                ///     T: TransmuteInto<u8>,
                ///     u8: TransmuteInto<Bool, NeglectValidity>
                /// {
                ///     fn try_into_bool(self) -> Option<Bool> {
                ///         let val: u8 = self.transmute_into();
                /// 
                ///         if val > 1 {
                ///             None
                ///         } else {
                ///             // Safe, because we've first verified that
                ///             // `val` is a bit-valid instance of a boolean.
                ///             Some(unsafe {val.unsafe_transmute_into()})
                ///         }
                ///     }
                /// }
                /// ```
                /// 
                /// Even with [NeglectValidity], the compiler will still statically reject transmutations that cannot possibly be valid:
                /// ```compile_fail
                /// #[derive(PromiseTransmutableInto)]
                /// #[repr(C)] enum Foo { A = 24 }
                /// 
                /// #[derive(PromiseTransmutableFrom)]
                /// #[repr(C)] enum Bar { Z = 42 }
                /// 
                /// let _ = <Bar as TransmuteFrom<Foo, NeglectValidity>::unsafe_transmute_from(Foo::N) // Compile error!
                /// ```
                pub struct NeglectValidity;
                impl TransmuteOptions for NeglectValidity {}

                /* FILL: Implementations for tuple combinations of options */

                // prevent third-party implementations of `TransmuteOptions`
                mod private {
                    use super::*;

                    pub trait Sealed {}

                    impl Sealed for () {}
                    impl Sealed for NeglectStability {}
                    impl Sealed for NeglectAlignment {}
                    impl Sealed for NeglectValidity {}

                    /* FILL: Implementations for tuple combinations of options */
                }
            }
        }


        /// **🌟** (Extension) Bit-altering conversions.
        ///
        /// This module demonstrates how the [transmute][module@transmute] API may be used to permit sound and complete slice casting and `Vec` casting.
        ///
        /// For more information on this extension, [**see here**](https://github.com/jswrenn/project-safe-transmute/blob/rfc/rfcs/0000-safe-transmute.md#case-study-abstractions-for-fast-parsing).
        #[cfg_attr(doc, unstable(feature = "cast", issue = "none"))]
        pub mod cast {

            use options::*;

            /// Cast `Self` into `Dst`.
            ///
            /// The reciprocal of [CastFrom]. This trait is implemented in terms of [CastFrom].
            pub trait CastInto<Dst, Neglect=()>
            where
                Dst: CastFrom<Self, Neglect>,
                Neglect: CastOptions,
            {
                /// Cast `self` into a value of type `Dst`, safely.
                fn cast_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: SafeCastOptions,
                {
                    CastFrom::<_, Neglect>::cast_from(self)
                }

                /// Cast `self` into a value of type `Dst`, potentially unsafely.
                unsafe fn unsafe_cast_into(self) -> Dst
                where
                    Self: Sized,
                    Dst: Sized,
                    Neglect: CastOptions,
                {
                    CastFrom::<_, Neglect>::unsafe_cast_from(self)
                }
            }

            impl<Src, Dst, Neglect> CastInto<Dst, Neglect> for Src
            where
                Dst: CastFrom<Self, Neglect>,
                Neglect: CastOptions,
            {}

            /// Instantiate `Self` from a value of type `Src`.
            ///
            /// The reciprocal of [CastInto].
            pub trait CastFrom<Src: ?Sized, Neglect=()>
            where
                Neglect: CastOptions,
            {
                /// Instantiate `Self` by casting a value of type `Src`, safely.
                fn cast_from(src: Src) -> Self
                where
                    Src: Sized,
                    Self: Sized,
                    Neglect: SafeCastOptions
                {
                    unsafe { CastFrom::<_,Neglect>::unsafe_cast_from(src) }
                }

                /// Instantiate `Self` by casting a value of type `Src`, potentially safely.
                unsafe fn unsafe_cast_from(src: Src) -> Self
                where
                    Src: Sized,
                    Self: Sized,
                    Neglect: CastOptions;
            }

            /// Options for casting.
            pub mod options {

                /// The super-trait of all *safe* casting options.
                #[marker] pub trait SafeCastOptions: CastOptions {}

                /// The super-trait of all casting options.
                #[marker] pub trait CastOptions {}

                impl SafeCastOptions for () {}
                impl CastOptions for () {}

            }
        }

    }

    use convert::transmute::*;

    /// Basic functions for dealing with memory.
    pub mod mem {
        #[doc(no_inline)]
        pub use core::mem::*;

        use crate::core::convert::transmute::{TransmuteFrom, stability::*, options::*};

        /// Implemented if `align_of::<Self>() <= align_of::<Rhs>()`
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        pub trait AlignLtEq<Rhs, Neglect=()>
        where
            Neglect: TransmuteOptions,
        {}

        /// By wrapping a type in a zero-sized array, we neutralize its validity and size qualities. The only quality by which `[Lhs; 0]` and `[Dst; 0]` can differ is their alignment. We check *only* if the alignment of `Lhs` is less than `Rhs` by transmuting between references of these zero-sized gadgets.
        impl<Lhs, Rhs, Neglect> AlignLtEq<Rhs, Neglect> for Lhs
        where
            Neglect: TransmuteOptions,
            for<'a> &'a [Lhs; 0]: TransmuteFrom<&'a [Rhs; 0], Neglect>
        {}

        /// Implemented if `align_of::<Self>() == align_of::<Rhs>()`
        ///
        /// See the [`Vec` casting demonstration][super::convert::cast::CastFrom#impl-CastFrom<Vec<Src>%2C%20Neglect>-for-Vec<Dst>] for an example of its use.
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        pub trait AlignEq<Rhs, Neglect=()>
        where
            Neglect: TransmuteOptions,
        {}

        /// See [AlignLtEq].
        impl<Lhs, Rhs, Neglect> AlignEq<Rhs, Neglect> for Lhs
        where
            Neglect: TransmuteOptions,
            Lhs: AlignLtEq<Rhs>,
            Rhs: AlignLtEq<Lhs>,
        {}

        mod private {
            use core::mem::MaybeUninit;

            // The alignment of this struct is always equal to `max(align_of::<A>(), align_of::<T>())`.
            // Its validity is always equal to `MaybeUninit<T>`.
            // Its size equals `T`
            /* #[derive(PromiseTransmutableFrom, PromiseTransmutableInto)] */
            #[repr(C)]
            pub struct Aligned<A, T>(pub [A; 0], pub MaybeUninit<T>);
        }

        /// Implemented if `size_of::<Self>() <= size_of::<Rhs>()`
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        pub trait SizeLtEq<Rhs, Neglect=()>
        where
            Neglect: TransmuteOptions,
        {}

        /// We wrap the types in a struct that neutralizes their alignment and validity differences, leaving size as the only quality that might differ between `Aligned<Rhs, Lhs>` and `Aligned<Lhs, Rhs>`.
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        impl<Lhs, Rhs, Neglect> SizeLtEq<Rhs, Neglect> for Lhs
        where
            Neglect: TransmuteOptions,
            for<'a> &'a private::Aligned<Rhs, Lhs>: TransmuteFrom<&'a private::Aligned<Lhs, Rhs>>,
        {}

        /// Implemented if `size_of::<Self>() == size_of::<Rhs>()`
        ///
        /// See the [`Vec` casting demonstration][super::convert::cast::CastFrom#impl-CastFrom<Vec<Src>%2C%20Neglect>-for-Vec<Dst>] for an example of its use.
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        pub trait SizeEq<Rhs, Neglect=()>
        where
            Neglect: TransmuteOptions,
        {}

        /// See [SizeLtEq].
        #[cfg_attr(doc, unstable(feature = "query", issue = "none"))]
        impl<Lhs, Rhs, Neglect> SizeEq<Rhs, Neglect> for Lhs
        where
            Neglect: TransmuteOptions,
            Lhs: SizeLtEq<Rhs>,
            Rhs: SizeLtEq<Lhs>,
        {}
    }

    /// A dynamically-sized view into a contiguous sequence, `[T]`.
    pub mod slice {
        #[doc(no_inline)]
        pub use core::slice::*;

        use crate::core::convert::{
            transmute::{
                TransmuteFrom,
                options::{SafeTransmuteOptions, TransmuteOptions},
            },
            cast::{
                CastFrom,
                options::{
                    SafeCastOptions,
                    CastOptions,
                },
            },
        };

        use core::{
            mem::{size_of, size_of_val},
            slice
        };

        /// **🌟** *Safe* options for casting **slices**.
        ///
        /// Slice casting transmutes the contents of the slice, and adjusts the slice's length as needed. All [SafeTransmuteOptions] are [SafeSliceCastOptions].
        #[cfg_attr(doc, unstable(feature = "cast", issue = "none"))]
        pub trait SafeSliceCastOptions
            : SafeCastOptions
            + SafeTransmuteOptions
            + SliceCastOptions
        {}

        /// **🌟** Options for casting **slices**.
        ///
        /// Slice casting transmutes the contents of the slice, and adjusts the slice's length as needed. All [TransmuteOptions] are [SliceCastOptions].
        #[cfg_attr(doc, unstable(feature = "cast", issue = "none"))]
        pub trait SliceCastOptions
            : CastOptions
            + TransmuteOptions
        {}

        impl<Neglect: SafeSliceCastOptions> SafeCastOptions for Neglect {}
        impl<Neglect: SafeTransmuteOptions> SafeSliceCastOptions for Neglect {}

        impl<Neglect: SliceCastOptions> CastOptions for Neglect {}
        impl<Neglect: TransmuteOptions> SliceCastOptions for Neglect {}

        /// #### `&[Src]` **🠮** `&[Dst]`
        /// If the sizes of `Src` and `Dst` differ, the length of the output slice is adjusted as-needed.
        ///
        /// ##### Static Options
        /// See [SliceCastOptions] and [SafeSliceCastOptions].
        ///
        /// ##### Example
        /// ```rust
        /// let src : &[i8] = &[-1, -2, -3, -4][..];
        /// let dst : &[[u8; 2]] = src.cast_into();
        /// assert_eq!(dst, &[[-255, -254], [-253, -252]][..]);
        /// ```
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i [Src], Neglect> for &'o [Dst]
        where
            Neglect: SliceCastOptions,
            &'o [Dst; 1]: TransmuteFrom<&'i [Src; usize::MAX], Neglect>
        {
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn unsafe_cast_from(src: &'i [Src]) -> &'o [Dst]
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe { slice::from_raw_parts(src.as_ptr() as *const Dst, len) }
            }
        }

        /// #### `&mut [Src]` **🠮** `&mut [Dst]`
        /// If the sizes of `Src` and `Dst` differ, the length of the output slice is adjusted as-needed.
        ///
        /// ##### Static Options
        /// See [SliceCastOptions] and [SafeSliceCastOptions].
        ///
        /// ##### Example
        /// ```rust
        /// let src : &mut [i8] = &mut [-1, -2, -3, -4][..];
        /// let dst : &mut [[u8; 2]] = src.cast_into();
        /// assert_eq!(dst, &mut [[-255, -254], [-253, -252]][..]);
        /// ```
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i mut [Src], Neglect> for &'o mut [Dst]
        where
            Neglect: SliceCastOptions,
            &'o mut [Dst; 1]: TransmuteFrom<&'i mut [Src; usize::MAX], Neglect>
        {
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn unsafe_cast_from(src: &'i mut [Src]) -> &'o mut [Dst]
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe { slice::from_raw_parts_mut(src.as_ptr() as *mut Dst, len) }
            }
        }

        /// #### `&mut [Src]` **🠮** `&[Dst]`
        /// If the sizes of `Src` and `Dst` differ, the length of the output slice is adjusted as-needed.
        ///
        /// ##### Static Options
        /// See [SliceCastOptions] and [SafeSliceCastOptions].
        ///
        /// ##### Example
        /// ```rust
        /// let src : &mut [i8] = &mut [-1, -2, -3, -4][..];
        /// let dst : &[[u8; 2]] = src.cast_into();
        /// assert_eq!(dst, &[[-255, -254], [-253, -252]][..]);
        /// ```
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i mut [Src], Neglect> for &'o [Dst]
        where
            Neglect: SliceCastOptions,
            &'o [Dst; 1]: TransmuteFrom<&'i mut [Src; usize::MAX], Neglect>
        {
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn unsafe_cast_from(src: &'i mut [Src]) -> &'o [Dst]
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe {
                    slice::from_raw_parts(src.as_ptr() as *const Dst, len)
                }
            }
        }
    }
}

/// Additions to `libstd`
pub mod std {
    /// A contiguous growable array type with heap-allocated contents, `Vec<T>`.
    pub mod vec {
        #[doc(no_inline)]
        pub use std::vec::*;

        use crate::core::convert::{
            transmute::{
                TransmuteFrom,
                options::{SafeTransmuteOptions, TransmuteOptions, NeglectAlignment},
            },
            cast::{
                CastFrom,
                options::{
                    SafeCastOptions,
                    CastOptions,
                },
            },
        };

        /// **🌟** Safe options for casting `Vec<T>` to `Vec<U>`.
        ///
        /// Vec casting transmutes the contents of the vec. All [SafeTransmuteOptions] are [SafeVecCastOptions].
        ///
        /// See the [here][crate::core::convert::cast::CastFrom#impl-CastFrom<Vec<Src>%2C%20Neglect>-for-Vec<Dst>] for examples.
        #[cfg_attr(doc, unstable(feature = "cast", issue = "none"))]
        pub trait SafeVecCastOptions
            : SafeCastOptions
            + SafeTransmuteOptions
            + VecCastOptions
        {}

        /// **🌟** Options for casting `Vec<T>` to `Vec<U>`.
        ///
        /// Vec casting transmutes the contents of the vec. All [TransmuteOptions] are [VecCastOptions].
        ///
        /// See the [here][crate::core::convert::cast::CastFrom#impl-CastFrom<Vec<Src>%2C%20Neglect>-for-Vec<Dst>] for examples.
        #[cfg_attr(doc, unstable(feature = "cast", issue = "none"))]
        pub trait VecCastOptions
            : TransmuteOptions
            + CastOptions
        {}

        impl<Neglect: SafeVecCastOptions> SafeCastOptions for Neglect {}
        impl<Neglect: SafeTransmuteOptions> SafeVecCastOptions for Neglect {}

        impl<Neglect: VecCastOptions> CastOptions for Neglect {}
        impl<Neglect: TransmuteOptions> VecCastOptions for Neglect {}

        use core::mem::MaybeUninit;
        use crate::core::mem::{SizeEq, AlignEq};

        /// #### `Vec<Src>` **🠮** `Vec<Dst>`
        /// [`Vec::from_raw_parts`][Vec::from_raw_parts] requires that the size and static alignment of `Src` and `Dst` be equal. [NeglectAlignment] is therefore ignored. We use the [AlignEq] and [SizeEq] traits to enforce these invariants statically. 
        /// ##### Static Options
        /// See [VecCastOptions] and [SafeVecCastOptions].
        ///
        /// ##### Example
        /// ```rust
        /// let src : Vec<i8> = vec![-1, -2, -3, -4];
        /// let dst : Vec<u8> = src.cast_into();
        /// assert_eq!(dst, vec![255, 254, 253, 252]);
        /// ```
        impl<Src, Dst, Neglect> CastFrom<Vec<Src>, Neglect> for Vec<Dst>
        where
            Neglect: VecCastOptions,
            Dst: TransmuteFrom<Src, Neglect>
               + AlignEq<Src, Neglect>
               + SizeEq<Src, Neglect>,
        {
            #[doc(hidden)]
            #[inline(always)]
            unsafe fn unsafe_cast_from(src: Vec<Src>) -> Vec<Dst>
            {
                let (ptr, len, cap) = src.into_raw_parts();
                Vec::from_raw_parts(ptr as *mut Dst, len, cap)
            }
        }
    }

}