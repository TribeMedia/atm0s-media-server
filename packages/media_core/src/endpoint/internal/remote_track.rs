//! RemoteTrack take care about publish local media to sdn, and react with feedback from consumers

use std::{collections::VecDeque, time::Instant};

use media_server_protocol::{
    endpoint::{BitrateControlMode, TrackMeta, TrackName, TrackPriority},
    media::MediaKind,
};
use sans_io_runtime::Task;

use crate::{
    cluster::{ClusterRemoteTrackControl, ClusterRemoteTrackEvent, ClusterRoomHash},
    endpoint::{EndpointRemoteTrackEvent, EndpointRemoteTrackReq, EndpointRemoteTrackRes, EndpointReqId},
    transport::RemoteTrackEvent,
};

use super::bitrate_allocator::IngressAction;

pub enum Input {
    JoinRoom(ClusterRoomHash),
    LeaveRoom,
    Cluster(ClusterRemoteTrackEvent),
    Event(RemoteTrackEvent),
    RpcReq(EndpointReqId, EndpointRemoteTrackReq),
    BitrateAllocation(IngressAction),
}

pub enum Output {
    Event(EndpointRemoteTrackEvent),
    Cluster(ClusterRoomHash, ClusterRemoteTrackControl),
    RpcRes(EndpointReqId, EndpointRemoteTrackRes),
    Started(MediaKind, TrackPriority),
    Stopped(MediaKind),
}

pub struct EndpointRemoteTrack {
    meta: TrackMeta,
    room: Option<ClusterRoomHash>,
    name: Option<String>,
    queue: VecDeque<Output>,
    allocate_bitrate: Option<u64>,
}

impl EndpointRemoteTrack {
    pub fn new(room: Option<ClusterRoomHash>, meta: TrackMeta) -> Self {
        Self {
            meta,
            room,
            name: None,
            queue: VecDeque::new(),
            allocate_bitrate: None,
        }
    }

    fn on_join_room(&mut self, _now: Instant, room: ClusterRoomHash) -> Option<Output> {
        assert_eq!(self.room, None);
        self.room = Some(room);
        log::info!("[EndpointRemoteTrack] join room {room}");
        let name = self.name.clone()?;
        log::info!("[EndpointRemoteTrack] started as name {name} after join room");
        Some(Output::Cluster(room, ClusterRemoteTrackControl::Started(TrackName(name), self.meta.clone())))
    }
    fn on_leave_room(&mut self, _now: Instant) -> Option<Output> {
        let room = self.room.take().expect("Must have room here");
        log::info!("[EndpointRemoteTrack] leave room {room}");
        let name = self.name.as_ref()?;
        log::info!("[EndpointRemoteTrack] stopped as name {name} after leave room");
        Some(Output::Cluster(room, ClusterRemoteTrackControl::Ended))
    }

    fn on_cluster_event(&mut self, _now: Instant, event: ClusterRemoteTrackEvent) -> Option<Output> {
        match event {
            ClusterRemoteTrackEvent::RequestKeyFrame => Some(Output::Event(EndpointRemoteTrackEvent::RequestKeyFrame)),
            ClusterRemoteTrackEvent::LimitBitrate { min, max: _ } => {
                match self.meta.control {
                    BitrateControlMode::MaxBitrate | BitrateControlMode::NonControl => None,
                    BitrateControlMode::DynamicConsumers => {
                        //TODO dynamic with type of scaling
                        let bitrate = min.min(self.allocate_bitrate?);
                        Some(Output::Event(EndpointRemoteTrackEvent::LimitBitrateBps(bitrate)))
                    }
                }
            }
        }
    }

    fn on_transport_event(&mut self, _now: Instant, event: RemoteTrackEvent) -> Option<Output> {
        match event {
            RemoteTrackEvent::Started { name, priority, meta: _ } => {
                self.name = Some(name.clone());
                let room = self.room.as_ref()?;
                log::info!("[EndpointRemoteTrack] started as name {name}");
                self.queue.push_back(Output::Started(self.meta.kind, priority));
                Some(Output::Cluster(*room, ClusterRemoteTrackControl::Started(TrackName(name), self.meta.clone())))
            }
            RemoteTrackEvent::Paused => None,
            RemoteTrackEvent::Resumed => None,
            RemoteTrackEvent::Media(media) => {
                let room = self.room.as_ref()?;
                Some(Output::Cluster(*room, ClusterRemoteTrackControl::Media(media)))
            }
            RemoteTrackEvent::Ended => {
                let name = self.name.take()?;
                let room = self.room.as_ref()?;
                log::info!("[EndpointRemoteTrack] stopped with name {name}");
                self.queue.push_back(Output::Stopped(self.meta.kind));
                Some(Output::Cluster(*room, ClusterRemoteTrackControl::Ended))
            }
        }
    }

    fn on_rpc_req(&mut self, _now: Instant, _req_id: EndpointReqId, _req: EndpointRemoteTrackReq) -> Option<Output> {
        None
    }

    fn on_bitrate_allocation_action(&mut self, _now: Instant, action: IngressAction) -> Option<Output> {
        match action {
            IngressAction::SetBitrate(bitrate) => match self.meta.control {
                BitrateControlMode::MaxBitrate => Some(Output::Event(EndpointRemoteTrackEvent::LimitBitrateBps(bitrate))),
                BitrateControlMode::DynamicConsumers | BitrateControlMode::NonControl => None,
            },
        }
    }
}

impl Task<Input, Output> for EndpointRemoteTrack {
    fn on_tick(&mut self, _now: Instant) -> Option<Output> {
        None
    }

    fn on_event(&mut self, now: Instant, input: Input) -> Option<Output> {
        match input {
            Input::JoinRoom(room) => self.on_join_room(now, room),
            Input::LeaveRoom => self.on_leave_room(now),
            Input::Cluster(event) => self.on_cluster_event(now, event),
            Input::Event(event) => self.on_transport_event(now, event),
            Input::RpcReq(req_id, req) => self.on_rpc_req(now, req_id, req),
            Input::BitrateAllocation(action) => self.on_bitrate_allocation_action(now, action),
        }
    }

    fn pop_output(&mut self, _now: Instant) -> Option<Output> {
        self.queue.pop_front()
    }

    fn shutdown(&mut self, _now: Instant) -> Option<Output> {
        None
    }
}

#[cfg(test)]
mod tests {
    //TODO start in room
    //TODO start not in room
    //TODO stop in room
    //TODO stop not in room
    //TODO switched room need fire events
    //TODO send media to cluster
    //TODO handle key-frame-request feedback
    //TODO handle bitrate limit feedback
}
