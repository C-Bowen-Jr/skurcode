#![allow(non_snake_case)]
#![allow(unused_variables)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use std::fs;
use std::io;
use std::path::Path;
use qrcodegen::{QrCode, QrCodeEcc};

#[derive(PartialEq, Copy, Clone)]
enum Action {
    None,
    Retire,
    Restore,
    Inspect,
}
impl Action {
    fn get_string(self: Action) -> String {
        match self {
            Action::None => return "".to_string(),
            Action::Retire => return "retire:".to_string(),
            Action::Restore => return "restore:".to_string(),
            Action::Inspect => return "inspect".to_string(),
        }
    }
}

fn QrCodeImg(cx: Scope) -> Element {
    cx.render(rsx! {
        img {
            src: "./qr.svg",
        }
    })
}

#[derive(PartialEq, Props)]
struct QrParts {
    #[props(into)]
    action: String,
    sku: String,
    quantity: i32,
}

impl QrParts {
    fn Qr_String(&self) -> String {
        if self.action == "".to_string() {
            return format!("{}{}", self.sku, self.quantity.to_string());
        }
        else {
            return format!("{}{}", self.action, self.sku);
        }
    }
    fn Qr_File(&self) -> String {
        format!("./{}.svg", self.Qr_String())
    }
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
// temp function
fn print_qr(qr: &QrCode) {
    let border: i32 = 4;
    for y in -border .. qr.size() + border {
        for x in -border .. qr.size() + border {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}
fn to_svg_string(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
    result += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0 .. qr.size() {
        for x in 0 .. qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    result
}

fn save_svg_from_string(filename: String, svg: String) {
    //println!("{}", filename);
    fs::write(filename, svg).expect("filename failure");
    //Ok(())
}

fn ProductGrid(cx: Scope) -> Element {
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
    Ok(file_names)
}

fn App(cx: Scope) -> Element {
    //let products = get_image_file_names().unwrap();
    use_shared_state_provider(cx, || Selection{sku:"none".to_string()});
    let selected_sku = use_shared_state::<Selection>(cx).unwrap();
    let mut quantity = use_state(cx, || 0);
    let action = use_state(cx, || Action::None); // no mut per lint?
    let mut action_text = "".to_string();
    let mut qr = QrCode::encode_text("test", QrCodeEcc::Medium).unwrap();
    let show_qr = use_state(cx, || false);
    let each_qr_part = use_ref(cx, || QrParts{
                                             action: "".to_string(),
                                             sku: "".to_string(),
                                             quantity: 0,
                                               });

    cx.render(rsx!(
        style { include_str!("../assets/skurcode.css") }
        div { class: "view",
            div { class: "product-grid",
                // Doesn't work here, seems to be because of the custom element
                //onclick: move |_| {quantity.set(0); action.set(Action::None)},
                ProductGrid{},
            }
            div { class: "action-buffer"},
            div { class: "action-tray",
                div{ class: "action-text",
                    onclick: move |_| {
                        let qr_text: String; // lint, mut not needed?
                        if action.get() == &Action::None {
                            qr_text = format!("{}*{}", &selected_sku.read().sku,quantity);
                            each_qr_part.write().sku = selected_sku.read().sku.to_string();
                            each_qr_part.write().quantity = *quantity.get();
                        }
                        else {
                            qr_text = format!("{}{}",action.get().get_string(),&selected_sku.read().sku);
                        }
                        qr = QrCode::encode_text(&qr_text, QrCodeEcc::Medium).unwrap();
                        save_svg_from_string(each_qr_part.read().Qr_File(),to_svg_string(&qr,1));
                        show_qr.set(true)
                    },
                    if action.get() == &Action::None {
                        rsx!(input { // without rsx! says invalid
                            value: "{selected_sku.read().sku}*{quantity}"
                        })
                    }
                    else {
                        action_text = action.get().get_string();
                        rsx!(input {
                            value: "{action_text}{selected_sku.read().sku}"
                        })
                    }
                },
                button { class: "sell-button",
                    onclick: move |_| {quantity -= 1; action.set(Action::None)},
                    "Sell"
                },
                button { class: "stock-button",
                    onclick: move |_| {quantity += 1; action.set(Action::None)},
                    "Stock"
                }
                button { class: "inspect-button",
                    onclick: move |_| action.set(Action::Inspect),
                    "Inspect"
                }
                button { class: "retire-button",
                    onclick: move |_| action.set(Action::Retire),
                    "Retire"
                }
                button { class: "restore-button",
                    onclick: move |_| action.set(Action::Restore),
                    "Restore"
                }

            },
            if *show_qr.get() {
                rsx!( div { // again, breaks if not in rsx!()
                    id: "overlay",
                    onclick: move |_| {
                        fs::remove_file(each_qr_part.read().Qr_File()).unwrap();
                        show_qr.set(false);
                        quantity.set(0);
                        action.set(Action::None)
                    },
                    div {
                        class: "overlay-qr",
                        img {
                            src: "{each_qr_part.read().Qr_File()}",
                        }
                    }
                })
            }
        }
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
