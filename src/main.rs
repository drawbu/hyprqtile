mod cli;

use anyhow::bail;

use hyprland::data::{Monitor, Workspace, Workspaces};
use hyprland::dispatch::{
    Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::shared::{HyprData, HyprDataActive};

/// Move window to a workspace
///
/// If there the workspace is active on a window, switch the two windows
fn move_to_workspace(t_workspace: &Workspace) -> anyhow::Result<()> {
    cli::log(&format!("move on monitor {:#?}", t_workspace));

    if t_workspace.monitor_id == Monitor::get_active()?.id {
        hyprland::dispatch!(
            Workspace,
            WorkspaceIdentifierWithSpecial::Id(t_workspace.id)
        )?;
        return Ok(())
    }

    hyprland::dispatch!(
        SwapActiveWorkspaces,
        MonitorIdentifier::Current,
        MonitorIdentifier::Id(t_workspace.monitor_id)
    )?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = cli::parse();
    cli::log(&format!("args: {:#?}", args));

    match &args.command {
        Some(cli::Commands::Move { target }) => {
            let workspaces = Workspaces::get()?;
            let workspace = match workspaces.iter().find(|w| w.id == *target) {
                Some(w) => w,
                None => bail!("workspace not found"),
            };
            move_to_workspace(&workspace)?;
        }
        None => bail!("No command provided"),
    }
    Ok(())
}
