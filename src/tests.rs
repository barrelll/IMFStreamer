#![cfg(test)]

#[test]
fn file_stream() {
    use std::{fs::File, path::Path};
    use AdamTree;
    let cargoman = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(cargoman)
        .join("..")
        .join("..")
        .join("download.tsi.telecom-paristech.fr")
        .join("gpac")
        .join("MPEG")
        .join("ISOBMFF-Conformance")
        .join("isobmff")
        .join("14_large.mp4");
    let mut handle = File::open(path).expect("Error opening file");
    let mut atree = AdamTree::new(&handle);
    let moov = atree.search_tree("moov.trak1.tkhd");
}
