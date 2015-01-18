use time;
use hyper;
use hyper::method::Method;
use regex;
use url::{SchemeData, RelativeSchemeData, Host, Url};
use std::default::Default;
use std::fmt;
use std::thread::Thread;
use std::sync::{Arc, Mutex, RwLock};
use std::time::duration::Duration;
use std::io;


/// Status of the request
///
/// Use Arc<RwLock<RequestStatus<_>>> for a kind of non-blocking future on the
/// request.
#[derive(Show, Clone, Copy, PartialEq)]
pub enum RequestStatus<T, E> {
    Pending,
    Complete(T),
    Failed(E)
}

impl<T, E> RequestStatus<T, E> {
    pub fn new() -> RequestStatus<T, E> {
        RequestStatus::Pending
    }
}


/// Represents a shard space
#[derive(Show, Clone)]
pub struct ShardSpace {
    name: String,
    retention_policy: Duration,
    shard_duration: Duration,
    regex: String,
    replication_factory: u16,
    split: u16
}

impl Default for ShardSpace {
    fn default() -> ShardSpace {
        ShardSpace {
            name: String::from_str(""),
            retention_policy: Duration::days(60),
            shard_duration: Duration::days(14),
            regex: String::from_str(".*"),
            replication_factory: 1,
            split: 1
        }
    }
}

/// A datapoint consists of some serialized data and a timestamp
pub struct DataPoint {
#[derive(Show, Clone)]
    pub time: time::Timespec,
    pub data: String
}

/// Represents an influx db service - might be spread over multiple
/// servers, multiple dbs etc...
pub struct Influx {
    /// The cluster instance to use for requests
    cluster: Cluster,
    /// The username for the account to use
    username: String,
    /// The password for the account to use
    password: String
}

impl Influx {
    pub fn new(scheme: Scheme, host: Host, port: u16,
           username: String, password: String) -> Cluster {
        Influx{
            cluster: Cluster {
                instances_available: Arc::new(Mutex::new(vec!(Instance{
                    scheme: scheme,
                    host: host,
                    port: port
                }))),
                ..Default::default()
            },
            username: username,
            password: password
        }
    }

    /// Create a new database - requires cluster admin privileges
    pub fn create_database(&mut self, name: String) -> Arc<RwLock<RequestStatus<(), ()>>> {
        self.request(Method::Post,
                           vec!(String::from_str("cluster"),
                                String::from_str("database_configs"),
                                name),
                           Some(vec!(("u", self.username), ("p", self.password))))
    }

    /// Delete a database - requires cluster admin privileges
    pub fn delete_database(&self, name: String) -> Result<(), String> {
        unimplemented!();
    }

    /// Get a list of databases - requires cluster admin privileges
    pub fn get_database_names(&self) -> Result<Vec<String>, String> {
        unimplemented!();
    }

    /// Get all users for a database - requires cluster admin privileges
    pub fn get_users(&self, db: String) -> Result<Vec<String>, String> {
        unimplemented!();
    }

    /// Get a user for a database - requires cluster admin privileges
    pub fn get_user(&self, db: String) -> Result<String, String> {
        unimplemented!();
    }

    /// Create a new user - requires cluster admin privileges
    pub fn create_user(&self, db: String, username: String,
                       password: String) -> Result<String, String> {
        unimplemented!();
    }

    /// Update existing user - requires cluster admin privileges
    pub fn update_user(&self, db: String, username: String,
                       options: String) -> Result<String, String> {
        unimplemented!();
    }

    /// Get database
    pub fn database(&self, name: String) -> Database {
        Database {
            cluster: self,
            name: name
        }
    }

    /// Set request timeout - default None (disabled)
    pub fn set_request_timeout(&mut self, value: Option<u32>) {
        self.request_timeout = value;
    }

    /// Set failover timeout - default 60s
    pub fn set_failover_timeout(&mut self, value: Duration) {
        *self.failover_timeout.lock().unwrap() = value;
    }

    /// Returns a copy of the vector of available hosts
    pub fn get_instances_available(&self) -> Vec<Instance> {
        (*self.instances_available).lock().unwrap().clone()
    }

    /// Returns a copy of the vector of disabled hosts
    pub fn get_instances_disabled(&self) -> Vec<Instance> {
        (*self.instances_disabled).lock().unwrap().clone()
    }
}


pub struct Database<'a> {
    cluster: &'a Cluster,
    pub name: String
}

impl<'a> Database<'a> {

    /// Get all series names from given database - requires database admin privileges
    pub fn get_series_names(&self, db: String) -> Result<Vec<String>, String> {
        unimplemented!();
    }

    fn write_point(&self, series: String, point: DataPoint,
                   options: String) -> Result<(), String> {
        unimplemented!();
    }

    fn write_points(&self, series: String, point: Vec<DataPoint>,
                    options: String) -> Result<(), String> {
        unimplemented!();
    }

    fn write_series(&self, series: Vec<(String, Vec<DataPoint>)>,
                    options: String) -> Result<(), String> {
        unimplemented!();
    }

    /// Query the database. Note that creating continuous queries requires db admin privileges
    fn query(&self, query: String) -> Result<String, String> {
        unimplemented!();
    }

    /// Requires db admin privileges
    fn get_continuous_queries(&self) -> Result<String, String> {
        unimplemented!();
    }

    /// Requires db admin privileges
    fn drop_continuous_query(&self, query_id: usize) -> Result<String, String> {
        unimplemented!();
    }

    /// Create shard space for db - requires cluster admin privileges
    fn create_shart_space(&self, shard_space: ShardSpace) -> Result<(), String> {
        unimplemented!();
    }

    fn update_shard_space(&self, shard_space: ShardSpace) -> Result<(), String> {
        unimplemented!();
    }

    fn delete_shard_space(&self, shard_space_name: String) -> Result<(), String> {
        unimplemented!();
    }

    fn drop_series(&self, series_name: String) -> Result<(), String> {
        unimplemented!();
    }
}
