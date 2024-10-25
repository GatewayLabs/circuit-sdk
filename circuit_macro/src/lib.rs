extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, BinOp, Expr, ExprBinary, ExprIf, ExprUnary, FnArg, ItemFn, Pat, PatType,
};

/// Macro to decorate functions and transform operators and if/else into circuit context operations
#[proc_macro_attribute]
pub fn circuit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident; // Function name
    let inputs = &input_fn.sig.inputs; // Function input parameters

    // We need to extract each input's identifier
    let mapped_inputs = inputs.iter().map(|input| {
        if let FnArg::Typed(PatType { pat, .. }) = input {
            if let Pat::Ident(pat_ident) = &**pat {
                let var_name = &pat_ident.ident;
                quote! {
                    let #var_name = &context.input(&#var_name.clone().into());
                }
            } else {
                quote! {}
            }
        } else {
            quote! {}
        }
    });

    // Replace "+" with context.add and handle if/else in the function body
    let transformed_block = modify_body(*input_fn.block);

    // Collect parameter names dynamically
    let param_names: Vec<_> = inputs
        .iter()
        .map(|input| {
            if let FnArg::Typed(PatType { pat, .. }) = input {
                if let Pat::Ident(pat_ident) = &**pat {
                    pat_ident.ident.clone()
                } else {
                    panic!("Expected identifier pattern");
                }
            } else {
                panic!("Expected typed argument");
            }
        })
        .collect();

    // Dynamically generate the `generate` function calls using the parameter names
    let match_arms = quote! {
        match std::any::type_name::<T>() {
            "u8" => generate::<8, T>(#(#param_names),*),
            "u16" => generate::<16, T>(#(#param_names),*),
            "u32" => generate::<32, T>(#(#param_names),*),
            "u64" => generate::<64, T>(#(#param_names),*),
            "u128" => generate::<128, T>(#(#param_names),*),
            _ => panic!("Unsupported type"),
        }
    };

    // Build the function body with circuit context, compile, and execute
    let expanded = quote! {
        fn #fn_name<T>(#inputs) -> T
        where
            T: Into<GarbledUint<8>>
                + From<GarbledUint<8>>
                + Into<GarbledUint<16>>
                + From<GarbledUint<16>>
                + Into<GarbledUint<32>>
                + From<GarbledUint<32>>
                + Into<GarbledUint<64>>
                + From<GarbledUint<64>>
                + Into<GarbledUint<128>>
                + From<GarbledUint<128>>
                + Clone,
        {
            fn generate<const N: usize, T>(#inputs) -> T
            where
                T: Into<GarbledUint<N>> + From<GarbledUint<N>> + Clone,
            {
                let mut context = CircuitBuilder::default();
                // Map each input to the circuit context's input function
                #(#mapped_inputs)*

                // Use the transformed function block (with context.add and if/else replacements)
                let output = {
                    #transformed_block
                };

                // Compile the circuit
                let compiled_circuit = context.compile(output);

                // Execute the circuit and get the result
                let result = context
                    .execute::<N>(&compiled_circuit)
                    .expect("Failed to execute the circuit");
                result.into()
            }

            #match_arms
        }
    };

    // Print the expanded code to stderr
    println!("Generated code:\n{}", expanded);

    TokenStream::from(expanded)
}

/// Traverse and transform the function body, replacing binary operators and if/else expressions.
fn modify_body(block: syn::Block) -> syn::Block {
    let stmts = block
        .stmts
        .into_iter()
        .map(|stmt| {
            match stmt {
                syn::Stmt::Expr(expr, semi_opt) => {
                    syn::Stmt::Expr(replace_expressions(expr), semi_opt)
                }
                syn::Stmt::Local(mut local) => {
                    if let Some(local_init) = &mut local.init {
                        // Replace the initializer expression
                        local_init.expr = Box::new(replace_expressions(*local_init.expr.clone()));
                    }
                    syn::Stmt::Local(local)
                }
                other => other,
            }
        })
        .collect();

    syn::Block {
        stmts,
        brace_token: syn::token::Brace::default(),
    }
}

/// Replaces binary operators and if/else expressions with appropriate context calls.
fn replace_expressions(expr: Expr) -> Expr {
    match expr {
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Add(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.add)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // subtraction
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Sub(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.sub)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // multiplication
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Mul(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.mul)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // division - TODO: Implement division
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Div(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.div)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // modulo - TODO: Implement modulo
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Rem(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.rem)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // bitwise AND
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitAnd(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.and)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // bitwise OR
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitOr(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.or)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // bitwise XOR
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitXor(_),
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.xor)),
                paren_token: syn::token::Paren::default(),
                args: vec![*left, *right].into_iter().collect(),
            })
        }
        // bitwise NOT
        Expr::Unary(ExprUnary {
            op: syn::UnOp::Not(_),
            expr,
            ..
        }) => {
            Expr::Call(syn::ExprCall {
                attrs: vec![], // No attributes on the call
                func: Box::new(syn::parse_quote!(context.not)),
                paren_token: syn::token::Paren::default(),
                args: vec![*expr].into_iter().collect(),
            })
        }
        // Handle if/else by translating to context.mux
        Expr::If(ExprIf {
            cond,
            then_branch,
            else_branch,
            ..
        }) => {
            if let Some((_, else_branch)) = else_branch {
                let then_expr = {
                    if then_branch.stmts.len() == 1 {
                        if let syn::Stmt::Expr(expr, _) = &then_branch.stmts[0] {
                            replace_expressions(expr.clone())
                        } else {
                            panic!("Expected a single expression in then branch");
                        }
                    } else {
                        panic!("Expected a single statement in then branch");
                    }
                };

                let else_expr = match *else_branch {
                    syn::Expr::Block(syn::ExprBlock { block, .. }) => {
                        if block.stmts.len() == 1 {
                            if let syn::Stmt::Expr(expr, _) = &block.stmts[0] {
                                replace_expressions(expr.clone())
                            } else {
                                panic!("Expected a single expression in else branch");
                            }
                        } else {
                            panic!("Expected a single statement in else branch");
                        }
                    }
                    _ => panic!("Expected a block in else branch"),
                };

                syn::parse_quote! {{
                    let if_true = &#then_expr;
                    let if_false = &#else_expr;
                    context.mux(#cond, if_true, if_false)
                }}
            } else {
                panic!("Expected else branch for if expression");
            }
        }
        other => other,
    }
}
