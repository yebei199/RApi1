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
    let width = window.inner_width()?.as_f64().unwrap();
    let height = window.inner_height()?.as_f64().unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    // 获取 2D 渲染上下文
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(30.0, 30.0, 100.0, 100.0);

    // 模拟一些数据点
    let data = [10.0, 50.0, 30.0, 70.0, 40.0, 90.0];

    // 设置线条样式
    context.set_stroke_style(&JsValue::from_str("blue"));
    context.set_line_width(2.0);

    // 开始路径
    context.begin_path();

    // 第一个点
    context.move_to(20.0, 100.0 - data[0]);

    // 依次连线
    for (i, y) in data.iter().enumerate().skip(1) {
        let x = 20.0 + i as f64 * 40.0; // 横坐标间隔 40
        let y = 100.0 - y; // 简单翻转一下坐标
        context.line_to(x, y);
    }

    // 绘制路径
    context.stroke();

    Ok(())
}
