use syn::Expr;

pub fn parse_file(file: syn::File) -> Option<Vec<(String, String)>> {
    for item in file.items {
        match item {
            syn::Item::Fn(fn_item) if fn_item.sig.ident.to_string().eq("main") => {
                if let Some(syn::Stmt::Semi(x, _)) = fn_item.block.stmts.first() {
                    println!("main:\n{x:?}");
                    let mut r = parse_fn(vec![], Box::new(x.clone()));
                    r.reverse();
                    println!("{r:?}");
                    return Some(r);
                }
            }
            _ => (),
        }
    }
    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_building_a_bevy_app_finds_systems() {
        let syntax = syn::parse_file(
            r#"fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(fade_transparency)
        .run();
    }"#,
        )
        .expect("Unable to parse file");
        let res = parse_file(syntax);
        assert_eq!(
            res,
            Some(vec![
                ("insert_resource".to_string(), "Msaa{samples:4}".to_string()),
                ("add_plugins".to_string(), "DefaultPlugins".to_string()),
                ("add_startup_system".to_string(), "setup".to_string()),
                ("add_system".to_string(), "fade_transparency".to_string()),
                ("run".to_string(), "".to_string())
            ])
        );
    }
}
