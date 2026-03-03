use cardwire_core::gpu::GpuRow;
pub fn print_gpu_table(rows: &[GpuRow]) {
    let mut id_w = 2usize;
    let mut name_w = 4usize;
    let mut pci_w = 3usize;
    let mut render_w = 6usize;

    for (id, name, pci, render, _, _) in rows {
        id_w = id_w.max(id.to_string().len());
        name_w = name_w.max(name.len());
        pci_w = pci_w.max(pci.len());
        render_w = render_w.max(render.len());
    }

    println!(
        "{:<id_w$}  {:<name_w$}  {:<pci_w$}  {:<render_w$}  {:<7}  {:<7}",
        "ID",
        "NAME",
        "PCI",
        "RENDER",
        "DEFAULT",
        "BLOCKED",
        id_w = id_w,
        name_w = name_w,
        pci_w = pci_w,
        render_w = render_w,
    );
    println!(
        "{}  {}  {}  {}  {}  {}",
        "-".repeat(id_w),
        "-".repeat(name_w),
        "-".repeat(pci_w),
        "-".repeat(render_w),
        "-".repeat(7),
        "-".repeat(7),
    );

    for (id, name, pci, render, is_default, blocked) in rows {
        println!(
            "{:<id_w$}  {:<name_w$}  {:<pci_w$}  {:<render_w$}  {:<7}  {:<7}",
            id,
            name,
            pci,
            render,
            if *is_default { "yes" } else { "no" },
            if *blocked { "on*" } else { "off" },
            id_w = id_w,
            name_w = name_w,
            pci_w = pci_w,
            render_w = render_w,
        );
    }
}
