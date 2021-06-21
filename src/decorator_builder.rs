use crate::{
    Decorator,
    ElementHandler,
    StandardDecorator
};

pub struct DecoratorBuilder {
    decorator: &'static dyn Decorator,
    handler_fallback: Option<Box<dyn FnMut(String)>>
}

impl Default for DecoratorBuilder {
    fn default() -> Self {
        DecoratorBuilder::with(StandardDecorator::default())
    }
}

impl DecoratorBuilder {
    pub fn with_ref<D: 'static + Decorator>(decorator: &'static D) -> Self {
        DecoratorBuilder {
            decorator,
            handler_fallback: Some(Box::new(do_nothing_handler))
        }
    }

    pub fn with<D: 'static + Decorator>(decorator: D) -> Self {
        Self::with_ref(Box::<D>::leak::<'static>(Box::new(decorator)))
    }

    pub fn default_handler<H: 'static + FnMut(String)>(mut self, handler: H) -> Self {
        self.handler_fallback = Some(Box::new(handler));
        self
    }

    pub fn build(mut self) -> Self {
        unsafe {
            crate::DECORATOR = self.decorator;

            crate::ELEMENT_HANDLER = Some(ElementHandler::new(
                self.handler_fallback.take().unwrap_or_else(|| Box::new(do_nothing_handler))
            ));
        }

        self
    }
}

fn do_nothing_handler(_m: String) {
}
