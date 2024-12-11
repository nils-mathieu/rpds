use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::Handle,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    handle: Handle,
}

#[instrument(name = "com.atproto.identity.updateHandle", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        handle = ?input.handle,
    );
    unimplemented!();
}

/// `com.atproto.identity.updateHandle`
pub fn route() -> impl Handler {
    handler.into_handler()
}