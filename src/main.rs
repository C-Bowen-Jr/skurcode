#![allow(non_snake_case)]
#![allow(unused_variables)]
#[allow(dead_code)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use std::fs;
use std::io;
use std::path::Path;
//use std::collections::HashMap;

#[derive(PartialEq)]
enum Action {
    None,
    Retire,
    Restore,
    Inspect,
}

#[derive(PartialEq, Props)]
struct Product {
    #[props(into)]
    content: String,
}

fn ClickableProduct(cx: Scope<Product>) -> Element {
    let selected_sku = use_shared_state::<Selection>(cx).unwrap();

    let input_sku = move |sku: String| {
        selected_sku.write().sku = sku;
    };
    let maybe = cx.props.content.clone();
    cx.render(rsx! {
        img {
            src: "./products/{cx.props.content}",
            // cannot move out of variables, captured by closure
            //onclick: move |_| selected_sku.write().sku = maybe,
        }
    })
}
#[derive(PartialEq, Props)]
struct Selection {
    #[props(into)]
    sku: String,
}

fn SelectedSku(cx: Scope) -> Element {
    let selected_sku = use_shared_state::<Selection>(cx).unwrap();
    cx.render(rsx! {
        //"{cx.props.sku}"
        "{selected_sku.read().sku}"
    })
}

fn ProductGrid(cx: Scope) -> Element {
    //let product_array: ProductArray;
    //product_array.all = get_image_file_names().unwrap();
    let products = get_image_file_names().unwrap();
    let selected_sku = use_shared_state::<Selection>(cx).unwrap();

    let input_sku = move |sku: String| {
        selected_sku.write().sku = sku;
    };

    // cannot return reference to local data 'products'
    // cant handle this with loop atm
    cx.render(rsx! {
        img {
            src: "./products/AWOOS.png",
            onclick: move |_| input_sku("AWOOS".to_string()),
        }
        img {
            src: "./products/BROCK.png",
            onclick: move |_| input_sku("BROCK".to_string()),
        }
        img {
            src: "./products/CAFDEF.png",
            onclick: move |_| input_sku("CAFDEF".to_string()),
        }
        img {
            src: "./products/CATSNEK.png",
            onclick: move |_| input_sku("CATSNEK".to_string()),
        }
        img {
            src: "./products/CHEESETAX.png",
            onclick: move |_| input_sku("CHEESETAX".to_string()),
        }
        img {
            src: "./products/CMHANA.png",
            onclick: move |_| input_sku("CMHANA".to_string()),
        }
        img {
            src: "./products/CORVTAXI.png",
            onclick: move |_| input_sku("CORVTAXI".to_string()),
        }
        img {
            src: "./products/CRYDOOK.png",
            onclick: move |_| input_sku("CRYDOOK".to_string()),
        }
        img {
            src: "./products/DSCORPS.png",
            onclick: move |_| input_sku("DSCORPS".to_string()),
        }
        img {
            src: "./products/FILIGRAVE_COO.png",
            onclick: move |_| input_sku("FILIGRAVE_COO".to_string()),
        }
        img {
            src: "./products/FILIGRAVE_WRM.png",
            onclick: move |_| input_sku("FILIGRAVE_WRM".to_string()),
        }
        img {
            src: "./products/FIREHAWK.png",
            onclick: move |_| input_sku("FIREHAWK".to_string()),
        }
        img {
            src: "./products/FOLIKED.png",
            onclick: move |_| input_sku("FOLIKED".to_string()),
        }
        img {
            src: "./products/GILLIAM.png",
            onclick: move |_| input_sku("GILLIAM".to_string()),
        }
        img {
            src: "./products/GITHUB.png",
            onclick: move |_| input_sku("GITHUB".to_string()),
        }
        img {
            src: "./products/GOCINT_GLD.png",
            onclick: move |_| input_sku("GOCINT_GLD".to_string()),
        }
        img {
            src: "./products/GOCINT_RED.png",
            onclick: move |_| input_sku("GOCINT_RED".to_string()),
        }
        img {
            src: "./products/JOJO_BLK.png",
            onclick: move |_| input_sku("JOJO_BLK".to_string()),
        }
        img {
            src: "./products/JOJO_BRN.png",
            onclick: move |_| input_sku("JOJO_BRN".to_string()),
        }
        img {
            src: "./products/KWEH_SPC.png",
            onclick: move |_| input_sku("KWEH_SPC".to_string()),
        }
        img {
            src: "./products/KWEH_YEL.png",
            onclick: move |_| input_sku("KWEH_YEL".to_string()),
        }
        img {
            src: "./products/LOVEPEACE.png",
            onclick: move |_| input_sku("LOVEPEACE".to_string()),
        }
        img {
            src: "./products/MERCHGUILD.png",
            onclick: move |_| input_sku("MERCHGUILD".to_string()),
        }
        img {
            src: "./products/MTCEXP.png",
            onclick: move |_| input_sku("MTCEXP".to_string()),
        }
        img {
            src: "./products/NOSKU.png",
            onclick: move |_| input_sku("NOSKU".to_string()),
        }
        img {
            src: "./products/PTGUN.png",
            onclick: move |_| input_sku("PTGUN".to_string()),
        }
        img {
            src: "./products/RAMPTF.png",
            onclick: move |_| input_sku("RAMPTF".to_string()),
        }
        img {
            src: "./products/RK9.png",
            onclick: move |_| input_sku("RK9".to_string()),
        }
        img {
            src: "./products/SAFARI.png",
            onclick: move |_| input_sku("SAFARI".to_string()),
        }
        img {
            src: "./products/SKOOMA_BRN.png",
            onclick: move |_| input_sku("SKOOMA_BRN".to_string()),
        }
        img {
            src: "./products/SKOOMA_GRY.png",
            onclick: move |_| input_sku("SKOOMA_GRY".to_string()),
        }
        img {
            src: "./products/SNFTS.png",
            onclick: move |_| input_sku("SNFTS".to_string()),
        }
        img {
            src: "./products/SWANSON.png",
            onclick: move |_| input_sku("SWANSON".to_string()),
        }
        img {
            src: "./products/TDB_GNS.png",
            onclick: move |_| input_sku("TDB_GNS".to_string()),
        }
        img {
            src: "./products/TDB_PLG.png",
            onclick: move |_| input_sku("TDB_PLG".to_string()),
        }
        img {
            src: "./products/TOOTHDIM.png",
            onclick: move |_| input_sku("TOOTHDIM".to_string()),
        }
        img {
            src: "./products/WRKHORSE.png",
            onclick: move |_| input_sku("WRKHORSE".to_string()),
        }
        img {
            src: "./products/XPFAIL.png",
            onclick: move |_| input_sku("XPFAIL".to_string()),
        }
        img {
            src: "./products/ZONAI.png",
            onclick: move |_| input_sku("ZONAI".to_string()),
        }
    })
    /*cx.render(rsx! {
        product_array.all.iter().map(|sku| rsx!{
            img {
                src: "./products/{sku}",
                //onclick: move |_| input_sku(format!("{}",sku.clone())),
            }
        })
    })*/
}

