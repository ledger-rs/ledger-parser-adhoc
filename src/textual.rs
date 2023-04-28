use crate::context::{ParseContext, ParseContextStack};

/**
 * textual.cc
 */

pub(crate) struct Instance<'a> {
    pub context_stack: &'a ParseContextStack<'a>,
    pub context: &'a ParseContext<'a>,
    //pub apply_stack: Vec<Application>
}

impl<'a> Instance<'a> {
    pub fn new(context_stack: &'a ParseContextStack<'a>,
        context: &'a ParseContext<'a>) -> Self {
        Self {
            context_stack,
            context,
        }
    }

    pub fn parse(&self) -> usize {
        log::info!("Parsing {:?}", self.context.pathname);

        // todo: self.context.linenum
        
        todo!()
    }
}
