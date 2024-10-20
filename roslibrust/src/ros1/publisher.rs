use crate::ros1::{
    names::Name,
    node::actor::NodeServerHandle,
    tcpros::{self, ConnectionHeader},
};
use abort_on_drop::ChildTask;
use log::*;
use roslibrust_codegen::RosMessageType;
use std::{
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc, RwLock},
};

/// The regular Publisher representation returned by calling advertise on a [crate::ros1::NodeHandle].
pub struct Publisher<T> {
    topic_name: String,
    sender: mpsc::Sender<Vec<u8>>,
    phantom: PhantomData<T>,
}

impl<T: RosMessageType> Publisher<T> {
    pub(crate) fn new(topic_name: &str, sender: mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            topic_name: topic_name.to_owned(),
            sender,
            phantom: PhantomData,
        }
    }

    /// Queues a message to be send on the related topic.
    /// Returns when the data has been queued not when data is actually sent.
    pub async fn publish(&self, data: &T) -> Result<(), PublisherError> {
        let data = roslibrust_serde_rosmsg::to_vec(&data)?;
        // TODO this is a pretty dumb...
        // because of the internal channel used for re-direction this future doesn't
        // actually complete when the data is sent, but merely when it is queued to be sent
        // This function could probably be non-async
        // Or we should do some significant re-work to have it only yield when the data is sent.
        self.sender
            .send(data)
            .await
            .map_err(|_| PublisherError::StreamClosed)?;
        debug!("Publishing data on topic {}", self.topic_name);
        Ok(())
    }
}

/// A specialty publisher used when message type is not known at compile time.
///
/// Relies on user to provide serialized data. Typically used with playback from bag files.
pub struct PublisherAny {
    topic_name: String,
    sender: mpsc::Sender<Vec<u8>>,
    phantom: PhantomData<Vec<u8>>,
}

impl PublisherAny {
    pub(crate) fn new(topic_name: &str, sender: mpsc::Sender<Vec<u8>>) -> Self {
        Self {
            topic_name: topic_name.to_owned(),
            sender,
            phantom: PhantomData,
        }
    }

    /// Queues a message to be send on the related topic.
    /// Returns when the data has been queued not when data is actually sent.
    ///
    /// This expects the data to be the raw bytes of the message body as they would appear going over the wire.
    /// See ros1_publish_any.rs example for more details.
    /// Body length should be included as first four bytes.
    pub async fn publish(&self, data: &Vec<u8>) -> Result<(), PublisherError> {
        // TODO this is a pretty dumb...
        // because of the internal channel used for re-direction this future doesn't
        // actually complete when the data is sent, but merely when it is queued to be sent
        // This function could probably be non-async
        // Or we should do some significant re-work to have it only yield when the data is sent.
        self.sender
            .send(data.to_vec())
            .await
            .map_err(|_| PublisherError::StreamClosed)?;
        debug!("Publishing data on topic {}", self.topic_name);
        Ok(())
    }
}

pub(crate) struct Publication {
    topic_type: String,
    listener_port: u16,
    _tcp_accept_task: ChildTask<()>,
    _publish_task: ChildTask<()>,
    publish_sender: mpsc::WeakSender<Vec<u8>>,
}

