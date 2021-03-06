use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Result};

use crate::context::Context;

pub fn table(input: DeriveInput) -> Result<TokenStream> {
    // Create context for generating expressions
    let context = Context::new(&input)?;

    // Used in the quasi-quotation below as `#name`
    let name = context.container.name;

    // Fetch cli_table crate name
    let cli_table = &context.container.crate_name;

    // Split a type's generics into the pieces required for implementing a trait for that type
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut field_names = Vec::new();
    let mut field_rows = Vec::new();

    for field in context.fields.into_iter() {
        field_names.push(field.name);

        let ident = field.ident;
        let justify = field.justify;
        let align = field.align;
        let color = field.color;
        let bold = field.bold;
        let span = field.span;

        let mut row = quote_spanned! {span=>
            #cli_table ::Cell::cell(self. #ident)
        };

        if let Some(justify) = justify {
            row = quote_spanned! {span=>
                #row .justify(#justify)
            };
        }

        if let Some(align) = align {
            row = quote_spanned! {span=>
                #row .align(#align)
            };
        }

        if let Some(color) = color {
            row = quote_spanned! {span=>
                #cli_table ::Style::foreground_color(#row, ::core::convert::From::from(#color))
            };
        }

        if let Some(bold) = bold {
            row = quote_spanned! {span=>
                #cli_table ::Style::bold(#row, #bold)
            };
        }

        field_rows.push(row);
    }

    // Build the output, possibly using quasi-quotation
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #cli_table ::Title for #name #ty_generics # where_clause{
            fn title() -> #cli_table ::RowStruct {
                #cli_table ::Row::row(::std::vec![
                    #(#cli_table ::Style::bold(#cli_table ::Cell::cell(#field_names), true),)*
                ])
            }
        }

        #[automatically_derived]
        impl #impl_generics #cli_table ::Row for & #name #ty_generics # where_clause{
            fn row(self) -> #cli_table ::RowStruct {
                #cli_table ::Row::row(::std::vec![
                    #(#field_rows,)*
                ])
            }
        }

        #[automatically_derived]
        impl #impl_generics #cli_table ::Row for #name #ty_generics # where_clause{
            fn row(self) -> #cli_table ::RowStruct {
                #cli_table ::Row::row(&self)
            }
        }
    })
}
