use std::rc::Rc;

use alloc::{format, string::String};
use wasm_bindgen::JsValue;
use web_sys::{Document, HtmlElement, Navigator, Storage, Window};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};
use core::str::FromStr;
use log::Level;
use web_sys::window;
use thiserror::Error;

use crate::routes::{Route, switch};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppContext {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub local_storage: Storage,
    pub navigator: Navigator,
    pub app_name: Rc<str>,
    pub version: Rc<str>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum AppContextError {
    #[error("Window object not found")]
    NoWindow,
    
    #[error("localStorage is not available")]
    NoStorage,
    
    #[error("Failed to access localStorage: {0}")]
    StorageAccess(String),
    
    #[error("Document object not found")]
    NoDocument,
    
    #[error("Document body not found")]
    NoBody,
}

impl From<JsValue> for AppContextError {
    fn from(value: JsValue) -> Self {
        AppContextError::StorageAccess(format!("{:?}", value))
    }
}

impl AppContext {
    pub fn new() -> Result<Self, AppContextError> {
        let window = window()
            .ok_or(AppContextError::NoWindow)?;
        
        let local_storage = window
            .local_storage()
            .map_err(|e| AppContextError::StorageAccess(format!("{:?}", e)))?
            .ok_or(AppContextError::NoStorage)?;
        
        let document = window
            .document()
            .ok_or(AppContextError::NoDocument)?;
        
        let body = document
            .body()
            .ok_or(AppContextError::NoBody)?;
        
        let navigator = window.navigator();
        let app_name = env!("CARGO_PKG_NAME").into();
        let version = env!("CARGO_PKG_VERSION").into();

        Ok(Self {
            window,
            document,
            body,
            local_storage,
            navigator,
            app_name,
            version,
        })
    }

    pub fn try_get_log_level_from_local_storage(&self, key: &str) -> Option<Level> {
        
        let level_str = self.local_storage
            .get_item(key)
            .ok()
            .flatten()?;
        
        Level::from_str(&level_str).ok()
    }
}

#[function_component(App)]
pub fn app(props: &AppContext) -> Html {

    html! {
        <ContextProvider<AppContext> context={props.clone()}>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </ContextProvider<AppContext>>
    }
}