impl Publication {
    /// Spawns a new publication and sets up all tasks to run it
    /// Returns a handle to the publication and a mpsc::Sender to send messages to be published
    /// Dropping the Sender will (eventually) result in the publication being dropped and all tasks being canceled
    pub(crate) async fn new(
        node_name: &Name,
        latching: bool,
        topic_name: &str,
        host_addr: Ipv4Addr,
        queue_size: usize,
        msg_definition: &str,
        md5sum: &str,
        topic_type: &str,
        node_handle: NodeServerHandle,
    ) -> Result<(Self, mpsc::Sender<Vec<u8>>), std::io::Error> {
        // Get a socket for receiving connections on
        let host_addr = SocketAddr::from((host_addr, 0));
        let tcp_listener = tokio::net::TcpListener::bind(host_addr).await?;
        let listener_port = tcp_listener.local_addr().unwrap().port();

        // Setup the channel will will receive messages to be published on
        let (sender, receiver) = mpsc::channel::<Vec<u8>>(queue_size);

        // Setup the ROS connection header that we'll respond to all incomming connections with
        let responding_conn_header = ConnectionHeader {
            caller_id: node_name.to_string(),
            latching,
            msg_definition: msg_definition.to_owned(),
            md5sum: Some(md5sum.to_owned()),
            topic: Some(topic_name.to_owned()),
            topic_type: topic_type.to_owned(),
            tcp_nodelay: false,
            service: None,
        };
        trace!("Publisher connection header: {responding_conn_header:?}");

        // Setup storage for internal list of TCP streams
        let subscriber_streams = Arc::new(RwLock::new(Vec::new()));

        // Setup storage for the last message published (used for latching)
        let last_message = Arc::new(RwLock::new(None));

        // Create the task that will accept new TCP connections
        let subscriber_streams_copy = subscriber_streams.clone();
        let last_message_copy = last_message.clone();
        let topic_name_copy = topic_name.to_owned();
        let tcp_accept_handle = tokio::spawn(async move {
            Self::tcp_accept_task(
                tcp_listener,
                subscriber_streams_copy,
                topic_name_copy,
                responding_conn_header,
                last_message_copy,
            )
            .await
        });

        // Create the task that will handle publishing messages to all streams
        let topic_name_copy = topic_name.to_string();
        let publish_task = tokio::spawn(async move {
            Self::publish_task(
                receiver,
                subscriber_streams,
                last_message,
                node_handle,
                topic_name_copy,
            )
            .await
        });

        let sender_copy = sender.clone();
        Ok((
            Self {
                topic_type: topic_type.to_owned(),
                _tcp_accept_task: tcp_accept_handle.into(),
                listener_port,
                publish_sender: sender.downgrade(),
                _publish_task: publish_task.into(),
            },
            sender_copy,
        ))
    }

    // Note: this returns Option<> due to a timing edge case
    // There can be a delay between when the last sender is dropped and when the publication is dropped
    pub(crate) fn get_sender(&self) -> Option<mpsc::Sender<Vec<u8>>> {
        self.publish_sender.clone().upgrade()
    }

    pub(crate) fn port(&self) -> u16 {
        self.listener_port
    }

    pub(crate) fn topic_type(&self) -> &str {
        &self.topic_type
    }

    /// Wraps the functionality that the publish task will perform
    /// this task is spawned by new, and canceled when the Publication is dropped
    /// This task constantly pulls new messages from the main publish buffer and
    /// sends them to all of the TCP Streams that are connected to the topic.
    async fn publish_task(
        mut rx: mpsc::Receiver<Vec<u8>>,
        subscriber_streams: Arc<RwLock<Vec<tokio::net::TcpStream>>>,
        last_message: Arc<RwLock<Option<Vec<u8>>>>,
        node_handle: NodeServerHandle,
        topic: String,
    ) {
        debug!("Publish task has started for publication: {topic}");
        loop {
            match rx.recv().await {
                Some(msg_to_publish) => {
                    trace!("Publish task got message to publish for topic: {topic}");
                    let mut streams = subscriber_streams.write().await;
                    let mut streams_to_remove = vec![];
                    // TODO: we're awaiting in a for loop... Could parallelize here
                    for (stream_idx, stream) in streams.iter_mut().enumerate() {
                        if let Err(err) = stream.write_all(&msg_to_publish[..]).await {
                            // TODO: A single failure between nodes that cross host boundaries is probably normal, should make this more robust perhaps
                            debug!("Failed to send data to subscriber: {err}, removing");
                            streams_to_remove.push(stream_idx);
                        }
                    }
                    // Subtract the removed count to account for shifting indices after each
                    // remove, only works if they're sorted which should be the case given how
                    // it's being populated (forward enumeration)
                    streams_to_remove.into_iter().enumerate().for_each(
                        |(removed_cnt, stream_idx)| {
                            streams.remove(stream_idx - removed_cnt);
                        },
                    );

                    // Note: optimization possible here, we're storing the last message always, even if we're not latching
                    *last_message.write().await = Some(msg_to_publish);
                }
                None => {
                    debug!(
                        "No more senders for the publisher channel, triggering publication cleanup"
                    );
                    // All senders dropped, so we want to cleanup the Publication off of the node
                    // Tell the node server to dispose of this publication and unadvertise it
                    // Note: we need to do this in a spawned task or a drop-loop race condition will occur
                    // Dropping publication results in this task being dropped, which can end up canceling the future that is doing the dropping
                    // if we simpply .await here
                    let nh_copy = node_handle.clone();
                    let topic = topic.clone();
                    tokio::spawn(async move {
                        let _ = nh_copy.unregister_publisher(&topic).await;
                    });
                    break;
                }
            }
        }
        debug!("Publish task has exited for publication: {topic}");
    }

