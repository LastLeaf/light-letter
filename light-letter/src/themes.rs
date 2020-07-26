use std::rc::Rc;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use libloading::{Library, Symbol};

use light_letter_web::Theme;

type LoadTheme = fn() -> Rc<dyn Theme>;

lazy_static! {
    static ref LOADED_LIBS: Arc<Mutex<HashMap<String, (&'static Library, Symbol<'static, LoadTheme>)>>> = Arc::new(Mutex::new(HashMap::new()));
}

thread_local! {
    static THEME: HashMap<String, Rc<dyn Theme>> = {
        let mut map = HashMap::new();
        for (name, (_, theme)) in LOADED_LIBS.lock().unwrap().iter() {
            map.insert(name.clone(), theme());
        }
        map
    };
}

pub(crate) fn init(config: &light_letter_rpc::SitesConfig) {
    load("backstage", &lib_subpath("light_letter_backstage", config.resource.backstage.as_str()));
    for (name, path) in config.resource.themes.iter() {
        let name = name.replace('-', "_");
        load(&name, &lib_subpath(&name, path))
    }
}

fn lib_subpath(name: &str, p: &str) -> PathBuf {
    #[cfg(target_os="linux")]
    let r = PathBuf::from(p).join(&format!("pkg/lib{}.so", name));
    #[cfg(target_os="macos")]
    let r = PathBuf::from(p).join(&format!("pkg/lib{}.dylib", name));
    #[cfg(target_os="windows")]
    let r = PathBuf::from(p).join(&format!("pkg/{}.dll", name));
    r
}

fn load(name: &str, path: &Path) {
    let lib = Box::leak(Box::new(Library::new(path).expect(&format!("Failed loading theme library {:?}", path))));
    unsafe {
        let lt: Symbol<LoadTheme> = lib.get(b"load_theme\0").expect(&format!("Failed loading theme library {:?}", path));
        LOADED_LIBS.lock().unwrap().insert(name.into(), (lib, lt));
    }
}

pub(crate) fn get(name: &str) -> Option<Rc<dyn Theme>> {
    THEME.with(|x| x.get(name).cloned())
}
