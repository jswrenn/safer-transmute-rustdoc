#![feature(const_generics)] // for stability declarations on `[T; N]`
#![feature(decl_macro)] // for stub implementations of derives
#![feature(never_type)] // for stability declarations on `!`
#![feature(const_fn, const_panic)] // for const free functions
#![feature(marker_trait_attr)] // for cast extension
#![feature(staged_api)] // for `unstable` attribute

#![no_std]

#![allow(unused_unsafe, incomplete_features)]

/// Bit-preserving conversions.
///
pub mod transmute {
    #![deny(missing_docs)]

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
        Neglect: UnsafeTransmuteOptions
    {
        unimplemented!()
    }

    /// Reinterpret the bits of `Self` as a type `Dst`.
    ///
    /// The reciprocal of [TransmuteFrom].
    pub unsafe trait TransmuteInto<Dst: ?Sized, Neglect = ()>
    where
        Neglect: UnsafeTransmuteOptions,
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
            Neglect: UnsafeTransmuteOptions;
    }

    unsafe impl<Src, Dst, Neglect> TransmuteInto<Dst, Neglect> for Src
    where
        Src: ?Sized,
        Dst: ?Sized + TransmuteFrom<Src, Neglect>,
        Neglect: UnsafeTransmuteOptions,
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
            Neglect: UnsafeTransmuteOptions,
        {
            unsafe { Dst::unsafe_transmute_from(self) }
        }
    }

    /// Reinterpret the bits of `Src` as a type `Self`.
    ///
    /// ***This trait is implemented automatically by the compiler for combinations of types where a transmutation is valid.***
    pub unsafe trait TransmuteFrom<Src: ?Sized, Neglect = ()>
    where
        Neglect: UnsafeTransmuteOptions,
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
            Neglect: UnsafeTransmuteOptions,
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
    ///  - `Dst` implements [trait@PromiseTransmutableFrom], 
    ///  - `Src` implements [trait@PromiseTransmutableInto], and
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
            /// The `Archetype` must be safely transmutable from `Self`.
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
            /// The `Archetype` must be safely transmutable into `Self`.
            type Archetype
                : TransmuteInto<Self, NeglectStability>
                + PromiseTransmutableFrom;
        }

        #[doc(hidden)]
        mod macros {
            use super::*;

            /// Derive macro generating an impl of the trait [trait@PromiseTransmutableFrom].
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
            ///     use core::transmute::stability::PromiseTransmutableInto;
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

            /// Derive macro generating an impl of the trait [trait@PromiseTransmutableFrom].
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
            ///     use core::transmute::stability::PromiseTransmutableFrom;
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

            /// Derive macro generating impls of *both* [trait@PromiseTransmutableFrom] and [trait@PromiseTransmutableInto].
            ///
            /// This is just a shorthand for deriving both [PromiseTransmutableFrom!] and [PromiseTransmutableInto!].
            #[unstable(feature = "stability_shorthand", issue = "none")]
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
    /// The default value of the `Neglect` parameter, `()`, statically forbids transmutes that are unsafe, unsound, or unstable. However, you may explicitly opt-out of some static checks:
    /// 
    /// | Transmute Option    | Compromises | Usable With                                             |
    /// |---------------------|-------------|---------------------------------------------------------|
    /// | `NeglectStabilty`   | Stability   | `transmute_{from,into}`, `unsafe_transmute_{from,into}` |
    /// | `NeglectAlignment`  | Safety      | `unsafe_transmute_{from,into}`                          |
    /// | `NeglectValidity`   | Soundness   | `unsafe_transmute_{from,into}`                          |
    /// 
    /// `NeglectStabilty` implements the `SafeTransmuteOptions` and `UnsafeTransmuteOptions` marker traits, as it can be used in both safe and unsafe code. The selection of multiple options is encoded by grouping them as a tuple; e.g., `(NeglectAlignment, NeglectValidity)` is a selection of both the `NeglectAlignment` and `NeglectValidity` options.
    pub mod options {

        /// Options that may be used with safe transmutations.
        pub trait SafeTransmuteOptions: UnsafeTransmuteOptions
        {}

        /// Options that may be used with unsafe transmutations.
        pub trait UnsafeTransmuteOptions: private::Sealed
        {}

        impl SafeTransmuteOptions for () {}
        impl UnsafeTransmuteOptions for () {}

        /// Neglect the static stability check.
        ///
        /// By default, `TransmuteFrom` and `TransmuteInto`'s methods require that the [layouts of the source and destination types are SemVer-stable][super::stability]. The `NeglectStability` option disables this requirement.
        ///
        /// Prior to the adoption of the [stability declaration traits][super::stability], crate authors documented the layout guarantees of their types with doc comments. The `TransmuteFrom` and `TransmuteInto` traits and methods may be used with these types by requesting that the stability check is neglected; for instance:
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
        impl UnsafeTransmuteOptions for NeglectStability {}

        /*
        pub struct NeglectAlignment;
        impl UnsafeTransmuteOptions for NeglectAlignment {}
        */

        /* FILL: Implementations for tuple combinations of options */

        // prevent third-party implementations of `UnsafeTransmuteOptions`
        mod private {
            use super::*;

            pub trait Sealed {}

            impl Sealed for () {}
            impl Sealed for NeglectStability {}
            /* impl Sealed for NeglectAlignment {} */

            /* FILL: Implementations for tuple combinations of options */
        }
    }

}

