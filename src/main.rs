use std::process::{Command, Stdio};

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use serde::Deserialize;

/// Launch an application under the COSMIC desktop, or focus one of its
/// already-open windows. If several windows are open, each invocation
/// cycles to the next one. Built on top of the `cosmicmsg` tool
/// (https://github.com/varbhat/cosmicmsg), which must be installed and
/// in $PATH.
#[derive(Parser)]
#[command(name = "cosmic-launch", version, about)]
struct Cli {
    /// App name used to match windows (app_id or title, case-insensitive).
    app: String,

    /// Extra arguments passed to the command when a new instance is launched.
    args: Vec<String>,

    /// Override the command to execute [default: <app>]
    #[arg(short, long)]
    exec: Option<String>,

    /// Force launching a new instance, never focus existing windows.
    #[arg(short, long)]
    launch: bool,
}

#[derive(Deserialize, Debug)]
struct Toplevel {
    title: String,
    app_id: String,
    identifier: String,
    state: Vec<String>,
}

fn cosmicmsg(args: &[&str]) -> Result<String> {
    let out = Command::new("cosmicmsg")
        .args(args)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow!("`cosmicmsg` not found in $PATH — install it from https://github.com/varbhat/cosmicmsg (e.g. `nix run github:varbhat/cosmicmsg`)")
            } else {
                anyhow!("failed to run cosmicmsg: {e}")
            }
        })?;
    if !out.status.success() {
        bail!(
            "cosmicmsg {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn is_activated(t: &Toplevel) -> bool {
    t.state.iter().any(|s| s == "activated")
}

fn matches_app(t: &Toplevel, app: &str) -> bool {
    let app = app.to_lowercase();
    let app_id = t.app_id.to_lowercase();
    // "zed" matches "dev.zed.Zed", "firefox" matches "firefox", etc.
    app_id == app
        || app_id.ends_with(&format!(".{app}"))
        || app_id.contains(&app)
        || t.title.to_lowercase().contains(&app)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = cli.app.clone();

    if !cli.launch {
        // List currently open windows via cosmicmsg (JSON).
        let json =
            cosmicmsg(&["--json", "get-toplevels"]).context("could not list COSMIC windows")?;
        let toplevels: Vec<Toplevel> = serde_json::from_str(&json)
            .context("could not parse `cosmicmsg get-toplevels` output")?;

        let mut matches: Vec<Toplevel> = toplevels
            .into_iter()
            .filter(|t| matches_app(t, &app))
            .collect();

        if !matches.is_empty() {
            // Cycle: activate the window following the currently
            // activated one (wrapping). If none is activated (e.g. all
            // minimized), take the first.
            let n = matches.len();
            let next = match matches.iter().position(|t| is_activated(t)) {
                Some(i) => (i + 1) % n,
                None => 0,
            };
            let target = matches.swap_remove(next);
            cosmicmsg(&["window", "activate", &target.identifier])?;
            eprintln!(
                "activated \"{}\" ({}) [{} / {}]",
                target.title,
                target.app_id,
                next + 1,
                n
            );
            return Ok(());
        }
    }

    // No window matched (or --launch): start a new instance, detached.
    let program = cli.exec.as_deref().unwrap_or(&app);
    let mut cmd = Command::new(program);
    cmd.args(&cli.args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // Detach: put the child in its own process group so it survives us.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    cmd.spawn()
        .with_context(|| format!("failed to launch `{program}"))?;
    eprintln!("launched `{program}`");
    Ok(())
}
