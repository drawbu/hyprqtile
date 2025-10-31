mod cli;

use anyhow::bail;
use hyprland::dispatch::WorkspaceIdentifier;
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::{
    data::{Monitor, Monitors, Workspaces},
    dispatch::{MonitorIdentifier, WorkspaceIdentifierWithSpecial},
};

/// Move target workspace in foreground of the active monitor.
///
/// See [`cli::Commands::Move`] for details.
#[tracing::instrument]
fn move_to_workspace(workspace_id: i32) -> anyhow::Result<()> {
    let workspaces = Workspaces::get()?;
    let target = match workspaces.iter().find(|w| w.id == workspace_id) {
        Some(w) => w,
        None => {
            tracing::debug!("Workspace is not active");
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_id))?;
            return Ok(());
        }
    };

    tracing::debug!("Workspace is active");

    let Some(monitor_id) = target.monitor_id else {
        anyhow::bail!("Workspace is not on any monitor");
    };

    if monitor_id == Monitor::get_active()?.id {
        tracing::debug!("Workspace is already on the active monitor");
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(target.id))?;
        return Ok(());
    }

    tracing::debug!("Workspace is not on the active monitor",);

    let snd_monitor = Monitors::get()?
        .into_iter()
        .find(|m| m.id == monitor_id)
        .ok_or_else(|| anyhow::anyhow!("Should not have happend: no other monitor"))?;

    if snd_monitor.active_workspace.id == target.id {
        tracing::debug!("Swaping active workspaces");
        hyprland::dispatch!(
            SwapActiveWorkspaces,
            MonitorIdentifier::Current,
            MonitorIdentifier::Id(monitor_id)
        )?;
        return Ok(());
    }

    tracing::debug!("Workspace is not the primary on the second monitor");
    hyprland::dispatch!(
        MoveWorkspaceToMonitor,
        WorkspaceIdentifier::Id(target.id),
        MonitorIdentifier::Current
    )?;
    hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(target.id))?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = cli::parse();
    tracing_subscriber::fmt::init();

    tracing::debug!("args: {:?}", args);

    if args.version {
        println!(
            "{} {} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            std::option_env!("GIT_REV").unwrap_or("unknown")
        );
        return Ok(());
    }

    match &args.command {
        Some(cli::Commands::Move { target }) => move_to_workspace(*target)?,
        None => bail!("No command provided"),
    }
    Ok(())
}
