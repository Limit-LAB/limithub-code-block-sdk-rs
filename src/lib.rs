pub mod error;
pub mod port;
pub mod value;

use std::{
    collections::{BTreeSet, HashMap},
    future::Future,
    io,
    net::SocketAddr,
    sync::Arc,
};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};

pub use error::{Error, Result};
pub use port::Port;
use serde::Serialize;
use tokio::net::TcpListener;
pub use value::{PortValue, PortValueType};

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlockAppBuilder {
    inputs: Vec<Port>,
    outputs: Vec<Port>,
}

impl CodeBlockAppBuilder {
    pub fn new() -> Self {
        CodeBlockAppBuilder {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn inputs(mut self, inputs: impl IntoIterator<Item = Port>) -> Self {
        self.inputs.extend(inputs);
        self
    }

    pub fn outputs(mut self, outputs: impl IntoIterator<Item = Port>) -> Self {
        self.outputs.extend(outputs);
        self
    }

    pub fn build(self) -> Result<CodeBlockApp> {
        if self.inputs.is_empty() {
            return Err(Error::EmptyPorts("inputs"));
        }

        let mut inputs = BTreeSet::new();
        let mut outputs = BTreeSet::new();

        for input in self.inputs {
            let name = input.name.clone();
            inputs
                .insert(input)
                .then_some(())
                .ok_or(Error::NameConflict(name))?;
        }

        for output in self.outputs {
            let name = output.name.clone();
            outputs
                .insert(output)
                .then_some(())
                .ok_or(Error::NameConflict(name))?;
        }

        Ok(CodeBlockApp { inputs, outputs })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlockApp {
    inputs: BTreeSet<Port>,
    outputs: BTreeSet<Port>,
}

impl CodeBlockApp {
    pub async fn serve<Fut>(
        self,
        handler: fn(HashMap<String, PortValue>) -> Fut,
    ) -> impl Future<Output = io::Result<()>>
    where
        Fut: Future<Output = Result<HashMap<String, PortValue>, String>> + Send + 'static,
    {
        let app = Router::new()
            .route("/ports", get(get_ports))
            .route("/run", post(move |app, req| run(app, req, handler)))
            .layer(Extension(Arc::new(self)));

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        async move {
            let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;
            axum::serve(listener, app).await
        }
    }

    fn validate_inputs(&self, inputs: &HashMap<String, PortValue>) -> Result<()> {
        self.inputs
            .iter()
            .map(|i| {
                inputs
                    .contains_key(&i.name)
                    .then_some(())
                    .ok_or(Error::MissingPort(i.name.clone()))
            })
            .collect::<Result<()>>()
    }

    fn validate_outputs(&self, outputs: &HashMap<String, PortValue>) -> Result<()> {
        self.outputs
            .iter()
            .map(|i| {
                outputs
                    .contains_key(&i.name)
                    .then_some(())
                    .ok_or(Error::MissingPort(i.name.clone()))
            })
            .collect::<Result<()>>()
    }
}

async fn get_ports(Extension(app): Extension<Arc<CodeBlockApp>>) -> Json<CodeBlockApp> {
    Json(app.as_ref().clone())
}

async fn run<Fut>(
    Extension(app): Extension<Arc<CodeBlockApp>>,
    Json(req): Json<HashMap<String, PortValue>>,
    handler: fn(HashMap<String, PortValue>) -> Fut,
) -> Result<Json<HashMap<String, PortValue>>, (StatusCode, String)>
where
    Fut: Future<Output = Result<HashMap<String, PortValue>, String>> + 'static + Send,
{
    app.validate_inputs(&req)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let outputs = handler(req)
        .await
        .map(|res| Json(res))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    app.validate_outputs(&outputs)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{CodeBlockAppBuilder, Port, PortValue, PortValueType};

    #[test]
    fn test() {
        _ = CodeBlockAppBuilder::new()
            .inputs([Port::new(String::from("port1"), PortValueType::Bool)])
            .build()
            .unwrap()
            .serve(handler);
    }

    async fn handler(
        _req: HashMap<String, PortValue>,
    ) -> Result<HashMap<String, PortValue>, String> {
        Ok(HashMap::new())
    }
}
