// xdg notifications
use color_eyre::Result;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::debug;
use zbus::fdo::MonitoringProxy;
use zbus::zvariant::{Structure, Value, OwnedValue};
use zbus::MessageStream;

#[derive(Debug, Default, Serialize, Deserialize, zbus::zvariant::Type)]
pub struct Notification {
    pub app_id: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    // variant of u8 or u32
    pub hints: NotificationHint,
    pub expire_timeout: i32,
}

pub enum Urgency {
    Low = 0,
    Normal = 1,
    High = 2,
}


#[derive(Default,Debug, zbus::zvariant::DeserializeDict, zbus::zvariant::Type, zbus::zvariant::SerializeDict)]
#[zvariant(signature = "a{sv}")]
pub struct NotificationHint {
    // length must be larger than >= 420081
    // no padding bytes
    #[zvariant(rename = "sender-pid")]
    pub sender_pid: i64,
    pub urgency: u8,
    pub transient: Option<bool>,

    // other hints
    pub category: Option<String>,
}


impl Notification {
    fn new() -> Self {
        Self {
            app_id: String::new(),
            replaces_id: 0,
            app_icon: String::new(),
            summary: String::new(),
            body: String::new(),
            actions: Vec::new(),
            hints: NotificationHint::default(),
            expire_timeout: 0,
        }
    }

    pub fn display(&self) -> String {
        format!("{}: {}", self.summary, self.body)
    }

    pub fn is_transient(&self) -> bool {
        self.hints.transient.unwrap_or(false)
    }
}

#[zbus::dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {}

pub async fn listen() -> Result<()> {
    // monitor notifications, do not replace existing daemon
    let conn = zbus::Connection::session().await?;
    // call monitor
    let mon = MonitoringProxy::builder(&conn)
        .destination("org.freedesktop.DBus")?
        .interface("org.freedesktop.DBus.Monitoring")?
        .path("/org/freedesktop/DBus")?
        .build()
        .await?;

    let funny = NotificationsProxy::new(&conn).await?;
    // listen for notifications
    // match rules to get just notifications
    let match_rules = vec![
        "interface='org.freedesktop.Notifications'",
        // "type='method_call'",
        // "member='Notify'",
    ];

    // funny.become_monitor(match_rules.as_slice(), 0).await?;

    mon.become_monitor(match_rules.as_slice(), 0).await?;
    let mut stream = MessageStream::from(&mon.connection().clone());
    while let Some(msg) = stream.try_next().await? {
        // msg.message_type();

        if msg.message_type() != zbus::MessageType::MethodCall {
            continue;
        }

        if let Some(member) = msg.member() {
            debug!(target: "dbus", "member: {}", member);

            match member.as_str() {
                "Notify" => {
                    let body = msg.body::<Notification>();
                    if body.is_err() {
                        debug!("Error: {:?}", body);

                        let b = msg.body::<Value>().ok();
                        debug!("Structure: {:#?}", b);
                        continue;
                    } else {
                        debug!("Notification: {:#?}", body);
                    }
                }
                _ => {
                    // debug!("Message: {:#?}", msg);
                }
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn test() {
    color_eyre::install().unwrap();
    listen().await.unwrap();
}
