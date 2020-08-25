var searchIndex = JSON.parse('{\
"rfc":{"doc":"Rustdoc for the API surface proposed by the safer…","i":[[0,"core","rfc","Additions to libcore.",null,null],[0,"convert","rfc::core","Traits for conversions between types.",null,null],[0,"transmute","rfc::core::convert","🌟 Bit-preserving conversions.",null,null],[0,"stability","rfc::core::convert::transmute","Traits for declaring the SemVer stability of a type\'s…",null,null],[8,"PromiseTransmutableInto","rfc::core::convert::transmute::stability","Promise that a type may be stably transmuted into other…",null,null],[16,"Archetype","","A type which exemplifies the greatest extent to which…",0,null],[8,"PromiseTransmutableFrom","","Promise that a type may be stably transmuted from other…",null,null],[16,"Archetype","","A type which exemplifies the greatest extent to which…",1,null],[14,"PromiseTransmutableInto","","Derive macro generating an impl of the trait…",null,null],[14,"PromiseTransmutableFrom","","Derive macro generating an impl of the trait…",null,null],[14,"PromiseTransmutable","","Derive macro generating impls of both…",null,null],[0,"options","rfc::core::convert::transmute","Static checks that may be neglected when determining if…",null,null],[3,"NeglectStability","rfc::core::convert::transmute::options","Neglect the static stability check.",null,null],[3,"NeglectAlignment","","Neglect the static alignment check.",null,null],[3,"NeglectValidity","","Partially neglect the static validity check.",null,null],[8,"SafeTransmuteOptions","","Options that may be used with safe transmutations.",null,null],[8,"TransmuteOptions","","Options that may be used with unsafe transmutations.",null,null],[8,"TransmuteInto","rfc::core::convert::transmute","Reinterpret the bits of `Self` as a type `Dst`.",null,null],[10,"transmute_into","","Reinterpret the bits of a value of one type as another…",2,[[]]],[10,"unsafe_transmute_into","","Reinterpret the bits of a value of one type as another…",2,[[]]],[8,"TransmuteFrom","","Reinterpret the bits of `Src` as a type `Self`.",null,null],[11,"transmute_from","","Reinterpret the bits of a value of one type as another…",3,[[]]],[11,"unsafe_transmute_from","","Reinterpret the bits of a value of one type as another…",3,[[]]],[0,"cast","rfc::core::convert","🌟 (Extension) Bit-altering conversions.",null,null],[0,"options","rfc::core::convert::cast","Options for casting.",null,null],[8,"SafeCastOptions","rfc::core::convert::cast::options","The super-trait of all safe casting options.",null,null],[8,"CastOptions","","The super-trait of all casting options.",null,null],[8,"CastInto","rfc::core::convert::cast","Cast `Self` into `Dst`.",null,null],[11,"cast_into","","Cast `self` into a value of type `Dst`, safely.",4,[[]]],[11,"unsafe_cast_into","","Cast `self` into a value of type `Dst`, potentially…",4,[[]]],[8,"CastFrom","","Instantiate `Self` from a value of type `Src`.",null,null],[11,"cast_from","","Instantiate `Self` by casting a value of type `Src`, safely.",5,[[]]],[10,"unsafe_cast_from","","Instantiate `Self` by casting a value of type `Src`,…",5,[[]]],[0,"mem","rfc::core","Basic functions for dealing with memory.",null,null],[8,"AlignLtEq","rfc::core::mem","Implemented if `align_of::<Self>() <= align_of::<Rhs>()`",null,null],[8,"AlignEq","","Implemented if `align_of::<Self>() == align_of::<Rhs>()`",null,null],[8,"SizeLtEq","","Implemented if `size_of::<Self>() <= size_of::<Rhs>()`",null,null],[8,"SizeEq","","Implemented if `size_of::<Self>() == size_of::<Rhs>()`",null,null],[0,"slice","rfc::core","A dynamically-sized view into a contiguous sequence, `[T]`.",null,null],[8,"SafeSliceCastOptions","rfc::core::slice","🌟 Safe options for casting slices.",null,null],[8,"SliceCastOptions","","🌟 Options for casting slices.",null,null],[0,"std","rfc","Additions to `libstd`",null,null],[0,"vec","rfc::std","A contiguous growable array type with heap-allocated…",null,null],[8,"SafeVecCastOptions","rfc::std::vec","🌟 Safe options for casting `Vec<T>` to `Vec<U>`.",null,null],[8,"VecCastOptions","","🌟 Options for casting `Vec<T>` to `Vec<U>`.",null,null],[14,"PromiseTransmutableInto","rfc","Derive macro generating an impl of the trait…",null,null],[14,"PromiseTransmutableFrom","","Derive macro generating an impl of the trait…",null,null],[14,"PromiseTransmutable","","Derive macro generating impls of both…",null,null],[11,"transmute_into","rfc::core::convert::transmute::options","",6,[[]]],[11,"unsafe_transmute_into","","",6,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"transmute_into","","",7,[[]]],[11,"unsafe_transmute_into","","",7,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"transmute_into","","",8,[[]]],[11,"unsafe_transmute_into","","",8,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]]],"p":[[8,"PromiseTransmutableInto"],[8,"PromiseTransmutableFrom"],[8,"TransmuteInto"],[8,"TransmuteFrom"],[8,"CastInto"],[8,"CastFrom"],[3,"NeglectStability"],[3,"NeglectAlignment"],[3,"NeglectValidity"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);