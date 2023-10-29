use dioxus::prelude::*;

const WIDTH: i64 = 400;
const HEIGHT: i64 = 20;
#[inline_props]
pub fn render_page(
    cx: Scope,
    page_no: i64,
    amount: i64,
    size: i64,
    sub_pages: Vec<i32>,
) -> Element {
    let items = sub_pages.iter().map(|page| {
        let page = *page as i64;
        let x = page * 4;
        rsx! { line { x1: x, y1: 0, x2: x, y2: HEIGHT, style: "stroke:blue;stroke-width:2" } }
    });
    render! {
        div { style: "background-color:var(--bg-coloer);padding:5px;",
            div { class: "info-line-xs", "Page:{page_no}; Amount: {amount}; Size: {size}" }
            svg { width: WIDTH, height: HEIGHT, style: "fill:white;stroke-width:;stroke:black",
                rect {
                    width: WIDTH,
                    height: HEIGHT,
                    rx: 5,
                    ry: 5,
                    style: "fill:white;stroke-width:;stroke:black"
                }
                items
            }
        }
    }
}
