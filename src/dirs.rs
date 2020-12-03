use std::{error::Error, ffi::OsStr, fmt::Display, path::{Path, PathBuf}};

#[derive(Clone,Debug)]
#[non_exhaustive]
pub struct InstallDirs{
    pub prefix: PathBuf,
    pub exec_prefix: PathBuf,
    pub bin: PathBuf,
    pub sbin: PathBuf,
    pub lib: PathBuf,
    pub libexec: PathBuf,
    pub include: PathBuf,
    pub dataroot: PathBuf,
    pub data: PathBuf,
    pub man: PathBuf,
    pub doc: PathBuf,
    pub info: PathBuf,
    pub locale: PathBuf,
    pub localstate: PathBuf,
    pub runstate: PathBuf,
    pub sharedstate: PathBuf,
    pub sysconf: PathBuf
}

#[derive(Debug)]
pub struct CanonicalizationError{
    prefix: PathBuf
}

impl Display for CanonicalizationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to canonicalize Install Dirs ")?;
        f.write_fmt(format_args!("(prefix {} is not an absolute path)",self.prefix.to_str().unwrap_or("(<non-unicode>)")))
    }
}

impl Error for CanonicalizationError{
    
}

impl InstallDirs{
    /// 
    /// 
    pub fn defaults() -> Self{
        Self{
            prefix: if cfg!(windows){
                "C:\\Program Files\\"
            }else{
                "/usr/local"
            }.into(),
            exec_prefix: "".into(),
            bin: "bin".into(),
            sbin: "sbin".into(),
            lib: "lib".into(),
            libexec: "libexec".into(),
            include: "include".into(),
            dataroot: "share".into(),
            data: "".into(),
            man: "man".into(),
            doc: "doc".into(),
            info: "info".into(),
            locale: "locale".into(),
            localstate: "var".into(),
            runstate: "run".into(),
            sharedstate: "com".into(),
            sysconf: "var".into()
        }
    }

    pub fn with_project_name<S: AsRef<OsStr>>(name: &S) -> Self{
        Self{
            prefix: if cfg!(windows){
                let mut buf = PathBuf::new();
                buf.push("C:\\Program Files");
                buf.push(name.as_ref());
                buf
            }else{
                "/usr/local".into()
            },
            exec_prefix: "".into(),
            bin: "bin".into(),
            sbin: "sbin".into(),
            lib: "lib".into(),
            libexec: "libexec".into(),
            include: "include".into(),
            dataroot: "share".into(),
            data: "".into(),
            man: "man".into(),
            doc: {
                let mut path = PathBuf::new();
                path.push("doc");
                path.push(name.as_ref());
                path
            },
            info: "info".into(),
            locale: "locale".into(),
            localstate: "var".into(),
            runstate: "run".into(),
            sharedstate: "com".into(),
            sysconf: "var".into()
        }
    }

    pub fn with_exec_target<S: AsRef<OsStr>>(target: &S) -> Self{
        Self{
            prefix: if cfg!(windows){
                "C:\\Program Files\\"
            }else{
                "/usr/local"
            }.into(),
            exec_prefix: target.as_ref().into(),
            bin: "bin".into(),
            sbin: "sbin".into(),
            lib: "lib".into(),
            libexec: "libexec".into(),
            include: "include".into(),
            dataroot: "share".into(),
            data: "".into(),
            man: "man".into(),
            doc: "doc".into(),
            info: "info".into(),
            locale: "locale".into(),
            localstate: "var".into(),
            runstate: "run".into(),
            sharedstate: "com".into(),
            sysconf: "var".into()
        }
    }

    pub fn with_project_name_and_target<S: AsRef<OsStr>,T: AsRef<OsStr>>(name: &S,target: &T) -> Self{
        Self{
            prefix: if cfg!(windows){
                let mut buf = PathBuf::new();
                buf.push("C:\\Program Files");
                buf.push(name.as_ref());
                buf
            }else{
                "/usr/local".into()
            },
            exec_prefix: target.as_ref().into(),
            bin: "bin".into(),
            sbin: "sbin".into(),
            lib: "lib".into(),
            libexec: "libexec".into(),
            include: "include".into(),
            dataroot: "share".into(),
            data: "".into(),
            man: "man".into(),
            doc: {
                let mut path = PathBuf::new();
                path.push("doc");
                path.push(name.as_ref());
                path
            },
            info: "info".into(),
            locale: "locale".into(),
            localstate: "var".into(),
            runstate: "run".into(),
            sharedstate: "com".into(),
            sysconf: "var".into()
        }
    }