fn get_image_file_names() -> Result<Vec<String>, io::Error> {
    let products_folder = Path::new("products");
    let mut file_names = Vec::new();

    if products_folder.is_dir() {
        for entry in fs::read_dir(products_folder)? {
            let entry = entry?;
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                file_names.push(file_name.to_string());
            }
        }
    }
    // should actually remove all .png endings, then
    // add them back in img.src:
    Ok(file_names)
}

fn App(cx: Scope) -> Element {
   // let products = get_image_file_names().unwrap();
    //let selected_sku = use_state(cx, || "none".to_string());
    use_shared_state_provider(cx, || Selection{sku:"none".to_string()});
    //let products = use_state(cx, || ProductArray::new);
    let products = get_image_file_names().unwrap();
    let selected_sku = use_shared_state::<Selection>(cx).unwrap();
    let mut quantity = use_state(cx, || 0);
    let mut action = use_state(cx, || Action::None);
    let mut action_text = "".to_string();

    cx.render(rsx!(
        style { include_str!("../assets/skurcode.css") }
        div { class: "view",
            div { class: "product-grid",
                ProductGrid{},
            }
            div { class: "action-buffer"},
            div { class: "action-tray",
                div{ class: "action-text",
                    onclick: move |_| println!("generate QR code"),
                    if action.get() != &Action::None{
                        match action.get() {
                            &Action::Inspect => action_text.push_str("inspect:"),
                            &Action::Retire => action_text.push_str("retire:"),
                            &Action::Restore => action_text.push_str("restore:"),
                            _ => println!("Impossible"),
                        }
                    },
                    SelectedSku{},
                },
                button { class: "sell-button",
                    onclick: move |_| quantity -= 1,
                    "Sell"
                },
                button { class: "stock-button",
                    onclick: move |_| quantity += 1,
                    "Stock"
                }
                button { class: "inspect-button",
                    onclick: move |_| action.set(Action::Inspect),
                    "Inspect"
                }
                button { class: "retire-button",
                    "Retire"
                }
                button { class: "restore-button",
                    "Restore"
                }

            }
        }

        // maybe hashmap? map |sku, id|
        /*products.iter().map(|sku| rsx!{
            img {
                src: "./products/{sku}",
                onclick: move |_| input_sku(sku),
            }
        })*/
        /*for (sku, id) in hash_products {
           p{ "{sku}"},
            "{id}",
        }*/
    ))
}

fn main() {
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("SKURcode")
            .with_inner_size(dioxus_desktop::LogicalSize::new(830.0,1008.0))
            .with_resizable(false)
            .with_position(dioxus_desktop::tao::dpi::PhysicalPosition::new(100,0)),
    );
    dioxus_desktop::launch_cfg(App, config);
}
