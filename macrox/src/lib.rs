extern crate proc_macro;

use std::fs;

use proc_macro2::TokenStream;
// format_ident
use quote::quote;
use regex::Regex;
use serde::{Deserialize, Serialize};

// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

#[derive(Debug, Deserialize, Serialize)]
struct EnumDefinition {
    pub id: i32,
    pub key: String,
    pub zh: String,
    pub en: String,
}

#[proc_macro]
pub fn generate_i18n_enum(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let local_path = input.to_string();
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join("locales/locale.json");

    let json_data = fs::read_to_string(locales_path.as_path().to_str().unwrap()).unwrap();
    let str_list: Vec<EnumDefinition> = serde_json::from_str(&json_data).unwrap();
    // 如果返回的结构中带()这种特殊符号，会自动加上"，恶心
    // 获取 key
    let enum_case = generate_enum_case(&str_list);
    let trans_func = generate_func(&str_list);

    let code = quote! {
        #[derive(Debug,Clone)]
        pub enum I18nKey {
            #(#enum_case)*
        }

        impl I18nKey {
            pub fn trans(&self, locale: Locale) -> String {
                match self {
                    #(#trans_func)*
                }
            }
        }

        impl Default for I18nKey{
            fn default() -> Self {
                I18nKey::Ok
            }
        }

        #[derive(Debug, Clone, Copy, strum_macros::Display)]
        pub enum Locale {
            En,
            Zh,
        }

        impl Default for Locale {
            fn default() -> Self {
                Locale::En
            }
        }

        impl From<&str> for Locale {
            fn from(value: &str) -> Self {
                if value == Locale::En.to_string().as_str() {
                    return Locale::En;
                }
                Locale::Zh
            }
        }
    };
    code.into()
}

///
/// Author         : ClzSkywalker
/// Date           : 2023-07-21
/// Description    : 生成枚举的名字与参数
/// param           {*} str_list
/// return          {*}
///
fn generate_enum_case(str_list: &Vec<EnumDefinition>) -> Vec<TokenStream> {
    let enum_case = str_list
        .iter()
        .map(|variant| {
            let item = syn::Ident::new(&variant.key, proc_macro2::Span::call_site());

            let mut argsstr = String::from("");
            let re = Regex::new(r"%\{(\w+)\}").unwrap();
            let args: Vec<&str> = re
                .captures_iter(&variant.en)
                .map(|item| item.get(1).unwrap().as_str())
                .collect();
            if args.len() == 0 {
                return quote! {#item,};
            }

            // 处理注释
            // let mut about=String::from("");
            // about.push_str(args.join(",").as_str());
            // dbg!(about.to_string());
            // let about=about;
            // let _about=syn::Ident::new(&about, proc_macro2::Span::call_site());

            // 处理参数
            let five_fives = std::iter::repeat("String").take(args.len());
            let v = Vec::from_iter(five_fives).join(",");
            argsstr.push_str(v.as_str());   
            let args = syn::Ident::new(&argsstr, proc_macro2::Span::call_site());
            quote! {
                #item(#args),
            }
        })
        .collect::<Vec<_>>();

    enum_case
}

///
/// Author         : ClzSkywalker
/// Date           : 2023-07-21
/// Description    : 生成 trans方法
/// param           {*} str_list
/// return          {*}
///
fn generate_func(str_list: &Vec<EnumDefinition>) -> Vec<TokenStream> {
    str_list
        .iter()
        .map(|variant| {
            let enum_name = syn::Ident::new(&variant.key, proc_macro2::Span::call_site());
            let en_lang = &variant.en;
            let zh_lang = &variant.zh;
            // 正则匹配参数
            let re = Regex::new(r"%\{(\w+)\}").unwrap();
            let marches: Vec<_> = re.captures_iter(&variant.en).collect();
            if marches.len() == 0 {
                return quote! {
                    I18nKey::#enum_name => match locale {
                        Locale::En => {
                            String::from(#en_lang)
                        }
                        Locale::Zh => {
                            String::from(#zh_lang)
                        },
                    },
                };
            }

            let args_info: Vec<&str> = marches
                .iter()
                .map(|item| item.get(1).unwrap().as_str())
                .collect();

            // 获取参数
            let mut args = Vec::<String>::default();
            let mut replace_code_list = Vec::default();

            for index in 0..args_info.len() {
                let item = "item".to_string() + (index + 1).to_string().as_str();
                args.push(item.clone());
                let mut replace_indentity = String::from("%{");
                replace_indentity.push_str(args_info.get(index).unwrap());
                replace_indentity.push('}');

                let arg_name = syn::Ident::new(
                    &args_info.get(index).unwrap(),
                    proc_macro2::Span::call_site(),
                );

                let item = syn::Ident::new(&item, proc_macro2::Span::call_site());

                let code = quote! {
                    let #arg_name = #replace_indentity;
                    lang = lang.replace(#arg_name, #item);
                };

                replace_code_list.push(code);
            }
            let args_str = args.join(",");
            let args_str = syn::Ident::new(&args_str, proc_macro2::Span::call_site());
            let code = quote! {
                I18nKey::#enum_name(#args_str) => match locale {
                    Locale::En => {
                        let mut lang = String::from(#en_lang);
                        #(#replace_code_list)*
                        lang
                    }
                    Locale::Zh => {
                        let mut lang = String::from(#zh_lang);
                        #(#replace_code_list)*
                        lang
                    },
                },
            };
            code
        })
        .collect::<Vec<_>>()
}