    pub fn canonicalize(mut self) -> Result<Self,CanonicalizationError>{
        if !self.prefix.has_root(){
            Err(CanonicalizationError{prefix: self.prefix})
        }else{

            if !self.exec_prefix.has_root(){
                self.exec_prefix = {
                    let mut path = PathBuf::new();
                    path.push(self.prefix.clone());
                    path.push(self.exec_prefix);
                    path
                }
            }

            let exec_prefix = if (&*self.exec_prefix)==Path::new("/"){
                let mut exec_prefix = PathBuf::new();
                exec_prefix.push("/usr");
                exec_prefix
            }else{
                self.exec_prefix.clone()
            };
            let data_prefix = if(&*self.prefix)==Path::new("/"){
                let mut exec_prefix = PathBuf::new();
                exec_prefix.push("/usr");
                exec_prefix
            }else{
                self.prefix.clone()
            };
            let state_prefix = if self.prefix.starts_with("/usr"){
                let mut prefix = PathBuf::new();
                prefix.push("/");
                prefix
            }else{
                self.prefix.clone()
            };
            if !self.bin.has_root(){
                self.bin = {
                    let mut path = exec_prefix.clone();
                    path.push(self.bin);
                    path
                };
            }

            if !self.sbin.has_root(){
                self.sbin = {
                    let mut path = exec_prefix.clone();
                    path.push(self.sbin);
                    path
                };
            }

            if !self.lib.has_root(){
                self.lib = {
                    let mut path = exec_prefix.clone();
                    path.push(self.lib);
                    path
                };
            }

            if !self.libexec.has_root(){
                self.libexec = {
                    let mut path = exec_prefix.clone();
                    path.push(self.libexec);
                    path
                };
            }

            if !self.include.has_root(){
                self.include = {
                    let mut path = exec_prefix.clone();
                    path.push(self.include);
                    path
                };
            }

            if !self.dataroot.has_root(){
                
                self.dataroot = {
                    let mut path = data_prefix.clone();
                    path.push(self.dataroot);
                    path
                };
            }

            if !self.data.has_root(){
                self.data = {
                    let mut path = self.dataroot.clone();
                    path.push(self.data);
                    path
                };
            }

            if !self.man.has_root(){
                self.man = {
                    let mut path = self.dataroot.clone();
                    path.push(self.man);
                    path
                };
            }

            if !self.info.has_root(){
                self.info = {
                    let mut path = self.dataroot.clone();
                    path.push(self.info);
                    path
                };
            }
            if !self.doc.has_root(){
                self.doc = {
                    let mut path = self.dataroot.clone();
                    path.push(self.doc);
                    path
                };
            }

            if !self.locale.has_root(){
                self.locale = {
                    let mut path = self.dataroot.clone();
                    path.push(self.locale);
                    path
                };
            }

            if !self.sharedstate.has_root(){
                self.sharedstate = {
                    let mut path = data_prefix.clone();
                    path.push(self.sharedstate);
                    path
                };
            }

            if !self.sysconf.has_root(){
                self.sysconf = if state_prefix.starts_with("/opt"){
                    let mut path = PathBuf::new();
                    path.push("/");
                    path.push(self.sysconf);
                    path.push(state_prefix.clone());
                    path
                }else{
                    let mut path = state_prefix.clone();
                    path.push( self.sysconf);
                    path
                }
            }

            if !self.localstate.has_root(){
                self.localstate = if state_prefix.starts_with("/opt"){
                    let mut path = PathBuf::new();
                    path.push("/");
                    path.push(self.localstate);
                    path.push(state_prefix.clone());
                    path
                }else{
                    let mut path = state_prefix.clone();
                    path.push( self.localstate);
                    path
                }
            }

            if !self.sharedstate.has_root(){
                self.sharedstate = {
                    let mut path = self.localstate.clone();
                    path.push(self.sharedstate);
                    path
                };
            }

            Ok(self)
        }
    }
}

