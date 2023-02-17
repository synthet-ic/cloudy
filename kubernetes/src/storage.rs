pub mod storage_class;
pub mod csi_driver;
pub mod csi_node;
pub mod csi_storage_capacity;
pub mod volume_attachment;

// use kfl::Decode;

pub use storage_class::StorageClass;
pub use csi_driver::CsiDriver;
pub use csi_node::CsiNode;
pub use csi_storage_capacity::CsiStorageCapacity;
pub use volume_attachment::VolumeAttachment;

// #[derive(Debug, Decode)]
// #[kfl(tag = "kind")]
// pub enum Storage {
//     StorageClass(StorageClass),
//     CsiDriver(CsiDriver),
//     CsiNode(CsiNode),
//     CsiStorageCapacity(CsiStorageCapacity),
//     VolumeAttachment(VolumeAttachment)
// }
