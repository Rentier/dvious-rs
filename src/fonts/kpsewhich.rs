use std::process::Command;

use errors::DviousError;

pub fn kpsewhich<S1, S2>(name: S1, file_format: S2) -> Result<String, DviousError>
where
    S1: Into<String>,
    S2: Into<String>,
{
    let output = Command::new("kpsewhich")
        .args(&[format!("--format={}", file_format.into()), name.into()])
        .output()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout)
            .into_owned()
            .trim()
            .to_string();
        Ok(s)
    } else {
        Err(DviousError::KpsewhichError("Kpsewhich finished with nonzero status code!".to_string()))
    }
}

pub fn get_path_to_pk<S: Into<String>>(name: S) -> Result<String, DviousError> {
    kpsewhich(name, "pk")
}