/// 
/// Parses the compile-time environment into an instance of InstallDirs.
/// Note: This returns an owning structure and is not const.
/// Likely you will want to either store this, or it's canonical representation,
/// Inside a lazy_static!.
/// 
/// This uses the default installation configuration, see [`InstallDirs::defaults()`]
/// If a package name is specified as an expression, it uses the defaults for that package name, [`InstallDirs::with_project_name()`].
#[macro_export]
macro_rules! parse_env{
    () => {
        {
            let mut dirs = InstallDirs::defaults();
            if let Some(dir) = std::option_env!("prefix"){
                dirs.prefix = dir.into();
            }

            if let Some(dir) = std::option_env!("exec_prefix"){
                dirs.exec_prefix = dir.into();
            }

            if let Some(dir) = std::option_env!("bindir"){
                dirs.bindir = dir.into();
            }

            if let Some(dir) = std::option_env!("sbindir"){
                dirs.sbindir = dir.into();
            }
            if let Some(dir) = std::option_env!("libdir"){
                dirs.libdir = dir.into();
            }

            if let Some(dir) = std::option_env!("libexecdir"){
                dirs.libexec = dir.into();
            }

            if let Some(dir) = std::option_env!("includedir"){
                dirs.include = dir.into();
            }

            if let Some(dir) = std::option_env!("datarootdir"){
                dirs.dataroot = dir.into();
            }

            if let Some(dir) = std::option_env!("datadir"){
                dirs.data = dir.into();
            }

            if let Some(dir) = std::option_env!("mandir"){
                dirs.man = dir.into();
            }

            if let Some(dir) = std::option_env!("docdir"){
                dirs.doc = dir.into();
            }

            if let Some(dir) = std::option_env!("infodir"){
                dirs.info = dir.into();
            }

            if let Some(dir) = std::option_env!("localedir"){
                dirs.locale = dir.into();
            }

            if let Some(dir) = std::option_env!("sharedstatedir"){
                dirs.sharedstate = dir.into();
            }

            if let Some(dir) = std::option_env!("localstatedir"){
                dirs.localstate = dir.into();
            }

            if let Some(dir) = std::option_env!("runstatedir"){
                dirs.runstate = dir.into();
            }

            if let Some(dir) = std::option_env!("sysconfdir"){
                dirs.sysconf = dir.into();
            }

            dirs
        }
    };
    ($project:expr) => {
        {
            let mut dirs = InstallDirs::with_project_name($project);
            if let Some(dir) = std::option_env!("prefix"){
                dirs.prefix = dir.into();
            }

            if let Some(dir) = std::option_env!("exec_prefix"){
                dirs.exec_prefix = dir.into();
            }

            if let Some(dir) = std::option_env!("bindir"){
                dirs.bindir = dir.into();
            }

            if let Some(dir) = std::option_env!("sbindir"){
                dirs.sbindir = dir.into();
            }
            if let Some(dir) = std::option_env!("libdir"){
                dirs.libdir = dir.into();
            }

            if let Some(dir) = std::option_env!("libexecdir"){
                dirs.libexec = dir.into();
            }

            if let Some(dir) = std::option_env!("includedir"){
                dirs.include = dir.into();
            }

            if let Some(dir) = std::option_env!("datarootdir"){
                dirs.dataroot = dir.into();
            }

            if let Some(dir) = std::option_env!("datadir"){
                dirs.data = dir.into();
            }

            if let Some(dir) = std::option_env!("mandir"){
                dirs.man = dir.into();
            }

            if let Some(dir) = std::option_env!("docdir"){
                dirs.doc = dir.into();
            }

            if let Some(dir) = std::option_env!("infodir"){
                dirs.info = dir.into();
            }

            if let Some(dir) = std::option_env!("localedir"){
                dirs.locale = dir.into();
            }

            if let Some(dir) = std::option_env!("sharedstatedir"){
                dirs.sharedstate = dir.into();
            }

            if let Some(dir) = std::option_env!("localstatedir"){
                dirs.localstate = dir.into();
            }

            if let Some(dir) = std::option_env!("runstatedir"){
                dirs.runstate = dir.into();
            }

            if let Some(dir) = std::option_env!("sysconfdir"){
                dirs.sysconf = dir.into();
            }

            dirs
        }
    }
}


pub fn from_env() -> InstallDirs{
    let mut dirs = InstallDirs::defaults();

    if let Ok(dir) = std::env::var("prefix"){
        dirs.prefix = dir.into()
    }

    if let Ok(dir) = std::env::var("exec_prefix"){
        dirs.exec_prefix = dir.into()
    }

    if let Ok(dir) = std::env::var("bindir"){
        dirs.bin = dir.into()
    }

    if let Ok(dir) = std::env::var("libdir"){
        dirs.lib = dir.into()
    }

    if let Ok(dir) = std::env::var("sbindir"){
        dirs.sbin = dir.into()
    }
    if let Ok(dir) = std::env::var("libexecdir"){
        dirs.libexec = dir.into()
    }
    if let Ok(dir) = std::env::var("includedir"){
        dirs.include = dir.into()
    }

    if let Ok(dir) = std::env::var("datarootdir"){
        dirs.dataroot = dir.into()
    }

    if let Ok(dir) = std::env::var("datadir"){
        dirs.data = dir.into()
    }

    if let Ok(dir) = std::env::var("mandir"){
        dirs.man = dir.into()
    }

    if let Ok(dir) = std::env::var("docdir"){
        dirs.doc = dir.into()
    }

    if let Ok(dir) = std::env::var("infodir"){
        dirs.info = dir.into()
    }

    if let Ok(dir) = std::env::var("localedir"){
        dirs.locale = dir.into()
    }

    if let Ok(dir) = std::env::var("sharedstatedir"){
        dirs.sharedstate = dir.into()
    }

    if let Ok(dir) = std::env::var("localstatedir"){
        dirs.localstate = dir.into()
    }

    if let Ok(dir) = std::env::var("runstatedir"){
        dirs.runstate = dir.into()
    }

    if let Ok(dir) = std::env::var("sysconfdir"){
        dirs.sysconf = dir.into()
    }


    dirs

}
