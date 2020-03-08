use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use maomi::prelude::*;
use maomi::Context;
use maomi::backend::{Dom, Empty};
use maomi::node::ComponentNodeRc;

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub path: String,
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct ReqArgs<T: Default> {
    pub(crate) path_args: HashMap<&'static str, String>,
    pub(crate) query: T,
}

#[derive(Clone)]
pub struct PrerenderResult {
    pub node_rc: ComponentNodeRc<Empty>,
    pub prerendered_data: Vec<u8>,
    pub title: String,
    pub is_ok: bool,
}

pub struct PageMetaData {
    pub title: String,
}

thread_local! {
    pub(crate) static CONTEXT: std::cell::RefCell<Option<Context<Dom>>> = std::cell::RefCell::new(None);
}

pub(crate) fn prerender<C: PrerenderableComponent<Empty, MetaData = PageMetaData>>(req_args: <C as PrerenderableComponent<Empty>>::Args, is_ok: bool) -> PrerenderResult {
    let (context, prerendered_data, meta_data) = maomi::context::Context::prerender::<C>(Empty::new(), req_args);
    let comp = context.root_component::<C>().unwrap();
    let comp_node = comp.into_node();
    PrerenderResult {
        node_rc: comp_node,
        prerendered_data,
        title: meta_data.title,
        is_ok,
    }
}

pub(crate) fn init_prerendered<C: PrerenderableComponent<Dom>>(prerendered_data: &str) {
    let prerendered_data = base64::decode(&prerendered_data).unwrap();
    let context = maomi::Context::new_prerendered::<C>(Dom::new_prerendering("maomi-prerendered"), &prerendered_data);
    CONTEXT.with(|c| {
        *c.borrow_mut() = Some(context);
    })
}

pub(crate) fn route_to(path: &str, query: &str) {
    let path_and_query = path.to_string() + if query.len() > 0 { "?" } else { "" } + query;
    let history = web_sys::window().unwrap().history().unwrap();
    history.push_state_with_url(&wasm_bindgen::JsValue::UNDEFINED, "", Some(&path_and_query)).unwrap();
    wasm_bindgen_futures::spawn_local(crate::routes::client_render_maomi_component(path.to_string(), query.to_string()));
}

pub(crate) fn history_state_init() {
    thread_local! {
        static ONPOPSTATE: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let location = web_sys::window().unwrap().location();
            let location_search = location.search();
            let location_search = location_search.unwrap();
            let location_search = if location_search.as_str().len() > 0 {
                &location_search[1..]
            } else {
                ""
            };
            route_to(location.pathname().unwrap().as_str(), location_search);
        }) as Box<dyn FnMut()>);
    };
    ONPOPSTATE.with(|cb| {
        web_sys::window().unwrap().set_onpopstate(Some(cb.as_ref().unchecked_ref()));
    });
}

macro_rules! routes {
    ($($route:expr => $comp:ty;)*) => {

        // The entrance in non-ssr mode (used when jump pages)
        pub(crate) async fn client_render_maomi_component(path: String, query: String) {
            debug!("Loading page {:?} query {:?}", path, query);
            let (target, path_args) = route_path(&path);
            let root_component_node = CONTEXT.with(|c| {
                let mut context = c.borrow_mut();
                let context = context.as_mut().unwrap();
                match target {
                    $( $route => {
                        let root_component = context.new_root_component::<$comp>();
                        context.set_root_component(root_component);
                        context.root_component_node().unwrap()
                    }, )*
                    _ => {
                        let root_component = context.new_root_component::<not_found::NotFound>();
                        context.set_root_component(root_component);
                        context.root_component_node().unwrap()
                    }
                }
            });
            match target {
                $( $route => {
                    let req_args = ReqArgs { path_args, query: serde_urlencoded::from_str(&query).unwrap_or_default() };
                    let root_component = root_component_node.with_type::<$comp>();
                    let (data, meta) = <$comp as PrerenderableComponent<_>>::get_prerendered_data(&root_component.borrow(), req_args).await;
                    web_sys::window().unwrap().document().unwrap().set_title(&meta.title);
                    <$comp as PrerenderableComponent<_>>::apply_prerendered_data(&mut root_component.borrow_mut(), &data);
                }, )*
                _ => {
                    let req_args = ReqArgs { path_args, query: serde_urlencoded::from_str(&query).unwrap_or_default() };
                    let root_component = root_component_node.with_type::<not_found::NotFound>();
                    let (data, meta) = <not_found::NotFound as PrerenderableComponent<Dom>>::get_prerendered_data(&root_component.borrow(), req_args).await;
                    web_sys::window().unwrap().document().unwrap().set_title(&meta.title);
                    <not_found::NotFound as PrerenderableComponent<Dom>>::apply_prerendered_data(&mut root_component.borrow_mut(), &data);
                }
            };
        }

        // Do ssr
        pub fn prerender_maomi_component(req_info: ReqInfo) -> PrerenderResult {
            let (target, path_args) = route_path(req_info.path.as_str());
            let query = &req_info.query;
            debug!("Prerendering path {:?} query {:?}, matched route {:?}", &req_info.path, &req_info.query, target);
            match target {
                $( $route => {
                    let req_args: <$comp as PrerenderableComponent<Empty>>::Args = ReqArgs { path_args, query: serde_urlencoded::from_str(query).unwrap_or_default() };
                    prerender::<$comp>(req_args, true)
                }, )*
                _ => {
                    let req_args: <not_found::NotFound as PrerenderableComponent<Empty>>::Args = ReqArgs { path_args, query: serde_urlencoded::from_str(query).unwrap_or_default() };
                    prerender::<not_found::NotFound>(req_args, false)
                }
            }
        }

        // Load ssr result
        #[wasm_bindgen]
        pub fn load_maomi_component(path: &str, data: &str) {
            let (target, _args) = route_path(path);
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
        fn route_path(path: &str) -> (&'static str, HashMap<&'static str, String>) {
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
                        args.insert(*key, slice.to_owned());
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
