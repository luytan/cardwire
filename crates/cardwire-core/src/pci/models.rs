#[derive(Clone)]
pub struct PciDevice {
    pub pci_address: String,
    pub iommu_group: Option<usize>,
    pub vendor_id: String,
    pub device_id: String,
    pub vendor_name: String,
    pub device_name: String,
    pub driver: String,
    pub class: String,
}

pub struct IommuGroup {
    pub id: usize,
    pub devices: Vec<String>,
}
