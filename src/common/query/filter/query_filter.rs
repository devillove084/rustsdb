use std::hash::Hash;

use crate::common::stats::span::Span;

#[async_trait::async_trait]
pub(crate) trait QueryFilter {
    fn get_type(&self) -> String;

    async fn initialize(&self, span: Box<dyn Span>);
}

// impl Hash for dyn QueryFilter<FilterSpan = > {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         todo!()
//     }
// }
