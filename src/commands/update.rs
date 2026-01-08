use cliclack::{ confirm, intro, log, outro, set_theme, spinner };
use console::style;

use crate::helper::{
    theme::CustomTheme,
    updater::{ check_version, fetch_latest_release, update_to_latest },
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    set_theme(CustomTheme);

    intro(style(" Restcreater (Esc to Exit) ").bold().on_green())?;

    let progress = spinner();

    progress.start("Checking for new updates…");

    let version = match fetch_latest_release() {
        Ok(v) => v,
        Err(_) => {
            progress.stop("Failed to check for updates.");
            outro(
                format!(
                    "Problems? {}\n",
                    style("https://github.com/thehaidarbahzi/restcreater/issues")
                        .cyan()
                        .underlined()
                )
            )?;
            return Ok(());
        }
    };

    progress.stop("Update check completed.");

    let latest = version.tag_name.trim_start_matches('v');

    if check_version(latest) {
        let update = confirm(format!("There is an update available. Do you want to update now?"))
            .initial_value(false)
            .interact()?;

        if update {
            match update_to_latest(&version) {
                Ok(_) => {
                    log::step("Update success.")?;
                }
                Err(e) => {
                    log::error(format!("Update failed: {}", e))?;
                }
            }
        }
    } else {
        log::step("Restcreater is already up to date.")?;
    }

    outro(
        format!(
            "Problems? {}\n",
            style("https://github.com/thehaidarbahzi/restcreater/issues").cyan().underlined()
        )
    )?;

    Ok(())
}
