
use std::default::Default;
use hyper;
use regex;
use time;

#[derive(Copy)]
pub enum Protocol {
    Http,
    Https
}

/// A host represented by a hostname and port
pub enum Host {
    Single(String, u16),
    Multi(Vec<(String, u16)>)
}

pub struct Cluster {
    pub protocol: Protocol,
    pub host: Host,
    pub username: Option<String>,
    pub password: Option<String>,
    hyper_client: hyper::Client<hyper::net::HttpConnector>,
    request_timeout: Option<u32>,
    failover_timeout: u32
}

pub struct Database<'a> {
    cluster: &'a Cluster,
    pub name: String
}

pub struct ShardSpace {
    name: String,
    retention_policy: TimePeriod,
    shard_duration: TimePeriod,
    regex: regex::Regex,
    replication_factory: u16,
    split: u16
}

pub struct DataPoint {
    time: time::Timespec,
    data: String
}

pub enum TimePeriod {
    Days(usize)
}

impl Default for Cluster {
    fn default() -> Cluster {
        Cluster {
            protocol: Protocol::Http,
            host: Host::Single(String::from_str("localhost"), 8086),
            username: None,
            password: None,
            hyper_client: hyper::Client::new(),
            request_timeout: None,
            failover_timeout: 60
        }
    }
}

impl Default for ShardSpace {
    fn default() -> ShardSpace {
        ShardSpace {
            name: String::from_str(""),
            retention_policy: TimePeriod::Days(60),
            shard_duration: TimePeriod::Days(14),
            regex: regex!(".*"),
            replication_factory: 1,
            split: 1
        }
    }
}

impl Cluster {
    fn new(protocol: Protocol, host: String, port: u16,
           username: String, password: String) -> Cluster {
        Cluster {
            protocol: protocol,
            host: Host::Single(host, port),
            username: Some(username),
            password: Some(password),
            ..Default::default()
        }
    }

    /// Private function to unwrap username/password
    // TODO work out how to move this check into compiletime
    #[inline]
    fn check_auth(&self) -> (&str, &str) {
        (
            self.username.as_ref().expect("Set username before firing request").as_slice(),
            self.password.as_ref().expect("Set password before firing request").as_slice()
        )
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
    fn set_request_timeout(&mut self, value: Option<u32>) -> Result<(), String> {
        unimplemented!();
    }

    /// Set failover timeout - default 60s
    fn set_failover_timeout(&mut self, value: u32) -> Result<(), String> {
        unimplemented!();
    }

    /// Get hosts available
    fn get_hosts_available(&self) -> Vec<Host> {
        unimplemented!();
    }

    /// Get hosts available
    fn get_hosts_disabled(&self) -> Vec<Host> {
        unimplemented!();
    }
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
