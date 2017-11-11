use std::io::Error;
use std::process::Command;

pub fn kpsewhich<S1, S2>(name: S1, file_format: S2) -> Result<String, Error>
where
    S1: Into<String>,
    S2: Into<String>,
{
    Command::new("kpsewhich")
        .args(&[format!("--format={}", file_format.into()), name.into()])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .into_owned()
                .trim()
                .to_string()
        })
}

pub fn get_path_to_pk<S: Into<String>>(name: S) -> Result<String, Error> {
    kpsewhich(name, "pk")
}
