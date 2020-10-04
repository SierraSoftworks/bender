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
            span
        }
    }
}