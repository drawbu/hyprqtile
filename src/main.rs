mod cli;

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
    let Some(target) = Workspaces::get()?
        .into_iter()
        .find(|w| w.id == workspace_id)
    else {
        tracing::debug!("Workspace is not currently used");
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_id))?;
        return Ok(());
    };

    tracing::debug!("Workspace is used");

    if let Some(monitor_id) = target.monitor_id {
        if monitor_id == Monitor::get_active()?.id {
            tracing::debug!("Workspace is already on the active monitor");
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(target.id))?;
            return Ok(());
        }

        tracing::debug!("Workspace is not on the active monitor");

        // Workspace is the active one on the other monitor. No need to move,
        // just swap. If it not, the target's monitor is irrelevant
        if let Some(snd_monitor) = Monitors::get()?.into_iter().find(|m| m.id == monitor_id)
            && snd_monitor.active_workspace.id == target.id
        {
            tracing::debug!("Both workspaces are actives. Swaping places");
            hyprland::dispatch!(
                SwapActiveWorkspaces,
                MonitorIdentifier::Current,
                MonitorIdentifier::Id(monitor_id)
            )?;
            return Ok(());
        }
    };

    tracing::debug!("Bringing workspace to current monitor");
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
        let rev = std::option_env!("BUILD_REV").unwrap_or_else(|| {
            tracing::warn!("BUILD_REV needs to be set at compile-time");
            "unknown"
        });
        println!("{} {}", env!("CARGO_PKG_NAME"), rev);
        return Ok(());
    }

    match args.command {
        None => Err(anyhow::anyhow!("No command provided")),
        Some(c) => match c {
            cli::Commands::Move { target } => move_to_workspace(target),
        },
    }
}
