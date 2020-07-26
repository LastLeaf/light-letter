#[macro_export]
macro_rules! routes {
    ($default_comp:ty, $($route:expr => $comp:ty;)*) => {
        mod routes {
            use std::collections::HashMap;
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            use maomi::prelude::*;
            use maomi::Context;
            use maomi::backend::{Dom, Empty};

            use super::*;
            
            thread_local! {
                pub static __CONTEXT: std::cell::RefCell<Option<Context<Dom>>> = std::cell::RefCell::new(None);
            }
            
            /// Routing method
            #[allow(dead_code)]
            pub fn route_to(path: &str, query: &str) {
                let path_and_query = path.to_string() + if query.len() > 0 { "?" } else { "" } + query;
                let history = web_sys::window().unwrap().history().unwrap();
                history.push_state_with_url(&wasm_bindgen::JsValue::UNDEFINED, "", Some(&path_and_query)).unwrap();
                $crate::run_client_async(__client_render_maomi_component(path.to_string(), query.to_string()));
            }

            fn __prerender<C: PrerenderableComponent<Empty, MetaData = PageMetaData>>(req_args: <C as PrerenderableComponent<Empty>>::Args, is_ok: bool) -> PrerenderResult {
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
            
            fn __init_prerendered<C: PrerenderableComponent<Dom>>(prerendered_data: &str) {
                let prerendered_data = $crate::base64::decode(&prerendered_data).unwrap();
                let context = maomi::Context::new_prerendered::<C>(Dom::new_prerendering("maomi-prerendered"), &prerendered_data);
                __CONTEXT.with(|c| {
                    *c.borrow_mut() = Some(context);
                })
            }
            
            fn __history_state_init() {
                thread_local! {
                    static ONPOPSTATE: () = {
                        let cb = Closure::wrap(Box::new(move || {
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
                        web_sys::window().unwrap().set_onpopstate(Some(cb.as_ref().unchecked_ref()));
                    };
                }
                ONPOPSTATE.with(|_| { });
            }
            
            // The entrance in non-ssr mode (used when jump pages)
            async fn __client_render_maomi_component(path: String, query: String) {
                __history_state_init();
                debug!("Loading page {:?} query {:?}", path, query);
                let (target, path_args) = route_path(&path);
                let root_component_node = __CONTEXT.with(|c| {
                    let mut context = c.borrow_mut();
                    let context = context.as_mut().unwrap();
                    match target {
                        $( $route => {
                            let root_component = context.new_root_component::<$comp>();
                            context.set_root_component(root_component);
                            context.root_component_node().unwrap()
                        }, )*
                        _ => {
                            let root_component = context.new_root_component::<$default_comp>();
                            context.set_root_component(root_component);
                            context.root_component_node().unwrap()
                        }
                    }
                });
                let request_channel = $crate::client_request_channel();
                match target {
                    $( $route => {
                        let req_args = ReqArgs { path_args, query: serde_urlencoded::from_str(&query).unwrap_or_default(), request_channel };
                        let root_component = root_component_node.with_type::<$comp>();
                        let (data, meta) = <$comp as PrerenderableComponent<_>>::get_prerendered_data(&root_component.borrow(), req_args).await;
                        web_sys::window().unwrap().document().unwrap().set_title(&meta.title);
                        <$comp as PrerenderableComponent<_>>::apply_prerendered_data(&mut root_component.borrow_mut(), &data);
                    }, )*
                    _ => {
                        let req_args = ReqArgs { path_args, query: serde_urlencoded::from_str(&query).unwrap_or_default(), request_channel };
                        let root_component = root_component_node.with_type::<$default_comp>();
                        let (data, meta) = <$default_comp as PrerenderableComponent<Dom>>::get_prerendered_data(&root_component.borrow(), req_args).await;
                        web_sys::window().unwrap().document().unwrap().set_title(&meta.title);
                        <$default_comp as PrerenderableComponent<Dom>>::apply_prerendered_data(&mut root_component.borrow_mut(), &data);
                    }
                };
            }

            /// SSR loading entrance (should export in `Theme` trait)
            pub fn prerender_maomi_component(req_info: ReqInfo, request_channel: RequestChannel) -> PrerenderResult {
                let (target, path_args) = route_path(req_info.path.as_str());
                let query = &req_info.query;
                debug!("Prerendering path {:?} query {:?}, matched route {:?}", &req_info.path, &req_info.query, target);
                match target {
                    $( $route => {
                        let req_args: <$comp as PrerenderableComponent<Empty>>::Args = ReqArgs { path_args, query: serde_urlencoded::from_str(query).unwrap_or_default(), request_channel };
                        __prerender::<$comp>(req_args, true)
                    }, )*
                    _ => {
                        let req_args: <$default_comp as PrerenderableComponent<Empty>>::Args = ReqArgs { path_args, query: serde_urlencoded::from_str(query).unwrap_or_default(), request_channel };
                        __prerender::<$default_comp>(req_args, false)
                    }
                }
            }

            /// Non-SSR loading entrance (should export in `Theme` trait)
            #[allow(dead_code)]
            pub fn load_maomi_component(path: &str, data: &str) {
                __history_state_init();
                let (target, _args) = route_path(path);
                debug!("Loading prerendered {:?} (path {:?})", target, path);
                match target {
                    $( $route => __init_prerendered::<$comp>(data), )*
                    _ => __init_prerendered::<$default_comp>(data)
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
        }
    };
}

#[macro_export]
macro_rules! stylesheets {
    ($($comp:ty;)*) => {
        mod stylesheets {
            use maomi::prelude::*;
            use maomi::backend::Empty;

            use super::*;

            $crate::lazy_static::lazy_static! {
                static ref CSS_STR: &'static str = {
                    let s = vec![
                        $(<$comp as ComponentTemplate<Empty>>::template_skin()),*
                    ];
                    Box::leak(s.join("").into_boxed_str())
                };
            }

            /// Get the preprocessed CSS string (should export in `Theme` trait)
            pub fn get_css_str() -> &'static str {
                &CSS_STR
            }
        }
    }
}
