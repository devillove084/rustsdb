pub(crate) enum WriteState {
    OK,
    RETRY,
    REJECTED,
    ERROR,
}

#[async_trait::async_trait]
pub(crate) trait WriteStatus {
    async fn state(&self) -> WriteState;

    async fn message(&self) -> String;

    fn ok(&self) -> WriteState {
        WriteState::OK
    }

    fn retry(&self) -> WriteState {
        WriteState::RETRY
    }

    fn reject(&self) -> WriteState {
        WriteState::REJECTED
    }

    // fn retry_with_msg(&self, msg: String) -> WriteState {
    //     if msg.is_empty() {
    //         return WriteState::RETRY;
    //     }
    // }
}
