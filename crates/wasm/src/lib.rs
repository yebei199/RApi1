use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, window,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 获取 window 和 document
    let window = window().unwrap();
    let document = window.document().unwrap();

    // 获取 canvas 元素
    let canvas = document
        .get_element_by_id("my-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    // 获取 2D 渲染上下文
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // 绘制一个红色矩形
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(30.0, 30.0, 100.0, 100.0);

    Ok(())
}
