//! logind session management

use logind_zbus::session::SessionProxy;
use logind_zbus::manager::ManagerProxy;
use zbus_systemd::systemd1::ManagerProxy as SystemdManagerProxy;
use color_eyre::Result;
use tracing::debug;


// session management
pub async fn new_session(unit: String) -> Result<()> {
    let conn = zbus::Connection::session().await?;

    // load the systemd target for the session

    let systemd_manager = SystemdManagerProxy::new(&conn).await?;
    // let name = "gnome-session-x11.target";
    let target = systemd_manager.start_unit(unit, "user".to_string()).await?;

    // activate session
    // manager.activate_session(&session_id).await?;


    Ok(())
}