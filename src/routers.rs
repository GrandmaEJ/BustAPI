use crate::method::Method;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub pattern: String,
    pub regex: Regex,
    pub param_names: Vec<String>,
    pub handler: Py<PyAny>,
}

#[derive(Clone, Default)]
pub struct Router {
    pub routes: Arc<RwLock<Vec<Route>>>,
}

static PARAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}").unwrap());

fn path_to_regex(path: &str) -> (Regex, Vec<String>) {
    let mut param_names = Vec::new();
    let mut pattern = String::from("^");
    let mut last = 0usize;

    for cap in PARAM_RE.captures_iter(path) {
        let m = cap.get(0).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();
        param_names.push(name);
        pattern.push_str(&regex::escape(&path[last..m.start()]));
        pattern.push_str("(?P<");
        pattern.push_str(cap.get(1).unwrap().as_str());
        pattern.push_str(">[^/]+)");
        last = m.end();
    }
    pattern.push_str(&regex::escape(&path[last..]));
    pattern.push('$');
    (Regex::new(&pattern).unwrap(), param_names)
}

impl Router {
    pub fn new() -> Self { Self { routes: Arc::new(RwLock::new(Vec::new())) } }

    pub fn add(&self, method: Method, path: &str, handler: Py<PyAny>) {
        let (regex, params) = path_to_regex(path);
        let route = Route { method, pattern: path.to_string(), regex, param_names: params, handler };
        self.routes.write().unwrap().push(route);
    }

    pub fn find(&self, method: Method, path: &str) -> Option<(Route, HashMap<String, String>)> {
        let routes = self.routes.read().unwrap();
        for r in routes.iter() {
            if r.method == method {
                if let Some(caps) = r.regex.captures(path) {
                    let mut params = HashMap::new();
                    for name in &r.param_names {
                        if let Some(v) = caps.name(name) {
                            params.insert(name.clone(), v.as_str().to_string());
                        }
                    }
                    return Some((r.clone(), params));
                }
            }
        }
        None
    }
}