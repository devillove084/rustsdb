pub(crate) enum QueryNodeConfigOptions {
    SUPPORTPUSHDOWN(bool),
    JOINS(bool),
    PERVIOUSINTERVALS(i32),
    PADDINGWINDOW(String),
    READCACHEABLE(bool),
}

impl QueryNodeConfigOptions {
    pub fn set_query_node_config_options(mut self, typ: QueryNodeConfigOptions) {
        self = typ;
    }

    pub fn get_query_node_config_options(self) -> Self {
        self
    }
}
