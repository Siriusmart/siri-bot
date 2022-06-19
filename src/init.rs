use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn init() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./storage");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/config");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/data/");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/data/global");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/data/global/giveaways");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/data/global/giveaways/index.json");
    if !path.exists() {
        let mut file = File::create(path)?;
        file.write_all("{}".as_bytes())?;
        file.flush()?;
    }

    let path = Path::new("./storage/data/guilds");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let path = Path::new("./storage/data/users");
    if !path.exists() {
        fs::create_dir(path)?;
    }

    Ok(())
}
