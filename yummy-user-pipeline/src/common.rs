pub use std::{
    env, fs,
    fs::File,
    io::{BufReader, Write},
    path::Path,
    sync::Arc,
};

pub use tokio::task::JoinHandle;

pub use derive_new::new;

pub use dotenv::dotenv;

pub use getset::Getters;

pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use anyhow::anyhow;

pub use serde_json::{from_reader, Value};

pub use async_trait::async_trait;

pub use log::{error, info};

pub use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, Record};

pub use rdkafka::{
    config::ClientConfig,
    consumer::{CommitMode, Consumer, DefaultConsumerContext, MessageStream, StreamConsumer},
    message::BorrowedMessage,
    Message,
};

pub use futures::stream::StreamExt;

pub use once_cell::sync::Lazy as once_lazy;

pub use lettre::message::{Mailbox, MultiPart, SinglePart};
pub use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    Message as letMessage, Tokio1Executor,
};
