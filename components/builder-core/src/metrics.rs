// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::borrow::Cow;
use std::sync::{Once, ONCE_INIT};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver, SyncSender};
use std::thread;
use statsd::Client;
use hab_core::env;

// Statsd Application name
pub const APP_NAME: &'static str = "bldr";

// Statsd Listener Address
pub const STATS_ENV: &'static str = "HAB_STATS_ADDR";

pub type ApiEndpoint = &'static str;
pub type InstallationId = u32;

// Public Interface
////////////////////////////////////////////////////////////////////////

/// Metric identifiers will usually be static `str`s, but some may
/// need to be dynamically-generated `String`s. With a `Cow`, we can
/// accept either.
pub type MetricId = Cow<'static, str>;

/// All metrics must implement the Metric trait, as well as one of the
/// type marker traits (e.g., `CounterMetric`).
pub trait Metric {
    /// Generate the metric name to be used
    fn id(&self) -> MetricId;
}

/// Marker trait for counter metrics (don't want to accidentally
/// increment a gauge metric!)
pub trait CounterMetric : Metric {}

/// Increment the given metric by one
pub fn incr<C>(counter: C)
    where C: CounterMetric {
    match sender().send((MetricType::Counter,
                         MetricOperation::Increment,
                         counter.id(),
                         None)) {
        Ok(_) => (),
        Err(e) => error!("Failed to increment counter, error: {:?}", e)
    }
}

// Implementation Details
////////////////////////////////////////////////////////////////////////////////

// Helper types
#[derive(Debug, Clone, Copy)]
enum MetricType {
    Counter,
}

#[derive(Debug, Clone, Copy)]
enum MetricOperation {
    Increment,
}

type MetricValue = f64;
type MetricTuple = (MetricType, MetricOperation, MetricId, Option<MetricValue>);

// One-time initialization
static mut SENDER: *const Sender<MetricTuple> = 0 as *const Sender<MetricTuple>;

static INIT: Once = ONCE_INIT;

fn sender() -> Sender<MetricTuple> {
    unsafe {
        INIT.call_once(|| { SENDER = Box::into_raw(Box::new(init())); });
        (*SENDER).clone()
    }
}

// init creates a worker thread ready to receive and process metric events,
// and returns a channel for use by metric senders
fn init() -> Sender<MetricTuple> {
    let (tx, rx) = channel::<MetricTuple>();
    let (rztx, rzrx) = sync_channel(0); // rendezvous channel

    thread::Builder::new()
        .name("metrics".to_string())
        .spawn(move || receive(rztx, rx))
        .expect("couldn't start metrics thread");

    match rzrx.recv() {
        Ok(()) => tx,
        Err(e) => panic!("metrics thread startup error, err={}", e),
    }
}

// receive runs in a separate thread and processes all metrics events
fn receive(rz: SyncSender<()>, rx: Receiver<MetricTuple>) {
    let mut client = statsd_client();
    rz.send(()).unwrap(); // Blocks until the matching receive is called

    loop {
        let (mtyp, mop, mid, mval): MetricTuple = rx.recv().unwrap();
        debug!("Received metrics tuple: {:?}", (mtyp, mop, &mid, mval));

        match client {
            Some(ref mut cli) => {
                match mtyp {
                    MetricType::Counter => {
                        match mop {
                            MetricOperation::Increment => cli.incr(&mid),
                        }
                    }
                }
            }
            None => (),
        }
    }
}

fn statsd_client() -> Option<Client> {
    match env::var(STATS_ENV) {
        Ok(addr) => {
            info!("Creating statsd client with addr: {}", addr);
            match Client::new(&addr, APP_NAME) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!("Error creating statsd client: {:?}", e);
                    None
                }
            }
        }
        Err(_) => None,
    }
}
