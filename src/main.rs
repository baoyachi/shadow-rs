use std::fs::File;
use std::fs;

mod channel;
mod err;
mod git;
mod shadow;
mod build;

use shadow::*;
use git::*;
use err::*;
use build::*;

use std::io::Write;
use std::path::Path;

fn main() {}

const SHADOW_RS: &str = "shadow.rs";

impl Shadow {
    pub fn new<P: Into<String>>(path: P) -> SdResult<Shadow> {
        let path = path.into();
        let build_path = format!("{}/{}", path, SHADOW_RS);
        let shadow_path = Path::new(&build_path);
        Ok(Shadow {
            f: File::create(shadow_path)?,
            project: Project::new(),
            sys_env: SystemEnv::new()?,
            // git: Git::new(Path::new(&path))?,
        })
    }


    fn write_const(&mut self, msg: ConstMessage) {
        let desc = format!("/// {}", msg.desc);
        let define = format!("pub const {} :{} = \"{}\";", msg.key.to_ascii_uppercase(), msg.t.to_string(), msg.val);
        writeln!(&self.f, "{}", desc);
        writeln!(&self.f, "{}\n", define);
    }
}

#[derive(Debug)]
pub struct Shadow {
    f: File,
    project: Project,
    sys_env: SystemEnv,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let mut shadow = Shadow::new("./").unwrap();
        let git = Git::new("./").unwrap();
        for g in git {
            shadow.write_const(g);
        }
    }
}