    /// Wraps the functionality that the tcp_accept task will perform
    /// This task is spawned by new, and canceled when the Publication is dropped
    /// This task constantly accepts new TCP connections and adds them to the list of streams to send data to.
    async fn tcp_accept_task(
        tcp_listener: tokio::net::TcpListener, // The TCP listener to accept connections on
        subscriber_streams: Arc<RwLock<Vec<tokio::net::TcpStream>>>, // Where accepted streams are stored
        topic_name: String,                                          // Only used for logging
        responding_conn_header: ConnectionHeader,                    // Header we respond with
        last_message: Arc<RwLock<Option<Vec<u8>>>>, // Last message published (used for latching)
    ) {
        debug!("TCP accept task has started for publication: {topic_name}");
        loop {
            if let Ok((mut stream, peer_addr)) = tcp_listener.accept().await {
                info!("Received connection from subscriber at {peer_addr} for topic {topic_name}");
                // Read the connection header:
                let connection_header = match tcpros::receive_header(&mut stream).await {
                    Ok(header) => header,
                    Err(e) => {
                        error!("Failed to read connection header: {e:?}");
                        stream
                            .shutdown()
                            .await
                            .expect("Unable to shutdown tcpstream");
                        continue;
                    }
                };

                debug!(
                    "Received subscribe request for {:?} with md5sum {:?}",
                    connection_header.topic, connection_header.md5sum
                );
                // I can't find documentation for this anywhere, but when using
                // `rostopic hz` with one of our publishers I discovered that the rospy code sent "*" as the md5sum
                // To indicate a "generic subscription"...
                // I also discovered that `rostopic echo` does not send a md5sum (even thou ros documentation says its required)
                if let Some(connection_md5sum) = connection_header.md5sum {
                    if connection_md5sum != "*" {
                        if let Some(local_md5sum) = &responding_conn_header.md5sum {
                            // TODO(lucasw) is it ok to match any with "*"?
                            // if local_md5sum != "*" && connection_md5sum != *local_md5sum {
                            if connection_md5sum != *local_md5sum {
                                warn!(
                                    "Got subscribe request for {}, but md5sums do not match. Expected {:?}, received {:?}",
                                    topic_name,
                                    local_md5sum,
                                    connection_md5sum,
                                    );
                                // Close the TCP connection
                                stream
                                    .shutdown()
                                    .await
                                    .expect("Unable to shutdown tcpstream");
                                continue;
                            }
                        }
                    }
                }
                // Write our own connection header in response
                let response_header_bytes = responding_conn_header
                    .to_bytes(false)
                    .expect("Couldn't serialize connection header");
                stream
                    .write_all(&response_header_bytes[..])
                    .await
                    .expect("Unable to respond on tcpstream");

                // If we're configured to latch, send the last message to the new subscriber
                if responding_conn_header.latching {
                    if let Some(last_message) = last_message.read().await.as_ref() {
                        debug!(
                            "Publication configured to be latching and has last_message, sending"
                        );
                        let res = stream.write_all(last_message).await;
                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to send latch message to subscriber: {e:?}");
                                // Note doing any handling here, TCP stream will be cleaned up during
                                // next regular publish in the publish task
                            }
                        }
                    }
                }

                let mut wlock = subscriber_streams.write().await;
                wlock.push(stream);
                debug!(
                    "Added stream for topic {:?} to subscriber {}",
                    connection_header.topic, peer_addr
                );
            }
        }
    }
}

impl Drop for Publication {
    fn drop(&mut self) {
        debug!("Dropping publication for topic {}", self.topic_type);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PublisherError {
    /// Serialize Error from `serde_rosmsg::Error` (stored as String because of dyn Error)
    #[error("serde_rosmsg Error: {0}")]
    SerializingError(String),
    #[error("connection closed, no further messages can be sent")]
    StreamClosed,
}

impl From<roslibrust_serde_rosmsg::Error> for PublisherError {
    fn from(value: roslibrust_serde_rosmsg::Error) -> Self {
        Self::SerializingError(value.to_string())
    }
}
