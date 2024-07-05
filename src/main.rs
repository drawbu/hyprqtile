mod cli;

use anyhow::bail;
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::{
    data::{Monitor, Workspaces},
    dispatch::{Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifierWithSpecial},
};

/// Move window to a workspace
///
/// If there the workspace is active on a window, switch the two windows
fn move_to_workspace(workspace_id: i32) -> anyhow::Result<()> {
    let workspaces = Workspaces::get()?;
    let target = match workspaces.iter().find(|w| w.id == workspace_id) {
        Some(w) => w,
        None => {
            cli::log(&format!("move to inactive workspace {:#?}", workspace_id));
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_id))?;
            return Ok(());
        }
    };
    cli::log(&format!("move to active workspace {:#?}", target));

    if target.monitor_id == Monitor::get_active()?.id {
        hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(target.id))?;
        return Ok(());
    }

    hyprland::dispatch!(
        SwapActiveWorkspaces,
        MonitorIdentifier::Current,
        MonitorIdentifier::Id(target.monitor_id)
    )?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = cli::parse();
    cli::log(&format!("args: {:#?}", args));

    match &args.command {
        Some(cli::Commands::Move { target }) => move_to_workspace(*target)?,
        None => bail!("No command provided"),
    }
    Ok(())
}
