use serde::Deserialize;
use reqwest::blocking::Client;
use semver::Version;
use std::{ fs::File, env, process::Command };
use std::os::windows::process::CommandExt;

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn fetch_latest_release() -> Result<Release, Box<dyn std::error::Error>> {
    let client = Client::new();

    let release = client
        .get("https://api.github.com/repos/thehaidarbahzi/restcreater/releases/latest")
        .header("User-Agent", "restcreater")
        .send()?
        .error_for_status()?
        .json::<Release>()?;

    Ok(release)
}

pub fn check_version(latest: &str) -> bool {
    let current = Version::parse(CURRENT_VERSION).unwrap();
    let latest = Version::parse(latest).unwrap();
    latest > current
}

fn find_windows_asset(release: &Release) -> Option<&Asset> {
    release.assets.iter().find(|a| a.name == "restcreater.exe")
}

fn download(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = Client::new().get(url).send()?.error_for_status()?;
    let mut file = File::create(output)?;
    std::io::copy(&mut resp, &mut file)?;
    Ok(())
}

fn replace_and_restart(new_exe: &str) {
    let current = env::current_exe().unwrap();
    let updater = current.with_file_name("update.bat");

    let script = format!(
        r#"
@echo off
timeout /t 2 /nobreak > nul
move /Y "{new}" "{cur}" > nul 2>&1
start "" "{cur}" > nul 2>&1
if exist "%~f0" del "%~f0" > nul 2>&1
"#,
        new = new_exe,
        cur = current.display()
    );

    std::fs::write(&updater, script).unwrap();

    Command::new("cmd")
        .args(["/C", updater.to_str().unwrap()])
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
}

pub fn update_to_latest(release: &Release) -> Result<bool, Box<dyn std::error::Error>> {
    let asset = find_windows_asset(release).ok_or("Windows binary not found in release")?;

    let temp_path = "restcreater_new.exe";

    download(&asset.browser_download_url, temp_path)?;

    replace_and_restart(temp_path);

    Ok(true)
}
