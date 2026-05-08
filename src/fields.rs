use core::{
    mem::{
        MaybeUninit,
        type_info::{Field, Type, TypeKind},
    },
    ptr::{self, DynMetadata, Pointee},
};

use crate::Assert;

mod complex_expressions {
    use super::fields;
    use core::any::TypeId;

    pub trait AllFieldsImplement<Trait: ?Sized + 'static>: Fields {
        type const OUTPUT: bool;
    }
    impl<T: Fields, Trait: ?Sized + 'static> AllFieldsImplement<Trait> for T {
        type const OUTPUT: bool = const {
            let fields = fields::<T>().unwrap();

            let mut index = 0;
            let mut all_implement = true;
            while index < fields.len() {
                let field = &fields[index];
                index += 1;

                if field
                    .ty
                    .trait_info_of_trait_type_id(TypeId::of::<Trait>())
                    .is_none()
                {
                    all_implement = false;
                }
            }

            all_implement
        };
    }

    pub trait Fields {
        type const OUTPUT: bool;
    }
    impl<T> Fields for T {
        type const OUTPUT: bool = const { fields::<T>().is_some() };
    }
}

/// Keep your custom error messages in here.
mod on_unimplemented {
    use super::Assert;

    #[diagnostic::on_unimplemented(message = "`{SelfType}` is not a type that has fields.")]
    pub trait Fields<SelfType> {}
    impl<SelfType> Fields<SelfType> for Assert<true> {}

    #[diagnostic::on_unimplemented(
        message = "`{SelfType}` is not a type in which all fields implement the trait `{Trait}`."
    )]
    pub trait AllFieldsImplement<SelfType, Trait: ?Sized> {}
    impl<SelfType, Trait: ?Sized> AllFieldsImplement<SelfType, Trait> for Assert<true> {}
}

const fn type_as_first_field<T: ?Sized>() -> &'static [Field] {
    let TypeKind::Tuple(tuple) = Type::of::<(T,)>().kind else {
        unreachable!();
    };

    tuple.fields
}

const fn fields<T>() -> Option<&'static [Field]> {
    let info = Type::of::<T>();

    if info.size.is_none() {
        return None;
    }

    match Type::of::<T>().kind {
        TypeKind::Tuple(info) => Some(info.fields),
        TypeKind::Struct(info) => Some(info.fields),
        TypeKind::Bool(..)
        | TypeKind::Char(..)
        | TypeKind::Int(..)
        | TypeKind::Str(..)
        | TypeKind::Reference(..)
        | TypeKind::Float(..)
        | TypeKind::Pointer(..) => Some(type_as_first_field::<T>()),
        _ => None,
    }
}

/// Types which have fields.
pub trait Fields {
    /// The quantity of fields the type has.
    type const LENGTH: usize;
}
impl<T> Fields for T
where
    Assert<{ <T as complex_expressions::Fields>::OUTPUT }>: on_unimplemented::Fields<T>,
{
    type const LENGTH: usize = const { fields::<T>().unwrap().len() };
}

/// Types in which all fields implement a specified trait.
pub trait AllFieldsImplement<Trait: ?Sized + 'static>: Fields {}
impl<T: Fields, Trait: ?Sized + 'static> AllFieldsImplement<Trait> for T where
    Assert<{ <T as complex_expressions::AllFieldsImplement<Trait>>::OUTPUT }>:
        on_unimplemented::AllFieldsImplement<T, Trait>
{
}

/// Allows accessing a type's fields as an array.
pub trait FieldsToArray: Fields {
    /// Presents a type's fields as an array of the specified `dyn Trait`.
    fn fields_to_array_ref<Trait: Pointee<Metadata = DynMetadata<Trait>> + ?Sized + 'static>(
        &self,
    ) -> [&Trait; Self::LENGTH]
    where
        Self: AllFieldsImplement<Trait>;
}
impl<T: Fields> FieldsToArray for T {
    fn fields_to_array_ref<Trait: Pointee<Metadata = DynMetadata<Trait>> + ?Sized + 'static>(
        &self,
    ) -> [&Trait; Self::LENGTH]
    where
        Self: AllFieldsImplement<Trait>,
    {
        let vtable_array: [(DynMetadata<Trait>, usize); Self::LENGTH] = const {
            let mut vtable_array =
                [MaybeUninit::<(DynMetadata<Trait>, usize)>::uninit(); Self::LENGTH];

            let fields = fields::<T>().unwrap();

            let mut index = 0;
            while index < fields.len() {
                let field = &fields[index];
                let vtable = &mut vtable_array[index];
                index += 1;

                vtable.write((
                    field.ty.trait_info_of::<Trait>().unwrap().get_vtable(),
                    field.offset,
                ));
            }

            unsafe { MaybeUninit::array_assume_init(vtable_array) }
        };

        let self_pointer: *const Self = self;
        vtable_array.map(|(vtable, offset)| unsafe {
            ptr::from_raw_parts::<Trait>(self_pointer.byte_add(offset), vtable).as_ref_unchecked()
        })
    }
}
