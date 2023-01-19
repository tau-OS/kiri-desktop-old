//! logind session management

use ashpd::zbus::dbus_interface;
use futures::{pending, StreamExt};
use logind_zbus::session::SessionProxy;
use logind_zbus::manager::{ManagerProxy, self};
use zbus::ObjectServer;
use zbus::fdo::DBusProxy;
use zbus_systemd::systemd1::ManagerProxy as SystemdManagerProxy;
use color_eyre::Result;
use tracing::{debug, info};
use event_listener::Event;

// catch the signal when ending session
struct D5 {
    pub quit_event: Event
}


#[dbus_interface(name = "com.fyralabs.d5")]
impl D5 {
    fn stop(&self) {
        info!("Stopping session");
        self.quit_event.notify(1);
    }
}

// session management
pub async fn new_session(unit: String) -> Result<()> {
    let conn = zbus::Connection::session().await?;

    // load the systemd target for the session

    let systemd_manager = SystemdManagerProxy::new(&conn).await?;
    // let name = "gnome-session-x11.target";
    let target = systemd_manager.start_unit(unit, "replace".to_string()).await?;
    // kill d5 when the target is stopped


    // systemd: watch this unit
    // let mut systemd_manager = SystemdManagerProxy::new(&conn).await?;
    // systemd_manager.subscribe().await?;
    // let mut stream = systemd_manager.receive_unit_removed().await?;
    // tokio::spawn(async move {
    //     let t = target;
    //     while let Some(unit) = stream.next().await {
    //         debug!("Unit removed: {:?}", unit);
    //     }
    // });

    // activate session
    // manager.activate_session(&session_id).await?;
    let sys = zbus::Connection::system().await?;
    let manager = ManagerProxy::new(&sys).await?;
    // recieve exit code from target

    let sess = SessionProxy::builder(&sys)
        .path("/org/freedesktop/login1/session/auto")?
        .build().await?;
    let id = sess.id().await?;
    debug!("Session ID: {:?}", id);

    let session = manager.get_session(&id).await?;

    sess.activate().await?;
    debug!("Session: {:?}", session);

    let event = Event::new();
    let listener = event.listen();
    let session = D5 {
        quit_event: event
    };
    

    let handle = crate::proc::BusHandle::from_interface(session, "com.fyralabs.d5".to_owned(), "/com/fyralabs/d5".to_owned()).await?;
    // object server
    crate::proc::HandleManager::fetch().add_handle(handle);

    listener.await;
    Ok(())
}