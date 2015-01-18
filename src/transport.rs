use std::fmt;
use url::Host;

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

/// A cluster takes requests for operations and performs them
/// on a cluster of influxdb instances, transparently handling
/// replication/load balancing
pub struct Cluster {
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

    /// Add a new host to available hosts
    fn add_host(&mut self, new_host: Instance) {
        self.instances_available.lock().unwrap().push(new_host);
    }

    /// Get an instance if any are available, or None if not
    fn get_instance(&mut self) -> Option<Instance> {
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

    /// Enable an instance by moving it to the instances_available vector
    fn enable_instance(&mut self, pos: usize) {
        let host = self.instances_disabled.lock().unwrap().remove(pos);
        self.instances_available.lock().unwrap().push(host);
    }

    /// Disable an instance, and schedule it for reenabling after failover_timeout
    fn disable_instance(&mut self, pos: usize) {
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
    /// other hosts will have been removed/added since) in disable_instance
    fn remove_instance_by_attrs(instances: Arc<Mutex<Vec<Instance>>>,
                                instance: Instance) {
        let mut instances = instances.lock().unwrap();
        instances.retain(|ref x| **x != instance);
    }

    /// Creates a url for a request
    fn build_url(&self,
                 instance: Instance,
                 path: Vec<String>,
                 query: Vec<(String, String)>) -> Url {
        Url {
            // bit before ://
            scheme: format!("{}", instance.scheme),
            // bit after :// before ?
            scheme_data: SchemeData::Relative(RelativeSchemeData {
                username: String::from_str(""),
                password: None,
                host: instance.host.clone(),
                port: Some(instance.port),
                default_port: Some(instance.port), // TODO what to do here?
                path: path
            }),
            // Bit after ? before #
            query: Some(::url::form_urlencoded::serialize_owned(query.as_slice())),
            // Bit after #
            fragment: None
        }
    }

    /// Builds a request and sends it - returning the request status, which can be
    /// queried like a future
    fn request(&mut self,
                     method: Method,
                     path: Vec<String>,
                     query: Option<Vec<(String, String)>>) -> Arc<RwLock<RequestStatus<(), ()>>> {
        // TODO handle no hosts more gracefully
        let instance = self.get_instance().expect("No hosts left");
        let mut query = match query {
            Some(vec) => {
                let mut vec = vec.clone();
                vec.reserve(2);
                vec
            },
            None => {Vec::with_capacity(2)}
        };
        query.push((String::from_str("u"), self.username.clone()));
        query.push((String::from_str("p"), self.password.clone()));
        let url = self.build_url(instance, path, query);
        let mut builder = self.hyper_client.request(method, url);
        let response = Arc::new(RwLock::new(RequestStatus::new()));
        let moved_response = response.clone();

        Thread::spawn(move || {
             
        });
        response
    }
}

