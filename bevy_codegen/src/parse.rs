use syn::Expr;

pub fn parse_file(file: syn::File) {
    for item in file.items {
        match item {
            syn::Item::Fn(fn_item) if fn_item.sig.ident.to_string().eq("main") => {
                if let Some(syn::Stmt::Semi(x, _)) = fn_item.block.stmts.first() {
                    println!("main1:\n{x:?}");
                    let mut r = parse_fn(vec![], Box::new(x.clone()));
                    r.reverse();
                    println!("{r:?}");
                }
            }
            _ => (),
        }
    }
}

fn parse_fn(mut init_app_builder: Vec<(String, String)>, expr: Box<Expr>) -> Vec<(String, String)> {
    match *expr {
        syn::Expr::Call(_) => (),
        syn::Expr::MethodCall(ref x) => {
            println!("Method: {}", x.method);
            let argument = x.args.clone().into_iter().filter_map(|f| match f {
                Expr::Path(expr_path) => expr_path
                    .path
                    .segments
                    .first()
                    .map(|segment| segment.ident.to_string()),
                Expr::Struct(x) => {
                    let struct_name = x
                        .path
                        .segments
                        .first()
                        .map(|segment| segment.ident.to_string());

                    let absct_fields = x.fields.into_iter().map(|f| {
                        let member_name = match f.member {
                            syn::Member::Named(x) => Some(x.to_string()),
                            syn::Member::Unnamed(_) => None,
                        };
                        let expr = match f.expr {
                            syn::Expr::Lit(a) => Some(match a.lit {
                                syn::Lit::Str(x) => x.token().to_string(),
                                syn::Lit::ByteStr(x) => x.token().to_string(),
                                syn::Lit::Byte(x) => x.token().to_string(),
                                syn::Lit::Char(x) => x.token().to_string(),
                                syn::Lit::Int(x) => x.token().to_string(),
                                syn::Lit::Float(x) => x.token().to_string(),
                                syn::Lit::Bool(x) => x.token().to_string(),
                                syn::Lit::Verbatim(x) => x.to_string(),
                            }),
                            _ => None,
                        };
                        println!("{member_name:?},{expr:?}");
                        (member_name, expr)
                    });
                    let fmt_fields = absct_fields.filter_map(|(a, b)| match (a, b) {
                        (None, None) => None,
                        (None, Some(y)) => Some(y),
                        (Some(x), None) => Some(x),
                        (Some(x), Some(y)) => Some(x + ":" + &y),
                    });
                    let joined_fields = fmt_fields.collect::<Vec<String>>().join(",");
                    match struct_name {
                        Some(x) => Some(x + "{" + &joined_fields + "}"),
                        None => Some(joined_fields),
                    }
                }
                _ => None,
            });
            let c = argument.collect::<Vec<String>>();
            println!("Arg: {c:?}");
            //println!("Other: {:?}", x);
            init_app_builder.push((x.method.to_string(), c.join(",")));
            return parse_fn(init_app_builder, x.receiver.clone());
        }
        _ => (),
    };
    init_app_builder
}
