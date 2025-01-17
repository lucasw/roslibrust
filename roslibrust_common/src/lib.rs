//! # roslibrust_common
//! This crate provides common types and traits used throughout the roslibrust ecosystem.

/// The central error type used throughout roslibrust.
///
/// For now all roslibrust backends must coerce their errors into this type.
/// We may in the future allow backends to define their own error types, for now this is a compromise.
///
/// Additionally, this error type is returned from all roslibrust function calls so failure types must be relatively generic.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Is returned when communication is fully lost.
    /// While this error is being returned messages should be assumed to be being lost.
    /// Backends are expected to be "self-healing" and when connection is restored existing Publishers, Subscribers, etc.
    /// should resume functionality without needing to be recreated.
    #[error("No connection to ROS backend")]
    Disconnected,
    /// Some backends aren't able to conclusively determine if an operation has failed.
    /// Timeout will be returned if an operation takes a unexpectedly long time.
    /// For the `rosbridge` backend where this is most frequently encountered the timeout is configurable on the client.
    #[error("Operation timed out: {0}")]
    Timeout(String),
    /// When a message is received but the backend is unable to serialize/deserialize it to the Rust type representing the message type.
    ///
    /// This error is also returned in the event of an md5sum mismatch.
    #[error("Serialization error: {0}")]
    SerializationError(String),
    /// When the backend "server" reports an error this type is returned.
    ///
    /// This can happen when there are internal issues on the rosbridge_server, or with xmlrpc communication with the ros1 master.
    #[error("Rosbridge server reported an error: {0}")]
    ServerError(String),
    /// Returned when there is a fundamental networking error.
    ///
    /// Typically reserved for situations when ports are unavailable, dns lookups fail, etc.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    /// When a topic name is used that isn't a valid topic name.
    #[error("Name does not meet ROS requirements: {0}")]
    InvalidName(String),
    /// Backends are free to return this error if they encounter any error that doesn't cleanly fit in the other categories.
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

/// Generic result type used throughout roslibrust.
pub type Result<T> = std::result::Result<T, Error>;

/// Fundamental traits for message types this crate works with
/// This trait will be satisfied for any types generated with this crate's message_gen functionality
pub trait RosMessageType:
    'static + serde::de::DeserializeOwned + Send + serde::Serialize + Sync + Clone + std::fmt::Debug
{
    /// Expected to be the combination pkg_name/type_name string describing the type to ros
    /// Example: std_msgs/Header
    const ROS_TYPE_NAME: &'static str;

    /// The computed md5sum of the message file and its dependencies
    /// This field is optional, and only needed when using ros1 native communication
    const MD5SUM: &'static str = "";

    /// The definition from the msg, srv, or action file
    /// This field is optional, and only needed when using ros1 native communication
    const DEFINITION: &'static str = "";
}

// This special impl allows for services with no args / returns
impl RosMessageType for () {
    const ROS_TYPE_NAME: &'static str = "";
    const MD5SUM: &'static str = "";
    const DEFINITION: &'static str = "";
}

/// Represents a ROS service type definition corresponding to a `.srv` file.
///
/// Typically this trait will not be implemented by hand but instead be generated by using [roslibrust's codegen functionality][<https://docs.rs/roslibrust/latest/roslibrust/codegen>].
/// This trait is used by the [ServiceProvider] trait to define types that can be used with [ServiceProvider::call_service] and [ServiceProvider::advertise_service]
pub trait RosServiceType: 'static + Send + Sync {
    /// Name of the ros service e.g. `rospy_tutorials/AddTwoInts`
    const ROS_SERVICE_NAME: &'static str;
    /// The computed md5sum of the message file and its dependencies
    const MD5SUM: &'static str;
    /// The type of data being sent in the request
    type Request: RosMessageType;
    /// The type of the data
    type Response: RosMessageType;
}

// Note: service Fn is currently defined here as it used by ros1 and roslibrust impls
/// This trait describes a function which can validly act as a ROS service
/// server with roslibrust. We're really just using this as a trait alias
/// as the full definition is overly verbose and trait aliases are unstable.
pub trait ServiceFn<T: RosServiceType>:
    Fn(
        T::Request,
    ) -> std::result::Result<T::Response, Box<dyn std::error::Error + 'static + Send + Sync>>
    + Send
    + Sync
    + 'static
{
}

/// Automatic implementation of ServiceFn for Fn
impl<T, F> ServiceFn<T> for F
where
    T: RosServiceType,
    F: Fn(
            T::Request,
        )
            -> std::result::Result<T::Response, Box<dyn std::error::Error + 'static + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
}

/// A generic message type used by some implementations to provide a generic subscriber / publisher without serialization
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Debug, Default, Clone, PartialEq)]
pub struct ShapeShifter(Vec<u8>);

// The equivalent of rospy AnyMsg or C++ ShapeShifter, subscribe_any() uses this type
impl RosMessageType for ShapeShifter {
    const ROS_TYPE_NAME: &'static str = "*";
    const MD5SUM: &'static str = "*";
    const DEFINITION: &'static str = "";
}

/// Contains functions for calculating md5sums of message definitions
/// These functions are needed both in roslibrust_ros1 and roslibrust_codegen so they're in this crate
pub mod md5sum;

/// Contains the generic traits represent a pubsub system and service system
/// These traits will be implemented for specific backends to provides access to "ROS Like" functionality
pub mod traits;
pub use traits::*; // Bring topic provider traits into root namespace
