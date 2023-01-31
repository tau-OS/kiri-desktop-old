use smithay::{delegate_shm, wayland::shm::ShmHandler};

use crate::state::GyakuState;

impl ShmHandler for GyakuState {
    fn shm_state(&self) -> &smithay::wayland::shm::ShmState {
        &self.shm_state
    }
}

delegate_shm!(GyakuState);
