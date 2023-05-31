use dyn_clone::{clone_trait_object, DynClone};

pub(crate) trait QueryExecutorFactory: DynClone {
    fn id(&self) -> String;

    // fn executor_type(&self) -> Box<dyn QueryExecutorFactory>;

    fn new_executor(&self) -> Box<dyn QueryExecutorFactory>;
}

clone_trait_object!(QueryExecutorFactory);
