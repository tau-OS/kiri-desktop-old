//! process management

use color_eyre::Result;
use lazy_static::lazy_static;
use parking_lot::{Mutex, MutexGuard};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug_span, instrument};
use zbus::{Connection, ConnectionBuilder, Interface};

type Manager = Arc<Mutex<HandleManager>>;
lazy_static! {
    static ref HANDLE_MANAGER: Manager = Arc::new(Mutex::new(HandleManager::new()));
}

#[derive(Debug)]
pub struct BusHandle {
    pub conn: Connection,
    pub name: String,
    pub path: String,
}

impl BusHandle {
    /// Create a new BusHandle from a connection, name, and path
    pub fn new(conn: Connection, name: String, path: String) -> Self {
        Self { conn, name, path }
    }
    pub async fn from_interface<T: Interface>(
        interface: T,
        name: String,
        path: String,
    ) -> Result<Self> {
        let s = tracing::span!(tracing::Level::TRACE, "from_interface", name = %name, path = %path);
        let _e = s.enter();
        let conn = ConnectionBuilder::session()?
            .name(name.clone())?
            .serve_at(path.clone(), interface)?
            .build()
            .await?;
        Ok(Self::new(conn, name.into(), path.into()))
    }

    /// Get the connection
    #[instrument]
    pub fn get_conn(&self) -> &Connection {
        &self.conn
    }

    /// Ends the dbus connection.
    ///
    /// Private, because the handle manager will handle this
    #[instrument]
    async fn end(&self) {
        self.conn.release_name(self.name.clone()).await.unwrap();
    }
}

pub struct HandleManager {
    pub handles: HashMap<String, BusHandle>,
}

impl HandleManager {
    fn new() -> Self {
        Self {
            handles: HashMap::new(),
        }
    }

    /// Get the handle manager
    pub fn fetch() -> MutexGuard<'static, HandleManager> {
        HANDLE_MANAGER.lock()
    }

    // HandleManager will be a singleton, so we will implement a function to either
    // get a handle to the singleton or create a new one if it doesn't exist

    pub fn add_handle(&mut self, handle: BusHandle) {
        debug_span!("add_handle", name = %handle.name, path = %handle.path ).in_scope(|| {
            self.handles.insert(handle.name.clone(), handle);
        });
    }

    pub async fn gen_handle<T: Interface>(
        &mut self,
        interface: T,
        name: String,
        path: String,
    ) -> Result<()> {
        self.add_handle(BusHandle::from_interface(interface, name, path).await?);
        Ok(())
    }

    pub fn get_handle(&self, name: &str) -> Option<&BusHandle> {
        self.handles.get(name)
    }

    pub fn get_handle_mut(&mut self, name: &str) -> Option<&mut BusHandle> {
        self.handles.get_mut(name)
    }

    pub async fn remove_handle(&mut self, name: &str) -> Result<()> {
        Ok({
            self.handles.get(name).unwrap().end().await;
            self.handles.remove(name);
        })
    }

    pub fn get_handles(&self) -> &HashMap<String, BusHandle> {
        &self.handles
    }
}