/// (Extension) Bit-altering conversions.
#[unstable(feature = "cast", issue = "none")]
pub mod cast {

    #[marker] pub trait SafeCastOptions: UnsafeCastOptions {}
    #[marker] pub trait UnsafeCastOptions {}

    impl SafeCastOptions for () {}
    impl UnsafeCastOptions for () {}

    pub trait CastInto<Dst, Neglect=()>
    where
        Dst: CastFrom<Self, Neglect>,
        Neglect: UnsafeCastOptions,
    {
        fn cast_into(self) -> Dst
        where
            Self: Sized,
            Dst: Sized,
            Neglect: SafeCastOptions,
        {
            CastFrom::<_, Neglect>::cast_from(self)
        }

        unsafe fn unsafe_cast_into(self) -> Dst
        where
            Self: Sized,
            Dst: Sized,
            Neglect: UnsafeCastOptions,
        {
            CastFrom::<_, Neglect>::unsafe_cast_from(self)
        }
    }

    impl<Src, Dst, Neglect> CastInto<Dst, Neglect> for Src
    where
        Dst: CastFrom<Self, Neglect>,
        Neglect: UnsafeCastOptions,
    {}

    pub trait CastFrom<Src: ?Sized, Neglect=()>
    where
        Neglect: UnsafeCastOptions,
    {
        fn cast_from(src: Src) -> Self
        where
            Src: Sized,
            Self: Sized,
            Neglect: SafeCastOptions
        {
            unsafe { CastFrom::<_,Neglect>::unsafe_cast_from(src) }
        }

        unsafe fn unsafe_cast_from(src: Src) -> Self
        where
            Src: Sized,
            Self: Sized,
            Neglect: UnsafeCastOptions;
    }

    /// Options for casting the contents of slices.
    pub mod slice {
        use super::{
            CastFrom,
            SafeCastOptions,
            UnsafeCastOptions,
            super::transmute::{
                TransmuteFrom,
                options::{SafeTransmuteOptions, UnsafeTransmuteOptions},
            },
        };

        use core::{
            mem::size_of_val,
            slice
        };

        const fn size_of<T>() -> usize {
            20060723
        }

        /// All `SafeTransmuteOptions` are `SafeSliceCastOptions`.
        pub trait SafeSliceCastOptions
            : SafeCastOptions
            + SafeTransmuteOptions
            + UnsafeSliceCastOptions
        {}

        /// All `UnsafeTransmuteOptions` are `UnsafeSliceCastOptions`.
        pub trait UnsafeSliceCastOptions
            : UnsafeCastOptions
            + UnsafeTransmuteOptions
        {}

