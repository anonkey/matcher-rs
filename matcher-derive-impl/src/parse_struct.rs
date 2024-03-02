extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ItemStruct;

#[derive(Debug, Default)]
pub struct FieldAttrs {
    pub start_with: bool,
}

pub fn get_fields(struct_data: &ItemStruct) -> Vec<(String, String, FieldAttrs)> {
    let mut fields = Vec::new();
    for field in struct_data.fields.iter() {
        let mut field_attrs = FieldAttrs::default();

        if field
            .attrs
            .clone()
            .into_iter()
            .any(|t| t.to_token_stream().to_string() == "#[match_start_with]")
        {
            field_attrs.start_with = true;
        }

        fields.push((
            field.ident.to_token_stream().to_string(),
            field.ty.to_token_stream().to_string(),
            field_attrs,
        ));
    }
    fields
}

pub fn get_fields_decls(fields: &Vec<(String, String, FieldAttrs)>) -> (TokenStream, TokenStream) {
    let mut field_matches = Vec::new();
    let mut field_decl = Vec::new();

    for (ident, ty, attrs) in fields {
        let match_fn_name = if attrs.start_with {
            "match_start"
        } else {
            "match_all"
        };
        field_decl.push(format!(
        "{ident}: matcher_derive_impl::matcher::BaseMatcher<<{ty} as matcher_derive_impl::matcher::Matcher>::AllMatcher>,"
    ));
        field_matches.push(format!(
            "value.{ident}.{match_fn_name}(matcher.{ident}.clone()) &&"
        ));
    }
    field_matches.push("true".to_string());

    let field_matches: proc_macro2::TokenStream =
        field_matches.join("\n").parse().expect("invalid2");
    let field_decl: proc_macro2::TokenStream = field_decl.join("\n").parse().expect("invalid3");

    (field_matches, field_decl)
}

pub fn get_struct_params(struct_data: &ItemStruct) -> TokenStream {
    let struct_params = struct_data.generics.params.clone();

    if struct_params.empty_or_trailing() {
        "".parse().unwrap()
    } else {
        format!("<{}>", struct_params.clone().into_token_stream())
            .parse()
            .unwrap()
    }
}

pub fn matcher_derive_struct(struct_data: &ItemStruct) -> TokenStream {
    let fields = get_fields(struct_data);

    let (field_matches, field_decl) = get_fields_decls(&fields);
    let struct_params = get_struct_params(struct_data);
    let struct_ident = struct_data.ident.clone();
    let struct_where = struct_data.generics.where_clause.clone();

    let matcher_type_name = format!("{}Matcher", struct_ident);
    let matcher_type_name: proc_macro2::TokenStream = matcher_type_name.parse().unwrap();

    let matcher_type = quote! {
      #[derive(Clone, Debug, Serialize, Deserialize)]
      pub struct #struct_params #matcher_type_name #struct_where {
        #field_decl
      }
    };

    let res = quote! {

      #matcher_type

      impl #struct_params Matcher for #struct_ident #struct_where {
        type AllMatcher = #matcher_type_name;
        fn match_all(&self, matcher: matcher_derive_impl::matcher::BaseMatcher<Self::AllMatcher>) -> bool {
          matcher_derive_impl::utils::match_with_vector_f(matcher, Some(self), |matcher, value| {
                #field_matches
            })
        }
    }

    };

    res
}
