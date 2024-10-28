extern crate proc_macro;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{
    parse_macro_input, BinOp, Expr, ExprBinary, ExprIf, ExprUnary, FnArg, ItemFn, Lit, Pat, PatType,
};

#[proc_macro_attribute]
pub fn circuit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mode = parse_macro_input!(attr as syn::Ident).to_string(); // Retrieve the mode (e.g., "compile" or "execute")
    generate_macro(item, &mode)
}

/// Generates the macro code based on the mode (either "compile" or "execute")
fn generate_macro(item: TokenStream, mode: &str) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident; // Function name
    let inputs = &input_fn.sig.inputs; // Function input parameters

    // get the type of the first input parameter
    let type_name = if let FnArg::Typed(PatType { ty, .. }) = &inputs[0] {
        quote! {#ty}
    } else {
        panic!("Expected typed argument");
    };

    // get the type of the first output parameter
    let output_type = if let syn::ReturnType::Type(_, ty) = &input_fn.sig.output {
        quote! {#ty}
    } else {
        panic!("Expected typed return type");
    };

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

    // Extract constants to be added at the top of the function
    let mut constants = vec![];
    let transformed_block = modify_body(*input_fn.block, &mut constants);

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
        match std::any::type_name::<#type_name>() {
            "bool" => generate::<1, #type_name>(#(#param_names),*),
            "u8" => generate::<8, #type_name>(#(#param_names),*),
            "u16" => generate::<16, #type_name>(#(#param_names),*),
            "u32" => generate::<32, #type_name>(#(#param_names),*),
            "u64" => generate::<64, #type_name>(#(#param_names),*),
            "u128" => generate::<128, #type_name>(#(#param_names),*),
            _ => panic!("Unsupported type"),
        }
    };

    // Set the output type and operation logic based on mode
    let output_type = if mode == "compile" {
        quote! {(Circuit, Vec<bool>)}
    } else {
        quote! {#output_type}
    };

    let operation = if mode == "compile" {
        quote! {
            (context.compile(&output), context.inputs().to_vec())
        }
    } else {
        quote! {
            let compiled_circuit = context.compile(&output.into());
            let result = context.execute::<N>(&compiled_circuit).expect("Execution failed");
            result.into()
        }
    };

    // Build the function body with circuit context, compile, and execute
    let expanded = quote! {
        #[allow(non_camel_case_types, non_snake_case, clippy::builtin_type_shadow, clippy::too_many_arguments)]
        fn #fn_name<#type_name>(#inputs) -> #output_type
        where
        #type_name: Into<GarbledUint<1>> + From<GarbledUint<1>>
                + Into<GarbledUint<8>> + From<GarbledUint<8>>
                + Into<GarbledUint<16>> + From<GarbledUint<16>>
                + Into<GarbledUint<32>> + From<GarbledUint<32>>
                + Into<GarbledUint<64>> + From<GarbledUint<64>>
                + Into<GarbledUint<128>> + From<GarbledUint<128>>
                + Clone,
        {
            fn generate<const N: usize, #type_name>(#inputs) -> #output_type
            where
                #type_name: Into<GarbledUint<N>> + From<GarbledUint<N>> + Clone,
            {
                let mut context = CircuitBuilder::default();
                #(#mapped_inputs)*
                #(#constants)*
                let const_true = &context.input::<N>(&1u128.into());
                let const_false = &context.input::<N>(&0u128.into());

                // Use the transformed function block (with context.add and if/else replacements)
                let output = { #transformed_block };

                #operation
            }

            #match_arms
        }
    };

    // Print the expanded code to stderr
    // println!("Generated code:\n{}", expanded);

    TokenStream::from(expanded)
}

/// Traverse and transform the function body, replacing binary operators and if/else expressions.
/// Also collects constants to add to the circuit context.
fn modify_body(block: syn::Block, constants: &mut Vec<proc_macro2::TokenStream>) -> syn::Block {
    let stmts = block
        .stmts
        .into_iter()
        .map(|stmt| {
            match stmt {
                syn::Stmt::Expr(expr, semi_opt) => {
                    syn::Stmt::Expr(replace_expressions(expr, constants), semi_opt)
                }
                syn::Stmt::Local(mut local) => {
                    if let Some(local_init) = &mut local.init {
                        // Replace the initializer expression
                        local_init.expr =
                            Box::new(replace_expressions(*local_init.expr.clone(), constants));
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
fn replace_expressions(expr: Expr, constants: &mut Vec<proc_macro2::TokenStream>) -> Expr {
    match expr {
        // return statement
        Expr::Return(_) => {
            panic!("Return statement not allowed in circuit macro");
        }
        // boolean literal
        Expr::Lit(syn::ExprLit {
            lit: Lit::Bool(lit_bool),
            ..
        }) => {
            let value = lit_bool.value;
            let const_var = format_ident!("const_{}", value as u128);
            constants.push(quote! {
                let #const_var = &context.input::<N>(&#value.into());
            });
            syn::parse_quote! {#const_var}
        }
        // integer literal
        Expr::Lit(syn::ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => {
            let value = lit_int
                .base10_parse::<u128>()
                .expect("Expected an integer literal");
            let const_var = format_ident!("const_{}", value);
            constants.push(quote! {
                let #const_var = &context.input::<N>(&#value.into());
            });
            syn::parse_quote! {#const_var}
        }
        // Handle parentheses to ensure proper order of operations
        Expr::Paren(expr_paren) => {
            let inner_expr = replace_expressions(*expr_paren.expr, constants);
            syn::parse_quote! { (#inner_expr) }
        }

        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Eq(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.eq(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Ne(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.ne(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Gt(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.gt(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Ge(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.ge(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Lt(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.lt(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Le(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.le(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Add(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.add(&left.into(), &right.into())
            }}
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::AddAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.add(&#left, &#right)
            }
        }
        // subtraction
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Sub(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.sub(&left.into(), &right.into())
            }}
        }
        // subtraction assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::SubAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.sub(&#left, &#right)
            }
        }

        // multiplication
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Mul(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.mul(&left.into(), &right.into())
            }}
        }

        // multiplication assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::MulAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.mul(&#left, &#right)
            }
        }
        // division
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Div(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.div(&left.into(), &right.into())
            }}
        }
        // division assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::DivAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.div(&#left, &#right)
            }
        }
        // modulo
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Rem(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.rem(&left.into(), &right.into())
            }}
        }
        // modulo assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::RemAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.rem(&#left, &#right)
            }
        }
        // logical AND
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::And(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.land(&left, &right)
            }}
        }

        // logical OR
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Or(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.lor(&left, &right)
            }}
        }

        // bitwise AND
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitAnd(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.and(&left.into(), &right.into())
            }}
        }
        // bitwise AND assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitAndAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.and(&#left, &#right)
            }
        }

        // bitwise OR
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitOr(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.or(&left.into(), &right.into())
            }}
        }
        // bitwise OR assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitOrAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.or(&#left, &#right)
            }
        }

        // bitwise XOR
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitXor(_),
            ..
        }) => {
            let left_expr = replace_expressions(*left, constants);
            let right_expr = replace_expressions(*right, constants);
            syn::parse_quote! {{
                let left = #left_expr;
                let right = #right_expr;
                &context.xor(&left.into(), &right.into())
            }}
        }
        // bitwise XOR assignment
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::BitXorAssign(_),
            ..
        }) => {
            syn::parse_quote! {
                &context.xor(&#left, &#right)
            }
        }

        // bitwise NOT
        Expr::Unary(ExprUnary {
            op: syn::UnOp::Not(_),
            expr,
            ..
        }) => {
            let single_expr = replace_expressions(*expr, constants);
            syn::parse_quote! {{
                let single = #single_expr;
                &context.not(&single.into())
            }}
        }

        Expr::If(ExprIf {
            cond,
            then_branch,
            else_branch,
            ..
        }) => {
            let cond_expr = replace_expressions(*cond, constants);
            let then_block = modify_body(then_branch, constants);

            if let Some((_, else_expr)) = else_branch {
                match *else_expr {
                    Expr::If(else_if) => {
                        let else_if_expr = replace_expressions(Expr::If(else_if), constants);
                        syn::parse_quote! {{
                            let if_true = #then_block;
                            let if_false = #else_if_expr;
                            let cond = #cond_expr;
                            &context.mux(cond.into(), if_true, if_false)
                        }}
                    }
                    _ => {
                        let else_block = modify_body(syn::parse_quote! { #else_expr }, constants);
                        syn::parse_quote! {{
                            let if_true = #then_block;
                            let if_false = #else_block;
                            let cond = #cond_expr;
                            &context.mux(cond.into(), if_true, if_false)
                        }}
                    }
                }
            } else {
                syn::parse_quote! {{
                    let if_true = #then_block;
                    //let if_false = context.len() + 1;
                    let cond = #cond_expr;
                    let if_false = &context.len();
                    &context.mux(cond.into(), &if_true, &if_false.into());
                }}
            }
        }

        other => other,
    }
}
