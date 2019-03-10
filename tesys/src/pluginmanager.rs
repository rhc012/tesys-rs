use std::path::Path;

use crate::loggable;
use crate::loggable::Loggable;
use crate::PluginHost;
use crate::net::Router;

#[derive(Loggable)]
#[derive(Debug)] // rhc 20190118
pub struct PluginManager {
    hosts: Vec<PluginHost>,

    // The vector containing plugin search paths
    plugin_search_paths: Vec<&'static str>,

    // Let's get an instance of a router so that we can route to our plugins
    router: Router,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        let pl = PluginManager {
            hosts: Vec::new(),
            plugin_search_paths: vec!["./"],
            router: Router::new(),
        };

        pl // Return our new plugin manager instance
    }

    pub fn load(&mut self, id: &'static str) -> Result<(), String> {
        match self.resolve_plugin_lib(id) {
            Ok(path) => { //     rhc 20190124
                println!("rhc path.to_string(){}", path.to_string());
                match PluginHost::load(path.to_string()) {
                    Ok(mut pgh) => {
                        pgh.test();
                        // More testing code.
                        pgh.start();
                        self.hosts.push(pgh);
                        Ok(())
                    }
                    Err(_e) => Err(format!("rhc Unable to load plugin '{}'", id).to_string()),
                }
            },
            Err(_e) => Err(format!("Unable to resolve library for plugin '{}'", id).to_string()),
        }
    }

    fn resolve_plugin_lib(&self, lib: &'static str) -> Result<String, ()> {
        let plugin_search_paths = self.plugin_search_paths.clone();
        let mut base = String::new();
        let mut dir = String::new();

        for path in plugin_search_paths {
            println!("rhc: path {}", path);
            if &lib[0..2] == "./" {
                // This is a relative path, we should resolve it within the current plugin dir
                let libtr = &lib[1..];

                if Path::new(libtr).exists() {
                    return Ok(libtr.to_owned());
                } else {
                    tesys_err!(Self, "Unable to find plugin '{}' at relative path. ", lib);
                }
            } else if &lib[0..1] == "/" {
                // If it's absolute, we don't need to resolve, so just check if the file exists
                if Path::new(lib).exists() {
                    return Ok(lib.to_owned());
                } else {
                    tesys_err!(Self, "Unable to find plugin '{}' at absolute path. ", lib);
                }
            } else {
                // Assume it's an relative path, so prepend the library dir
                base = lib.to_string();
                dir = path.to_owned();
            }

            //if Path::new( &format!("{}/{}", path.to_owned(), base) ).is_dir() { plugin_search_paths.push( &format!("{}/{}", path.to_owned(), base)) }

            // Append a forward slash if there isn't one
            if dir.chars().last() != Some('/') {
                dir.push_str("/");
            }

            // First let's check for a .so file on UNIX type systems
            if Path::new(&format!("{}lib{}.so", dir, base)).exists() {
                return Ok(format!("{}lib{}.so", dir, base).to_owned());
            }

            // Now we check for another favourite, dylib.
            if Path::new(&format!("{}lib{}.dylib", dir, base)).exists() {
                return Ok(format!("{}lib{}.dylib", dir, base).to_owned());
            }

            // Now let's check for a framework, and correct the path accordingly to actually get the lib
            if Path::new(&format!("{}lib{}.framework", dir, base)).is_dir() {
                return Ok(format!("{}lib{}.framework/{}", dir, base, base).to_owned());
            }

            if Path::new(&format!("{}lib{}.dll", dir, base)).is_dir() {
                return Ok(format!("{}lib{}.dll", dir, base).to_owned());
            }
        }

        tesys_err!("Unable to find resolve plugin '{}'.", lib);
        Err(())
    }

    pub fn add_plugin_search_directory(&mut self, path: &'static str) {
        let tmp = path.clone();
        let fin: &'static str;
        let dir = Path::new(&tmp);

        if dir.exists() && dir.is_dir() {
            if &tmp[0..1] == "/" {
                fin = &tmp[1..];
            } else {
                fin = &tmp;
            }
            self.plugin_search_paths.push(fin);
        }
    }

    pub fn set_plugin_search_directories(&mut self, paths: Vec<&'static str>) {
        self.plugin_search_paths = paths;
    }
}
