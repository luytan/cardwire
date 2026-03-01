#[derive(Clone)]
pub struct Gpu {
    pub id: usize,
    pub name: String,
    pub pci: String,
    pub render: String,
    pub card: String,
    pub default: bool,
}

impl Gpu {
    pub fn pci_address(&self) -> &str {
        &self.pci
    }

    pub fn is_default(&self) -> bool {
        self.default
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn render_node(&self) -> &str {
        &self.render
    }

    pub fn card_node(&self) -> &str {
        &self.card
    }

}