use std::collections::HashMap;

use apollo_metrics::metrics::{MetricCounter, MetricGauge};
use libp2p::gossipsub::TopicHash;

pub struct BroadcastNetworkMetrics {
    pub num_sent_broadcast_messages: MetricCounter,
    pub num_received_broadcast_messages: MetricCounter,
}

impl BroadcastNetworkMetrics {
    pub fn register(&self) {
        self.num_sent_broadcast_messages.register();
        self.num_received_broadcast_messages.register();
    }
}

pub struct SqmrNetworkMetrics {
    pub num_active_inbound_sessions: MetricGauge,
    pub num_active_outbound_sessions: MetricGauge,
}

impl SqmrNetworkMetrics {
    pub fn register(&self) {
        self.num_active_inbound_sessions.register();
        self.num_active_inbound_sessions.set(0f64);
        self.num_active_outbound_sessions.register();
        self.num_active_outbound_sessions.set(0f64);
    }
}

pub struct GossipsubMetrics {
    pub num_messages_received: MetricCounter,
    pub num_messages_published: MetricCounter,
    pub num_subscriptions: MetricCounter,
    pub num_unsubscriptions: MetricCounter,
    pub num_peer_subscribed: MetricCounter,
    pub num_peer_unsubscribed: MetricCounter,
    pub num_gossipsub_not_supported: MetricCounter,
    pub num_slow_peers: MetricCounter,
    pub num_peer_added: MetricCounter,
    pub num_peer_removed: MetricCounter,
}

impl GossipsubMetrics {
    pub fn register(&self) {
        self.num_messages_received.register();
        self.num_messages_published.register();
        self.num_subscriptions.register();
        self.num_unsubscriptions.register();
        self.num_peer_subscribed.register();
        self.num_peer_unsubscribed.register();
        self.num_gossipsub_not_supported.register();
        self.num_slow_peers.register();
        self.num_peer_added.register();
        self.num_peer_removed.register();
    }
}

// TODO(alonl, shahak): Consider making these fields private and receive Topics instead of
// TopicHashes in the constructor
pub struct NetworkMetrics {
    pub num_connected_peers: MetricGauge,
    pub num_blacklisted_peers: MetricGauge,
    pub broadcast_metrics_by_topic: Option<HashMap<TopicHash, BroadcastNetworkMetrics>>,
    pub sqmr_metrics: Option<SqmrNetworkMetrics>,
    pub gossipsub_metrics: Option<GossipsubMetrics>,
}

impl NetworkMetrics {
    pub fn register(&self) {
        self.num_connected_peers.register();
        self.num_connected_peers.set(0f64);
        self.num_blacklisted_peers.register();
        self.num_blacklisted_peers.set(0f64);
        if let Some(broadcast_metrics_by_topic) = self.broadcast_metrics_by_topic.as_ref() {
            for broadcast_metrics in broadcast_metrics_by_topic.values() {
                broadcast_metrics.register();
            }
        }
        if let Some(sqmr_metrics) = self.sqmr_metrics.as_ref() {
            sqmr_metrics.register();
        }
        if let Some(gossipsub_metrics) = self.gossipsub_metrics.as_ref() {
            gossipsub_metrics.register();
        }
    }
}
