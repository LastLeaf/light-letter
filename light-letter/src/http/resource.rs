use std::sync::{Arc, Mutex};
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref RESOURCE_GLOBAL: Arc<Mutex<Option<Resource>>> = Arc::new(Mutex::new(None));
}

thread_local! {
    static RESOURCE: Resource = (*RESOURCE_GLOBAL.lock().unwrap()).as_ref().unwrap().clone();
}

#[derive(Clone)]
pub(crate) struct Resource {
    pub(crate) backstage_js: &'static [u8],
    pub(crate) backstage_wasm: &'static [u8],
    pub(crate) theme_js: HashMap<String, &'static [u8]>,
    pub(crate) theme_wasm: HashMap<String, &'static [u8]>,
}

impl Resource {
    fn new(backstage_path: &str, theme_paths: &HashMap<String, String>) -> Self {
        let backstage_path = PathBuf::from(backstage_path);
        let backstage_js = Self::load_file(&backstage_path.join("pkg/light_letter_backstage.js"));
        let backstage_wasm = Self::load_file(&backstage_path.join("pkg/light_letter_backstage_bg.wasm"));
        let theme: (HashMap<String, _>, HashMap<String, _>) = theme_paths.iter().map(|(name, theme_path)| {
            let name = name.replace('-', "_");
            let theme_path = PathBuf::from(theme_path);
            let js = Self::load_file(&theme_path.join(&format!("pkg/{}.js", name)));
            let wasm = Self::load_file(&theme_path.join(&format!("pkg/{}_bg.wasm", name)));
            ((name.clone(), js), (name.clone(), wasm))
        }).unzip();
        Self {
            backstage_js,
            backstage_wasm,
            theme_js: theme.0,
            theme_wasm: theme.1,
        }
    }

    fn load_file(p: &Path) -> &'static [u8] {
        let s = Box::new(fs::read(p).expect(&format!("Failed loading resource file {:?}", p)));
        Box::leak(s.into_boxed_slice())
    }
}

pub(crate) fn init(config: &light_letter_rpc::SitesConfig) {
    *RESOURCE_GLOBAL.lock().unwrap() = Some(Resource::new(config.resource.backstage.as_str(), &config.resource.themes));
}

pub(crate) fn get<R>(f: impl FnOnce(&Resource) -> R) -> R {
    RESOURCE.with(f)
}
