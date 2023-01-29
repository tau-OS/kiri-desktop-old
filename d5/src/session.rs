//! logind session management

use color_eyre::Result;
use event_listener::Event;
use futures::{pending, StreamExt};
use logind_zbus::manager::{self, ManagerProxy};
use logind_zbus::session::SessionProxy;
use tracing::{debug, info};
use zbus::dbus_interface;
use zbus::fdo::DBusProxy;
use zbus::ObjectServer;
use zbus_systemd::systemd1::ManagerProxy as SystemdManagerProxy;

use crate::config::Config;

// catch the signal when ending session
struct D5 {
    pub quit_event: Event,
}

#[dbus_interface(name = "com.fyralabs.d5")]
impl D5 {
    fn goodbye_declaration(&self) {
        info!("Stopping session");
        self.quit_event.notify(1);
    }
}

// session management
pub async fn new_session(config: Config) -> Result<()> {
    let conn = zbus::Connection::session().await?;

    // load the systemd target for the session

    // let systemd_manager = SystemdManagerProxy::new(&conn).await?;
    // let target = systemd_manager.start_unit(unit, "replace".to_string()).await?;

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

    // shell_words to split command
    let cmd = shell_words::split(&config.session.leader).unwrap();
    let (cmd, args) = cmd.split_first().unwrap();

    let mut cmd = tokio::process::Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to spawn command");

    // activate session
    // manager.activate_session(&session_id).await?;
    let sys = zbus::Connection::system().await?;
    let manager = ManagerProxy::new(&sys).await?;
    // recieve exit code from target

    let sess = SessionProxy::builder(&sys)
        .path("/org/freedesktop/login1/session/auto")?
        .build()
        .await?;
    let id = sess.id().await?;
    debug!("Session ID: {:?}", id);

    let session = manager.get_session(&id).await?;

    sess.activate().await?;
    debug!("Session: {:?}", session);

    let event = Event::new();
    let listener = event.listen();
    let session = D5 { quit_event: event };

    let handle = crate::proc::BusHandle::from_interface(
        session,
        "com.fyralabs.d5".to_owned(),
        "/com/fyralabs/d5".to_owned(),
    )
    .await?;
    // object server
    crate::proc::HandleManager::fetch().add_handle(handle);

    // tokio select wait for listener signal or wait for cmd to finish
    tokio::select! {
        _ = cmd.wait() => {
            info!("Command finished");
        }
        _ = listener => {
            info!("Listener finished");
        }
    }
    // listener.await;
    Ok(())
}
