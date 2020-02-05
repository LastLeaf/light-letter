#[macro_use] extern crate log;

use std::collections::HashMap;
#[allow(unused_imports)] use wasm_bindgen::prelude::*;
use maomi::prelude::*;
use maomi::Context;
use maomi::backend::{Dom, Empty};
use maomi::node::ComponentNodeRc;

mod not_found;
mod backstage;

thread_local! {
    static CONTEXT: std::cell::RefCell<Option<Context<Dom>>> = std::cell::RefCell::new(None);
}

#[cfg(target_os = "wasm32")]
fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        console_log::init_with_level(log::Level::Debug).unwrap();
    });
}

#[cfg(target_os = "wasm32")]
fn init_prerendered<C: PrerenderableComponent<Dom>>(prerendered_data: &str) {
    init_logger();
    let prerendered_data = base64::decode(&prerendered_data).unwrap();
    let context = maomi::Context::new_prerendered::<C>(Dom::new_prerendering("maomi-prerendered"), &prerendered_data);
    CONTEXT.with(|c| {
        *c.borrow_mut() = Some(context);
    })
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub path: String,
    pub query: String,
}

#[derive(Clone)]
pub struct PrerenderResult {
    pub node_rc: ComponentNodeRc<Empty>,
    pub prerendered_data: Vec<u8>,
    pub title: String,
    pub is_ok: bool,
}

fn prerender<C: PrerenderableComponent<Empty>>(args: HashMap<&'static str, &str>, query: &str, is_ok: bool) -> PrerenderResult {
    let (context, prerendered_data) = maomi::context::Context::prerender::<C>(Empty::new());
    let comp = context.root_component::<C>().unwrap().into_node();
    PrerenderResult {
        node_rc: comp,
        prerendered_data,
        title: String::new(), // TODO
        is_ok,
    }
}

// The entrance in non-ssr mode
#[cfg(target_os = "wasm32")]
#[wasm_bindgen]
pub fn client_render_maomi_component(path: &str, query: &str) {
    unimplemented!()
}

macro_rules! routes {
    ($($route:expr => $comp:ty;)*) => {
        // Do ssr
        pub fn prerender_maomi_component(req_info: ReqInfo) -> PrerenderResult {
            let (target, args) = route_path(req_info.path.as_str());
            debug!("Prerendering path {:?} query {:?}, matched route {:?}", &req_info.path, &req_info.query, target);
            match target {
                $( $route => prerender::<$comp>(args, &req_info.query, true), )*
                _ => prerender::<not_found::NotFound>(HashMap::new(), "", false)
            }
        }

        // Load ssr result
        #[cfg(target_os = "wasm32")]
        #[wasm_bindgen]
        pub fn load_maomi_component(path: &str, data: &str) {
            let (target, _args) = route_path(req_info.path.as_str());
            debug!("Loading prerendered {:?}", target);
            match target {
                $( $route => init_prerendered::<$comp>(data), )*
                _ => init_prerendered::<not_found::NotFound>(data)
            }
        }

        thread_local! {
            static ROUTES: RouteSlice = init_routes();
        }

        struct RouteSlice {
            target: &'static str,
            static_routes: HashMap<&'static str, RouteSlice>,
            dynamic_route: Option<Box<(&'static str, RouteSlice)>>,
        }

        impl RouteSlice {
            fn new() -> Self {
                RouteSlice {
                    target: "",
                    static_routes: HashMap::new(),
                    dynamic_route: None,
                }
            }
        }

        // Prepare routing
        fn init_routes() -> RouteSlice {
            let mut root_route_slice = RouteSlice::new();
            let mut add_route = |s: &'static str| {
                debug!("Registering route {:?}", s);
                let slices = s.split('/');
                let mut cur = &mut root_route_slice;
                for slice in slices {
                    if slice.len() == 0 {
                        continue;
                    }
                    if &slice[0..1] == "{" && &slice[(slice.len() - 1)..] == "}" {
                        let key = &slice[1..(slice.len() - 1)];
                        let next = RouteSlice::new();
                        cur.dynamic_route = Some(Box::new((key, next)));
                        cur = &mut cur.dynamic_route.as_mut().unwrap().1;
                    } else {
                        if !cur.static_routes.contains_key(slice) {
                            cur.static_routes.insert(slice, RouteSlice::new());
                        }
                        cur = cur.static_routes.get_mut(slice).unwrap();
                    }
                }
                cur.target = s;
            };
            $( add_route($route); )*
            root_route_slice
        }

        // Find route
        fn route_path(path: &str) -> (&'static str, HashMap<&'static str, &str>) {
            let slices = path.split('/');
            let mut args = HashMap::new();
            let target = ROUTES.with(|root_route_slice| {
                let mut cur = root_route_slice;
                for slice in slices {
                    if slice.len() == 0 {
                        continue;
                    }
                    if let Some(next) = cur.static_routes.get(slice) {
                        cur = next;
                    } else if let Some(v) = cur.dynamic_route.as_ref() {
                        let (key, next) = &**v;
                        cur = &next;
                        args.insert(*key, slice);
                    } else {
                        args.clear();
                        return "";
                    }
                }
                cur.target
            });
            (target, args)
        }
    };
}

macro_rules! stylesheets {
    ($($comp:ty;)*) => {
        lazy_static::lazy_static! {
            static ref CSS_STR: &'static str = {
                let s = vec![
                    $(<$comp as ComponentTemplate<Empty>>::template_skin()),*
                ];
                Box::leak(s.join("").into_boxed_str())
            };
        }
        pub fn get_css_str() -> &'static str {
            &CSS_STR
        }
    }
}

routes! {
    "/hello_world" => backstage::login::HelloWorld;
}

stylesheets! {
    backstage::login::HelloWorld;
}
