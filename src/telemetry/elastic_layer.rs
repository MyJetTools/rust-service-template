use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result as AnyResult;
use rand::prelude::*;
use serde_json::{json, Value};
use tracing::{
    span::{Attributes, Record},
    Event, Id, Level, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};
//use elasticsearch::Elasticsearch;

/// Telemetry capability that publishes events and spans to Elastic APM.
pub struct ElasticLayer {
    //client: Elasticsearch,
}

impl<S> Layer<S> for ElasticLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) { 
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) { 
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        println!("{:?}", event);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {

    }
}

impl ElasticLayer {
    pub(crate) fn new() -> AnyResult<Self> {
        
        Ok(ElasticLayer {
            //client: Elasticsearch::default()?,
        })
    }

    async fn start_writer(&self) {
        //self.client.
    } 
}