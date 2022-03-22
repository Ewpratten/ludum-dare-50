//! This module contains state machine definitions for the backend rendering system.

sad_machine::state_machine! {
    RenderBackendStates {
        InitialStates {
            Preload
        }
        FinishPreload {
            Preload => Loading
        }
        FinishLoading {
            // TODO: Make this hand off to the main render code
            Loading => SmFailed
        }
    }
}
