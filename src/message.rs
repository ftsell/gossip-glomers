use serde::{Deserialize, Serialize};
use uuid::Uuid;

type NodeId = String;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Message {
    #[serde(flatten)]
    pub header: MessageHeader,
    /// The payload of the message
    pub body: MessageBody,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessageHeader {
    /// A string identifying the node this message came from
    pub src: NodeId,
    /// A string identifying the node this message is to
    pub dest: NodeId,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessageBodyMetadata {
    /// A unique integer identifier
    pub msg_id: Option<usize>,
    /// For req/response, the msg_id of the request
    pub in_reply_to: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessageBody {
    #[serde(rename = "init")]
    Init {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
        #[serde(flatten)]
        content: InitMsg,
    },
    #[serde(rename = "init_ok")]
    InitResponse {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
    },
    #[serde(rename = "echo")]
    EchoRequest {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
        #[serde(flatten)]
        content: EchoRequest,
    },
    #[serde(rename = "echo_ok")]
    EchoResponse {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
        #[serde(flatten)]
        content: EchoResponse,
    },
    #[serde(rename = "generate")]
    UniqueIdRequest {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
    },
    #[serde(rename = "generate_ok")]
    UniqueIdResponse {
        #[serde(flatten)]
        metadata: MessageBodyMetadata,
        id: Uuid,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InitMsg {
    /// Indicates the ID of the node which is receiving this message
    pub node_id: NodeId,
    /// Lists all nodes in the cluster, including the recipient
    /// All nodes receive an identical list
    pub node_ids: Vec<NodeId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct EchoRequest {
    pub echo: String,
}

pub type EchoResponse = EchoRequest;
