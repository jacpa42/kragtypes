use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(BindValues)]
pub fn __internal_derive_bind_values(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = input.ident;
    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(st) => match st.fields {
            syn::Fields::Named(fields) => fields.named,
            _ => panic!("Fields must be named for BindValues to be derived"),
        },
        _ => panic!("BindValues is only derivable for data structs."),
    };

    let bound_values_internal = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let field_type = &field.ty;
        if is_option(field_type) {
            Some(quote! {
                if self.#field_name.is_some() {
                    values.push(smol_str::SmolStr::from(#field_str));
                }
            })
        } else if is_phantom(field_type) {
            None
        } else {
            Some(quote! {
                values.push(smol_str::SmolStr::from(#field_str));
            })
        }
    });

    let bind_values_internal = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if is_option(field_type) {
            Some(quote! {
                if let Some(val) = self.#field_name.as_ref() {
                   query = query.bind(val);
                }
            })
        } else if is_phantom(field_type) {
            None
        } else {
            Some(quote! {
                query = query.bind(&self.#field_name);
            })
        }
    });

    quote! {
        impl BindValues for #struct_name #generics {
    fn bind_values<'q>(
        &'q self,
        mut query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    ) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
                #(#bind_values_internal)*
            query
            }

    fn bound_values(&self) -> Vec<smol_str::SmolStr> {
                let mut values = Vec::new();
                #(#bound_values_internal)*
                values
        }
        }
    }
    .into()
}

fn is_phantom(ty: &syn::Type) -> bool {
    if let syn::Type::Path(path_type) = ty {
        path_type
            .path
            .segments
            .last()
            .map_or(false, |s| s.ident == "PhantomData")
    } else {
        false
    }
}

fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(path_type) = ty {
        path_type.path.segments.iter().any(|s| s.ident == "Option")
    } else {
        false
    }
}
