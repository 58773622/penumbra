pub mod emmc;
pub mod gpt;
pub mod ufs;

pub use emmc::EmmcPartition;
pub use gpt::parse_gpt;
pub use ufs::UfsPartition;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Unknown = 0,
    Emmc = 0x1,
    Ufs = 0x30,
}

#[derive(Debug, Clone, Copy)]
pub enum PartitionKind {
    Emmc(EmmcPartition),
    Ufs(UfsPartition),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub size: usize,
    pub address: u64,
    pub kind: PartitionKind,
}

impl Partition {
    pub fn new(name: &str, size: usize, address: u64, kind: PartitionKind) -> Self {
        Self { name: name.to_string(), size, address, kind }
    }
}

impl PartitionKind {
    pub fn as_u32(&self) -> u32 {
        match self {
            PartitionKind::Emmc(part) => *part as u32,
            PartitionKind::Ufs(part) => *part as u32,
            PartitionKind::Unknown => 0,
        }
    }
}

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    fn kind(&self) -> StorageType;
    fn block_size(&self) -> u32;
    fn total_size(&self) -> u64;

    fn get_user_part(&self) -> PartitionKind;
    fn get_pl_part1(&self) -> PartitionKind;
    fn get_pl_part2(&self) -> PartitionKind;
}
