#[derive(Debug)]
pub(crate) struct PartitionTableSubType {
    pub ty: &'static str,
    pub subtype: &'static str,
    pub name: &'static str,
}

impl PartitionTableSubType {
    const fn new(ty: &'static str, subtype: &'static str, name: &'static str) -> Self {
        //TODO: wrap name with gettext call
        Self { ty, subtype, name }
    }
}

/// Known [PartitionTableSubType]s.
pub(crate) const PARTITION_TABLE_SUBTYPES: [PartitionTableSubType; 11] = [
    //Translators: name of partition table format
    PartitionTableSubType::new("dos", "generic", "Generic"),
    PartitionTableSubType::new("dos", "linux", "Linux"),
    PartitionTableSubType::new("dos", "microsoft", "Windows"),
    PartitionTableSubType::new("dos", "other", "Other"),
    //
    PartitionTableSubType::new("gpt", "generic", "Generic"),
    PartitionTableSubType::new("gpt", "linux", "Linux"),
    PartitionTableSubType::new("gpt", "microsoft", "Windows"),
    PartitionTableSubType::new("gpt", "apple", "Mac OS X"),
    PartitionTableSubType::new("gpt", "other", "Other"),
    //
    PartitionTableSubType::new("apm", "apple", "Mac OS X"),
    PartitionTableSubType::new("apm", "microsoft", "Windows"),
];
