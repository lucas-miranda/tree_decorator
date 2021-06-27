use crate::{
    Decorator,
    ElementHandler,
    StandardDecorator
};

/// Builds a [`Decorator`] and prepares it.
/// 
/// A builder can be created by providing a custom [`Decorator`] 
/// through [ref](DecoratorBuilder::with_ref) or [moving](DecoratorBuilder::with) value.
///
/// By default, [`StandardDecorator`] will be used.
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
    /// Starts builder with a [`Decorator`] static reference.
    pub fn with_ref<D: 'static + Decorator>(decorator: &'static D) -> Self {
        DecoratorBuilder {
            decorator,
            handler_fallback: Some(Box::new(do_nothing_handler))
        }
    }

    /// Starts builder with a [`Decorator`].
    pub fn with<D: 'static + Decorator>(decorator: D) -> Self {
        Self::with_ref(Box::<D>::leak::<'static>(Box::new(decorator)))
    }

    /// Defines a default handler to tree items.
    ///
    /// Each call to [`tree_item!`] will also call provided handler.
    ///
    /// It isn't required, but can be used to simplify calls.
    pub fn default_handler<H: 'static + FnMut(String)>(mut self, handler: H) -> Self {
        self.handler_fallback = Some(Box::new(handler));
        self
    }

    /// Completes decorator build and effectively starts using it.
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

/// Placeholder handler
fn do_nothing_handler(_m: String) {
}
