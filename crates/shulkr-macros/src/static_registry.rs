use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Expr, ExprCall, ExprPath, Ident, ImplItem, ImplItemConst, ItemImpl, LitInt, Path, Type};

fn self_ty_ident(self_ty: &Type) -> Ident {
    let Type::Path(type_path) = self_ty else {
        panic!("static_registry only supports plain type names, e.g. `impl Block`");
    };
    type_path
        .path
        .segments
        .last()
        .expect("type path must have a name")
        .ident
        .clone()
}

fn data_ty_ident(self_ident: &Ident) -> Ident {
    format_ident!("{}Data", self_ident)
}

fn retarget_path(path: &Path, self_ident: &Ident, data_ty: &Ident) -> Option<Path> {
    let first = path.segments.first()?;
    if first.ident != *self_ident {
        return None;
    }
    let mut path = path.clone();
    path.segments.first_mut().unwrap().ident = data_ty.clone();
    Some(path)
}

fn retarget_ctor(expr: &Expr, self_ident: &Ident, data_ty: &Ident) -> TokenStream {
    match expr {
        Expr::Call(ExprCall { func, args, .. }) => {
            if let Expr::Path(ExprPath {
                path, qself: None, ..
            }) = func.as_ref()
                && let Some(path) = retarget_path(path, self_ident, data_ty)
            {
                return quote! { #path(#args) };
            }
        }
        Expr::Struct(expr_struct) => {
            if let Some(path) = retarget_path(&expr_struct.path, self_ident, data_ty) {
                let mut expr_struct = expr_struct.clone();
                expr_struct.path = path;
                return quote! { #expr_struct };
            }
        }
        _ => {}
    }

    quote! { #expr }
}

pub fn expand(input: ItemImpl) -> TokenStream {
    let self_ty = &input.self_ty; // `Block`
    let self_ident = self_ty_ident(self_ty); // `Block`
    let data_ty = data_ty_ident(&self_ident); // `BlockData`

    let mod_ident = format_ident!(
        "__{}_static_registry",
        data_ty.to_string().to_case(Case::Snake)
    );

    let mut consts = Vec::new();
    let mut statics = Vec::new();
    let mut match_arms = Vec::new();
    let mut next_id: u16 = 0;

    for impl_item in &input.items {
        let ImplItem::Const(ImplItemConst { ident, expr, .. }) = impl_item else {
            continue;
        };

        let id = next_id;
        next_id += 1;
        let id_lit = LitInt::new(&id.to_string(), Span::call_site());

        // pub const AIR: Block = Block(0);
        consts.push(quote! {
            pub const #ident: #self_ty = #self_ty(#id_lit);
        });

        // pub static AIR: BlockData = BlockData::new(0, 0, &[], None);
        let ctor = retarget_ctor(expr, &self_ident, &data_ty);
        statics.push(quote! {
            pub static #ident: #data_ty = #ctor;
        });

        // 0 => &static_objects::AIR,
        match_arms.push(quote! {
            #id_lit => &#mod_ident::#ident,
        });
    }

    quote! {
        impl #self_ty {
            #(#consts)*
        }

        mod #mod_ident {
            use super::*;

            #(#statics)*
        }

        impl #self_ty {
            #[inline]
            pub const fn data(self) -> &'static #data_ty {
                match self.0 {
                    #(#match_arms)*
                    _ => unreachable!(),
                }
            }
        }
    }
}