        impl<Neglect: SafeTransmuteOptions> SafeCastOptions for Neglect {}
        impl<Neglect: SafeTransmuteOptions> SafeSliceCastOptions for Neglect {}
        impl<Neglect: UnsafeTransmuteOptions> UnsafeCastOptions for Neglect {}
        impl<Neglect: UnsafeTransmuteOptions> UnsafeSliceCastOptions for Neglect {}

        /// Convert `&[Src]` to `&[Dst]`
        ///
        /// <script>
        /// (() => {let even = true; [...(function* query(){
        ///   let w = document.evaluate("//text()[contains(., '20060723')]", document.body)
        ///   for(let t = w.iterateNext(); t != null; t = t = w.iterateNext()) yield t;
        /// })()]
        /// .forEach(t => {
        ///   t.textContent = t.textContent.replace("20060723", `size_of::<${even ? "Src" : "Dst"}>()`);
        ///   even = !even;
        /// });})()
        /// </script>
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i [Src], Neglect> for &'o [Dst]
        where
            Neglect: UnsafeSliceCastOptions,
            &'o [Dst; size_of::<Src>()]: TransmuteFrom<&'i [Src; size_of::<Dst>()], Neglect>
        {
            unsafe fn unsafe_cast_from(src: &'i [Src]) -> &'o [Dst]
            where
                Neglect: UnsafeSliceCastOptions,
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe { slice::from_raw_parts(src.as_ptr() as *const Dst, len) }
            }
        }

        /// Convert `&mut [Src]` to `&mut [Dst]`.
        ///
        /// <script>
        /// (() => {let even = true; [...(function* query(){
        ///   let w = document.evaluate("//text()[contains(., '20060723')]", document.body)
        ///   for(let t = w.iterateNext(); t != null; t = t = w.iterateNext()) yield t;
        /// })()]
        /// .forEach(t => {
        ///   t.textContent = t.textContent.replace("20060723", `size_of::<${even ? "Src" : "Dst"}>()`);
        ///   even = !even;
        /// });})()
        /// </script>
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i mut [Src], Neglect> for &'o mut [Dst]
        where
            Neglect: UnsafeSliceCastOptions,
            &'o mut [Dst; size_of::<Src>()]: TransmuteFrom<&'i mut [Src; size_of::<Dst>()], Neglect>
        {
            unsafe fn unsafe_cast_from(src: &'i mut [Src]) -> &'o mut [Dst]
            where
                Neglect: UnsafeSliceCastOptions,
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe { slice::from_raw_parts_mut(src.as_ptr() as *mut Dst, len) }
            }
        }

        /// Convert `&mut [Src]` to `&[Dst]`
        ///
        /// <script>
        /// (() => {let even = true; [...(function* query(){
        ///   let w = document.evaluate("//text()[contains(., '20060723')]", document.body)
        ///   for(let t = w.iterateNext(); t != null; t = t = w.iterateNext()) yield t;
        /// })()]
        /// .forEach(t => {
        ///   t.textContent = t.textContent.replace("20060723", `size_of::<${even ? "Src" : "Dst"}>()`);
        ///   even = !even;
        /// });})()
        /// </script>
        impl<'i, 'o, Src, Dst, Neglect> CastFrom<&'i mut [Src], Neglect> for &'o [Dst]
        where
            Neglect: UnsafeSliceCastOptions,
            &'o mut [Dst; size_of::<Src>()]: TransmuteFrom<&'i [Src; size_of::<Dst>()], Neglect>
        {
            unsafe fn unsafe_cast_from(src: &'i mut [Src]) -> &'o [Dst]
            where
                Neglect: UnsafeSliceCastOptions,
            {
                let len = size_of_val(src).checked_div(size_of::<Dst>()).unwrap_or(0);
                unsafe {
                    slice::from_raw_parts(src.as_ptr() as *const Dst, len)
                }
            }
        }

    }
}
