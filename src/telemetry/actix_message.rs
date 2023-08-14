use actix::Message;
use tracing::Span;

pub struct TraceMessage<M: Message> {
    pub message: M,
    pub span: Span
}

impl<M: Message> Message for TraceMessage<M> {
    type Result = M::Result;
}

pub trait TraceMessageExt {
    type Message: Message + Sized;

    fn trace(self) -> TraceMessage<Self::Message>;
    fn with_span(self, span: Span) -> TraceMessage<Self::Message>;
}

impl<T> TraceMessageExt for T where T : Message {
    type Message = T;

    fn trace(self) -> TraceMessage<Self> {
        TraceMessage {
            message: self,
            span: Span::current()
        }
    }

    fn with_span(self, span: Span) -> TraceMessage<Self> {
        TraceMessage {
            message: self,
            span: span.or_current()
        }
    }
}

#[macro_export]
macro_rules! trace_handler {
    ($actor:ty, $message:ty, $result:ty) => {
        impl actix::Handler<TraceMessage<$message>> for $actor {
            type Result = $result;

            fn handle(&mut self, msg: TraceMessage<$message>, ctx: &mut Self::Context) -> Self::Result {
                let _entered = msg.span.enter();
                self.handle(msg.message, ctx)
            }
        }
    };
}