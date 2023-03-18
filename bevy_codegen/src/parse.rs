use syn::{Expr, UseGroup, UseTree};

pub fn parse_file(file: syn::File) -> Option<ParseBevyModel> {
    //println!("all:\n{file:#?}");
    let mut pbm = ParseBevyModel::default();
    let mut imports = vec![];
    for item in file.items {
        match item {
            syn::Item::Fn(fn_item) if fn_item.sig.ident.to_string().eq("main") => {
                if let Some(syn::Stmt::Expr(x, _)) = fn_item.block.stmts.first() {
                    //println!("main:\n{x:#?}");
                    let mut r = parse_fn(ParseBevyModel::default(), Box::new(x.clone()));
                    r.app_builder.reverse();
                    //println!("{:?}", r.app_builder);
                    pbm.app_builder = r.app_builder;
                }
            }
            syn::Item::Use(use_item) => {
                let rs = parse_use(vec![], use_item.tree).join("::");
                //println!("Use: {rs:?}");
                imports.push(rs);
            }
            _ => (),
        }
    }

    pbm.imports = imports;

    if pbm.eq(&ParseBevyModel::default()) {
        None
    } else {
        Some(pbm)
    }
}

fn parse_use(mut imports: Vec<String>, use_tree: UseTree) -> Vec<String> {
    let res = match use_tree {
        syn::UseTree::Path(x) => {
            imports.push(x.ident.to_string());
            parse_use(imports, *x.tree)
        }
        syn::UseTree::Name(x) => {
            imports.push(x.ident.to_string());
            imports
        }
        syn::UseTree::Rename(_) => imports,
        syn::UseTree::Glob(_) => {
            imports.push("*".to_string());
            imports
        }
        syn::UseTree::Group(x) => {
            parse_group(x)
                .iter()
                .for_each(|f| imports.push(f.to_string()));
            imports
        }
    };
    res
}

fn parse_group(group: UseGroup) -> Vec<String> {
    let mut imports = vec![];
    //println!("Group:\n{:#?}", group.items);
    let g = group
        .items
        .into_iter()
        .map(|f| parse_use(imports.clone(), f));
    let gg = g.collect::<Vec<Vec<String>>>();
    //println!("VV: {:#?}", gg);
    let mut b = vec![];
    for a in &gg {
        let l = a.split_last();
        if let Some((bb, cc)) = l {
            let mut oo = cc.join("::");
            if !cc.is_empty() {
                oo += "::";
            }
            oo += bb;
            b.push(oo);
        }
    }
    //println!("WW\n{b:#?}");
    imports.push("{".to_string() + &b.join(",") + "}");
    imports
}

#[derive(PartialEq, Eq, Debug, Default)]
pub struct ParseBevyModel {
    pub imports: Vec<String>,
    pub app_builder: Vec<(String, String)>,
}

#[allow(clippy::boxed_local)]
fn parse_fn(mut init_app_builder: ParseBevyModel, expr: Box<Expr>) -> ParseBevyModel {
    match *expr {
        syn::Expr::MethodCall(ref x) => {
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

                    let abstract_fields = x.fields.into_iter().map(|f| {
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
                                _ => todo!(),
                            }),
                            _ => None,
                        };
                        (member_name, expr)
                    });
                    let fmt_fields = abstract_fields.filter_map(|(a, b)| match (a, b) {
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
            let tf = match x.turbofish.clone() {
                Some(x) => {
                    let res = x
                        .args
                        .into_iter()
                        .map(|f| match f {
                            syn::GenericArgument::Type(syn::Type::Path(z)) => {
                                z.path.segments.first().unwrap().ident.to_string()
                            }
                            _ => "".to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join(",");
                    if res.is_empty() {
                        None
                    } else {
                        Some(res)
                    }
                }
                None => None,
            };
            let method = match tf {
                Some(y) => x.method.to_string() + "::<" + &y + ">",
                None => x.method.to_string(),
            };
            println!("Method: {method:?}");
            println!("Arg: {c:?}");
            init_app_builder.app_builder.push((method, c.join(",")));
            return parse_fn(init_app_builder, x.receiver.clone());
        }
        Expr::Await(_) => (),
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
        .test::<A,B>()
        .add_system(fade_transparency)
        .run();
    }"#,
        )
        .expect("Unable to parse file");
        let res = parse_file(syntax);
        assert_eq!(
            res,
            Some(ParseBevyModel {
                app_builder: vec![
                    ("insert_resource".to_string(), "Msaa{samples:4}".to_string()),
                    ("add_plugins".to_string(), "DefaultPlugins".to_string()),
                    ("add_startup_system".to_string(), "setup".to_string()),
                    ("test::<A,B>".to_string(), "".to_string()),
                    ("add_system".to_string(), "fade_transparency".to_string()),
                    ("run".to_string(), "".to_string())
                ],
                ..Default::default()
            })
        );
    }

    #[test]
    fn parse_use() {
        let bevy_file = r#"
        use bevy::{
            asset::{AssetIo, AssetIoError, Metadata},
            prelude::*,
            utils::BoxedFuture,
        };
        use std::path::{Path, PathBuf};
        "#;

        let res = parse_file(syn::parse_file(bevy_file).expect("Unable to parse file"));
        println!("{res:?}");
        assert_eq!(
            res,
            Some(ParseBevyModel {
                imports: vec![
                    "bevy::{asset::{AssetIo,AssetIoError,Metadata},prelude::*,utils::BoxedFuture}"
                        .to_string(),
                    "std::path::{Path,PathBuf}".to_string()
                ],
                ..Default::default()
            })
        );
    }

    #[test]
    fn parse_full_bevy_file() {
        let bevy_file = r#"
        use bevy::prelude::*;

        fn main() {
            App::new()
                .insert_resource(Msaa { samples: 4 })
                .add_plugins(DefaultPlugins)
                .add_startup_system(setup)
                .add_system(fade_transparency)
                .run();
        }
        
        fn setup(
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<StandardMaterial>>,
        ) {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 6.0 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            });
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 3,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                    alpha_mode: AlphaMode::Mask(0.5),
                    ..default()
                }),
                transform: Transform::from_xyz(1.0, 0.5, -1.5),
                ..default()
            });
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 3,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
                    alpha_mode: AlphaMode::Mask(0.5),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(-1.0, 0.5, -1.5),
                ..default()
            });
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.0).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            });
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 3,
                })),
                material: materials.add(Color::rgb(0.7, 0.2, 0.1).into()),
                transform: Transform::from_xyz(0.0, 0.5, -1.5),
                ..default()
            });
            
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_xyz(4.0, 8.0, 4.0),
                ..default()
            });
            
            commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        }
        
        pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
            let alpha = (time.elapsed_seconds().sin() / 2.0) + 0.5;
            for (_, material) in materials.iter_mut() {
                material.base_color.set_a(alpha);
            }
        }
        "#;

        let res = parse_file(syn::parse_file(bevy_file).expect("Unable to parse file"));
        assert_eq!(
            res,
            Some(ParseBevyModel {
                imports: vec!["bevy::prelude::*".to_string()],
                app_builder: vec![
                    ("insert_resource".to_string(), "Msaa{samples:4}".to_string()),
                    ("add_plugins".to_string(), "DefaultPlugins".to_string()),
                    ("add_startup_system".to_string(), "setup".to_string()),
                    ("add_system".to_string(), "fade_transparency".to_string()),
                    ("run".to_string(), "".to_string())
                ],
                ..Default::default()
            })
        );
    }
}
