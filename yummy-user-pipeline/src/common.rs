pub use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    fs::File,
    io::{BufReader, Read, Write},
    ops::Deref,
    path::Path,
    str::FromStr,
    sync::Arc,
};

pub use derive_new::new;

pub use reqwest::Client;

pub use tokio::{
    io::AsyncReadExt,
    signal,
    sync::{Mutex, MutexGuard},
    time::{sleep, Duration, Interval},
};

pub use dotenv::dotenv;

pub use getset::{Getters, Setters};

pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use anyhow::anyhow;

pub use serde_json::{from_reader, json, Value};

pub use async_trait::async_trait;

pub use log::{error, info, warn};

pub use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, Record};

//pub use futures::{future::join_all, stream::TryStreamExt, Future};

// pub use rdkafka::consumer::StreamConsumer;
// pub use rdkafka::config::ClientConfig;
// pub use rdkafka::Message;

pub use once_cell::sync::Lazy as once_lazy;

pub use lettre::message::{Mailbox, MultiPart, SinglePart};
pub use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

// pub use elasticsearch::{
//     http::response::Response,
//     http::transport::{ConnectionPool, Transport as EsTransport},
//     http::transport::{SingleNodeConnectionPool, TransportBuilder},
//     http::Url,
//     Elasticsearch, IndexParts, SearchParts, DeleteParts,
// };

// pub use rand::{prelude::SliceRandom, rngs::StdRng, SeedableRng};

// pub use lettre::{
//     message::{MultiPart, SinglePart},
//     transport::smtp::authentication::Credentials,
//     AsyncSmtpTransport, AsyncTransport, Message, Transport,
// };

// pub use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};

// pub use chrono_tz::Asia::Seoul;

// pub use cron::Schedule;

// pub use regex::Regex;

// pub use num_format::{Locale, ToFormattedString};
