//! This module contains state machine definitions for the backend rendering system.

sad_machine::state_machine! {
    RenderBackendStates {
        InitialStates {
            Preload, SmFailed
        }
        FinishPreload {
            Preload => Loading
        }
        FinishLoading {
            Loading => RenderGame
        }
        ForceSmFailure {
            Preload => SmFailed,
            Loading => SmFailed,
            RenderGame => SmFailed
        }
    }
}
