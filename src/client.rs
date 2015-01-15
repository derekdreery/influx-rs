use time;
use hyper;
use regex;
use std::default::Default;
use std::fmt;
use std::thread;

/// Represents a url protocol
pub enum Protocol {
    Http,
    Https
}

impl Default for Protocol {
    fn default() -> Protocol {
        Protocol::Http
    }
}

impl fmt::String for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Protocol::Http => {
                write!(f, "http")
            },
            &Protocol::Https => {
                write!(f, "https")
            }
        }
    }
}

/// Represents a host & port
pub struct Host {
    pub protocol: Protocol,
    pub hostname: String,
    pub port: u16
}

impl Default for Host {
    fn default() -> Host {
        Host {
            protocol: Default::default(),
            hostname: String::from_str("localhost"),
            port: 8086
        }
    }
}

impl fmt::String for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{protocol}://{hostname}:{port}",
               protocol=self.protocol,
               hostname=self.hostname,
               port=self.port)
    }
}

/// Represents a shard space
pub struct ShardSpace {
    name: String,
    retention_policy: TimePeriod,
    shard_duration: TimePeriod,
    regex: String,
    replication_factory: u16,
    split: u16
}

impl Default for ShardSpace {
    fn default() -> ShardSpace {
        ShardSpace {
            name: String::from_str(""),
            retention_policy: TimePeriod::Days(60),
            shard_duration: TimePeriod::Days(14),
            regex: String::from_str(".*"),
            replication_factory: 1,
            split: 1
        }
    }
}

/// A datapoint consists of some serialized data and a timestamp
pub struct DataPoint {
    pub time: time::Timespec,
    pub data: String
}

/// Handy enum for specifying time periods
pub enum TimePeriod {
    // Add more variants
    Days(u32)
}

/// A cluster takes requests for operations and performs them
/// on a cluster of influxdb instances, transparently handling
/// replication/load balancing
pub struct Cluster {
    pub username: String,
    pub password: String,
    hyper_client: hyper::Client<hyper::net::HttpConnector>,
    request_timeout: Option<u32>,
    failover_timeout: u32,
    hosts_available: Vec<Host>,
    hosts_disabled: Vec<Host>,
    hosts_available_pointer: usize,
    thread_pool: Vec<thread::Thread>
}

impl Default for Cluster {
    fn default() -> Cluster {
        Cluster {
            username: String::from_str(""),
            password: String::from_str(""),
            hyper_client: hyper::Client::new(),
            request_timeout: None,
            failover_timeout: 60,
            hosts_available: vec!(Host::default()),
            hosts_disabled: vec!(),
            hosts_available_pointer: 0,
            thread_pool: vec!()
        }
    }
}

impl Cluster {
    pub fn new(protocol: Protocol, hostname: String, port: u16,
           username: String, password: String) -> Cluster {
        Cluster {
            hosts_available: vec!(Host{
                protocol: protocol,
                hostname: hostname,
                port: port
            }),
            username: username,
            password: password,
            ..Default::default()
        }
    }

    /// Create a new database - requires cluster admin privileges
    pub fn create_database(&self, name: String) -> Result<(), String> {
        unimplemented!();
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
    pub fn set_failover_timeout(&mut self, value: u32) {
        self.failover_timeout = value;
    }

    /// Get hosts available
    pub fn get_hosts_available(&self) -> &Vec<Host> {
        &self.hosts_available
    }

    /// Get hosts available
    pub fn get_hosts_disabled(&self) -> &Vec<Host> {
        &self.hosts_disabled
    }

    // Private helper methods
    fn get_host(&mut self) -> Option<&Host> {
        if self.hosts_available.is_empty() {
            None
        } else {
            if self.hosts_available_pointer > self.hosts_available.len() {
                self.hosts_available_pointer = 0;
            }
            let host = &self.hosts_available[self.hosts_available_pointer];
            self.hosts_available_pointer += 1;
            Some(host)
        }
    }

    fn enable_host(&mut self, pos: usize) {
        let host = self.hosts_disabled.remove(pos);
        self.hosts_available.push(host);
    }

    fn disable_host(&mut self, pos: usize) {
        let host = self.hosts_available.remove(pos);
        self.hosts_disabled.push(host);
        // TODO schedule attempt to reenable
    }

    fn build_url(&self, pos: usize) -> String {
        let host = &self.hosts_available[pos];
        format!("{}://{}:{}/", host.protocol, host.hostname, host.port)
    }

    fn request(&mut self) {
        let host = self.get_host().expect("No hosts left");
        
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
