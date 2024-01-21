use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, Type,
    punctuated::Punctuated, Field, token::Comma,
    Data, Fields, Lit, Meta, Token, Expr,
};
use quote::quote;
use proc_macro2::Span;

/// 获取带名称的字段列表
fn get_named_fields(input: &DeriveInput) -> Option<&Punctuated<Field, Comma>> {
    if let Data::Struct(ref s) = input.data {
        if let Fields::Named(ref named_fields) = s.fields {
            return Some(&named_fields.named);
        }
    }

    unimplemented!("derive builder 目前只支持带名称的字段");
}

/// builder 属性参数
struct BuilderAttribute {
    /// 属性 key
    pub key: String,
    // 属性值
    pub value: Option<String>,
    // 属性 meta path, 用于错误提示位置显示
    // 类似：
    //   --> main.rs:14:7
    //    |
    // 14 |     #[builder(eac = "arg")]
    //    |       ^^^^^^^^^^^^^^^^^^^^
    pub meta: Meta,
}

/// 属性参数解析
/// #[builder(each = "env")]
/// 转为
/// [BuilderAttribute { key: "each", value: Some("env"), meta: syn::Meta }]
fn parse_builder_attr(attr: &syn::Attribute) -> Vec<BuilderAttribute> {
    let mut arguments = Vec::new();

    if attr.path().is_ident("builder") {
        // parse_args_with 参考:
        // https://docs.rs/syn/2.0.48/syn/struct.Attribute.html#alternatives
        // nested 类型 Vec<Meta::NameValue>
        if let Ok(nested) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
            for meta in &nested {
                if let Some(ident) = meta.path().get_ident() {
                    let key = ident.to_string();
                    let mut value: Option<String> = None;

                    if let Meta::NameValue(name_value) = meta {
                        if let Expr::Lit(ref expr_lit) = name_value.value {
                            value = match expr_lit.lit {
                                Lit::Str(ref lit_str) => Some(lit_str.value()),
                                Lit::Bool(ref lit_bool) => Some(lit_bool.value.to_string()),
                                Lit::Int(ref lit_int) => Some(lit_int.base10_digits().to_owned()),
                                _ => None,
                            }
                        }
                    };

                    arguments.push(BuilderAttribute {
                        key,
                        value,
                        meta: attr.meta.clone(),
                    });
                }
            }
        }
    }

    arguments
}

/// 获取当前类型具体内部类型
/// Option<String> 获取到 String
fn extract_inner_type<'a>(ty: &'a Type, expected_type_wrapper: &str) -> Option<&'a Type> {
    if let Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if let std::option::Option::Some(syn::PathSegment {
            ident,
            arguments:
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }),
        }) = segments.last()
        {
            if ident == expected_type_wrapper {
                if let std::option::Option::Some(syn::GenericArgument::Type(ty)) = args.last() {
                    return std::option::Option::Some(ty);
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    let ident = &input.ident;
    let builder_name = Ident::new(&format!("{}Builder", &ident), Span::call_site());
    // 获取所有 Fields::Named 类型的字段数据
    let fields = get_named_fields(&input).unwrap();

    // CommandBuilder 默认值 
    let builder_default_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        // Vec 类型初始化为 Some(vec!()) 空数组
        if extract_inner_type(ty, "Vec").is_some() {
            return quote! { #name: std::option::Option::Some(vec!()) };
        }
        
        quote! { #name: std::option::Option::None }
    });

    // CommandBuilder 结构体声明 字段参数
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        // 是 Option 类型的字段 直接指定为原类型
        if extract_inner_type(ty, "Option").is_some() {
            return quote! { #name: #ty };
        } else {
            // 非Otion 类型转为 Option<T>
            return quote! { #name: std::option::Option<#ty> };
        }
    });

    // CommandBuilder setter 方法生成
    // pub fn executable(&mut self, executable: String) -> &mut Self {
    //   self.executable = std::option::Option::Some(executable);
    //   self
    // }
    // 
    // args env 可以根据 each 属性指定单数据参数 setter 方法
    // 
    let setter_fns = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        // 如果是 Option<T> 获取内部类型T
        // 否则使用原类型
        let inner_type = extract_inner_type(ty, "Option").unwrap_or(ty);

        // 不存在属性参数
        if f.attrs.is_empty() {
            return quote! {
                pub fn #name(&mut self, #name: #inner_type) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            };
        }

        // println!("attr: {:#?}", &f);
        for attr in f.attrs.iter() {
            let arguments = parse_builder_attr(attr);
            for argument in arguments.iter() {
                let key = argument.key.as_str();

                match key {
                    "each" => {
                        // 获取 each 指定的属性值
                        if let Some(attr_value) = &argument.value {
                            let vec_inner_type = extract_inner_type(ty, "Vec").expect("Vec 内部类型获取失败");
                            // 字段名称转为字符串
                            let name_str = name.clone().unwrap().to_string();
                            // each 属性值String转为 Ident
                            let each_name_ident = Ident::new(attr_value.as_str(), Span::call_site());
                            // each = "env"
                            // 指定的单参数函数名和多参数函数名一样只实现单参数函数
                            if name_str.as_str() == attr_value.as_str() {
                                return quote! {
                                    pub fn #each_name_ident(&mut self, #each_name_ident: #vec_inner_type) -> &mut Self {
                                        if let std::option::Option::Some(ref mut v) = self.#name {
                                            v.push(#each_name_ident);
                                        } else {
                                            self.#name = std::option::Option::Some(vec![#each_name_ident]);
                                        }
                                        self
                                    }
                                };
                            } else {
                                // 否则单参数和多参数函数同时实现
                                return quote! {
                                    pub fn #name(&mut self, #name: #inner_type) -> &mut Self {
                                        self.#name = std::option::Option::Some(#name);
                                        self
                                    }
                                    pub fn #each_name_ident(&mut self, #each_name_ident: #vec_inner_type) -> &mut Self {
                                        if let std::option::Option::Some(ref mut v) = self.#name {
                                            v.push(#each_name_ident);
                                        } else {
                                            self.#name = std::option::Option::Some(vec![#each_name_ident]);
                                        }
                                        self
                                    }
                                };
                            }
                        }
                    },
                    _ => {                      
                        // 错误信息提示
                        //     |
                        //  14 |     #[builder(eac = "arg")]
                        //     |       ^^^^^^^^^^^^^^^^^^^^
                        return syn::Error::new_spanned(&argument.meta, "expected `builder(each = \"...\")`")
                            .to_compile_error()
                    }
                }
            }
        }

        unimplemented!("未实现的builder属性");
    });

    // builder build 方法设置值
    let builder_build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        // 字段名称转为字符串
        let name_str = name.clone().unwrap().to_string();

        // Command应字段是Option类型 直接复制数据
        if extract_inner_type(ty, "Option").is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            // Command对应字段非Option 需要将CommandBuilder Option数据解构出来再符值
            quote! {
                #name: self.#name.clone().ok_or(std::concat!(#name_str, " needed"))?
            }
        }
    });

    let expanded = quote! {
        // 原Command结构体实现 builder 方法
        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_default_fields,)*
                }
            }
        }

        // 建造者结构体 CommandBuilder
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            // 建造者 setters
            #(#setter_fns)*

            // 建造者 build 生成实体
            pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                let command = #ident {
                    #(#builder_build_fields,)*
                };

                Ok(command)
            }
        }
    };

    TokenStream::from(expanded)
}
