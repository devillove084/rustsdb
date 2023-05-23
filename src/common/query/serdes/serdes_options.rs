pub(crate) trait SerdesOptions {
    fn get_id(&self) -> String;

    fn get_type(&self) -> String;

    fn get_filter(&self) -> Vec<String>;
}
