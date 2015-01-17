use time;
use hyper;
use regex;
use url::{SchemeData, RelativeSchemeData, Host, Url};
use std::default::Default;
use std::fmt;
use std::thread::Thread;
use std::sync::{Arc, Mutex};
use std::time::duration::Duration;
use std::io;

/// Represents a url scheme
#[derive(Show, Clone, Copy, PartialEq)]
pub enum Scheme {
    Http,
    Https
}

impl Default for Scheme {
    fn default() -> Scheme {
        Scheme::Http
    }
}

impl fmt::String for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Scheme::Http => {
                write!(f, "http")
            },
            &Scheme::Https => {
                write!(f, "https")
            }
        }
    }
}

/// Represents a http-like scheme, host & port
#[derive(Show, Clone, PartialEq)]
pub struct Instance {
    pub scheme: Scheme,
    pub host: Host,
    pub port: u16
}

impl Default for Instance {
    fn default() -> Instance {
        Instance {
            scheme: Default::default(),
            host: Host::Domain(String::from_str("127.0.0.1")),
            port: 8086
        }
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

/// A cluster takes requests for operations and performs them
/// on a cluster of influxdb instances, transparently handling
/// replication/load balancing
pub struct Cluster {
    pub username: String,
    pub password: String,
    hyper_client: hyper::Client<hyper::net::HttpConnector>,
    request_timeout: Option<u32>,
    failover_timeout: Arc<Mutex<Duration>>,
    instances_available: Arc<Mutex<Vec<Instance>>>,
    instances_disabled: Arc<Mutex<Vec<Instance>>>,
    instances_available_pointer: Mutex<usize>,
    pending_request_threads: Vec<Thread>
}

impl Default for Cluster {
    fn default() -> Cluster {
        Cluster {
            username: String::from_str(""),
            password: String::from_str(""),
            hyper_client: hyper::Client::new(),
            request_timeout: None,
            failover_timeout: Arc::new(Mutex::new(Duration::seconds(60))),
            instances_available: Arc::new(Mutex::new(vec!(Default::default()))),
            instances_disabled: Arc::new(Mutex::new(vec!())),
            instances_available_pointer: Mutex::new(0),
            pending_request_threads: vec!()
        }
    }
}

impl Cluster {
    pub fn new(scheme: Scheme, host: Host, port: u16,
           username: String, password: String) -> Cluster {
        Cluster {
            instances_available: Arc::new(Mutex::new(vec!(Instance{
                scheme: scheme,
                host: host,
                port: port
            }))),
            username: username,
            password: password,
            ..Default::default()
        }
    }

    /// Create a new database - requires cluster admin privileges
    pub fn create_database(&mut self, name: String) -> Result<(), String> {
        self.request(vec!(String::from_str("cluster"),
                          String::from_str("database_configs"),
                          name))
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

    // Private helper methods
    // ======================

    /// Add a new host to available hosts
    fn add_host(&mut self, new_host: Instance) {
        self.instances_available.lock().unwrap().push(new_host);
    }

    fn get_host(&mut self) -> Option<Instance> {
        let instances_available = self.instances_available.lock().unwrap();
        let mut instances_available_pointer = self.instances_available_pointer.lock().unwrap();
        if instances_available.is_empty() {
            None
        } else {
            if *instances_available_pointer > instances_available.len() {
                *instances_available_pointer = 0;
            }
            let host = instances_available[*instances_available_pointer].clone();
            *instances_available_pointer += 1;
            Some(host)
        }
    }

    fn enable_host(&mut self, pos: usize) {
        let host = self.instances_disabled.lock().unwrap().remove(pos);
        self.instances_available.lock().unwrap().push(host);
    }

    fn disable_host(&mut self, pos: usize) {
        let host = self.instances_available.lock().unwrap().remove(pos);
        self.instances_disabled.lock().unwrap().push(host.clone());
        // schedule attempt to reenable
        let instances_disabled = self.instances_disabled.clone();
        let instances_available = self.instances_available.clone();
        let timeout = self.failover_timeout.lock().unwrap().clone();
        Thread::spawn(move || {
            io::timer::sleep(timeout);
            Cluster::remove_instance_by_attrs(instances_disabled, host.clone());
            instances_available.lock().unwrap().push(host);
        });
    }

    /// Helper function to remote a host by attrs
    ///
    /// We do this because we cannot garuntee the position of the host (as
    /// other hosts will have been removed/added since)
    fn remove_instance_by_attrs(instances: Arc<Mutex<Vec<Instance>>>,
                                instance: Instance) {
        let mut instances = instances.lock().unwrap();
        instances.retain(|ref x| **x != instance);
    }

    /// Creates a url for a request
    fn build_url(&self,
                 instance: Instance,
                 path: Vec<String>,
                 query: &mut Vec<(String, String)>) -> Url {
        query.push((String::from_str("u"), self.username.clone()));
        query.push((String::from_str("p"), self.password.clone()));
        Url {
            scheme: format!("{}", instance.scheme),
            scheme_data: SchemeData::Relative(RelativeSchemeData {
                username: String::from_str(""),
                password: None,
                host: instance.host.clone(),
                port: Some(instance.port),
                default_port: Some(instance.port), // TODO what to do here?
                path: path
            }),
            query: Some(::url::form_urlencoded::serialize_owned(query.as_slice())),
            fragment: None
        }
    }

    // TODO write this when we know what we want
    fn request(&mut self, path: Vec<String>) {
        // TODO handle no hosts more gracefully
        let instance = self.get_host().expect("No hosts left");
        self.build_url(instance, path, &vec!());
        
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
