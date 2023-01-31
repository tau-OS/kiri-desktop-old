use smithay::{delegate_shm, wayland::shm::ShmHandler};
use tracing::instrument;

use crate::state::GyakuState;

impl ShmHandler for GyakuState {
    #[instrument(skip(self))]
    fn shm_state(&self) -> &smithay::wayland::shm::ShmState {
        &self.shm_state
    }
}

delegate_shm!(GyakuState);
