use alloc::{format, string::{String, ToString}};
use js_sys::Uint8Array;
use sudoku_core::Cell;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlAnchorElement, HtmlElement, Url, window};
use yew::prelude::*;

pub fn now() -> f64 {
    let window = unsafe { window().unwrap_unchecked() };
    let performance = unsafe { window.performance().unwrap_unchecked() };
    performance.now()
}

pub fn cell_from_event(event: &MouseEvent) -> Option<Cell> {
    let target = event.current_target()?;
    let element: Element = target.unchecked_into();
    let attribute = element.get_attribute("data-index")?;
    let index: usize = attribute.parse().ok()?;
    Cell::from_index(index)
}

pub fn element_from_event(event: &MouseEvent) -> HtmlElement {
    event.target().unwrap().unchecked_into()
}