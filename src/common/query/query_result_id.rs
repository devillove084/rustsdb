pub(crate) trait QueryResultID {
    fn node_id(&self) -> String;

    fn data_source(&self) -> String;
}
