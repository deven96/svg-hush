use svg_hush::*;

// Minimal SVG with a C2PA manifest embedded the same way c2patool does it:
// <metadata><c2pa:manifest>...</c2pa:manifest><?xpacket ...?>XMP<?xpacket end="w"?></metadata>
const C2PA_SVG: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10">
  <metadata>
    <c2pa:manifest xmlns:c2pa="http://c2pa.org/manifest">JUMBFDATA</c2pa:manifest>
    <?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
    <x:xmpmeta xmlns:x="adobe:ns:meta/">
      <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
        <rdf:Description rdf:about="" xmlns:xmpMM="http://ns.adobe.com/xap/1.0/mm/"
          xmpMM:InstanceID="xmp.iid:abc123"/>
      </rdf:RDF>
    </x:xmpmeta>
    <?xpacket end="w"?>
  </metadata>
  <rect width="10" height="10"/>
</svg>"#;

#[test]
fn whole_file() {
    let test = std::fs::read("tests/test.xml").unwrap();
    let expected = std::fs::read_to_string("tests/filtered.xml").unwrap();
    let mut f = Filter::new();
    f.set_data_url_filter(data_url_filter::allow_standard_images);
    let mut out = Vec::new();
    f.filter(&mut test.as_slice(), &mut out).unwrap();
    // cargo run -- tests/test.xml  > tests/filtered.xml
    assert_eq!(std::str::from_utf8(&out).unwrap(), expected);
}

#[test]
fn ns() {
    let svg = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg" xmlns:vector="http://www.w3.org/2000/svg">
        <rect height="300" width="300"/>
        <svg:rect height="200" width="200">
            <title>Test</title>
        </svg:rect>
        <vector:rect height="100" width="100"/>
        <svg:text xml:space="preserve">  Hallo World  </svg:text>
    </svg>
    "##;

    let f = Filter::new();
    let mut out = Vec::new();
    let mut out2 = Vec::new();
    f.filter(&mut svg.as_bytes(), &mut out).unwrap();
    f.filter(&mut out.as_slice(), &mut out2).unwrap();
    assert_eq!(&out, &out2);
}

#[test]
fn script_in_metadata_still_dropped_with_preserve_c2pa() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
      <metadata>
        <script>alert(1)</script>
        <c2pa:manifest xmlns:c2pa="http://c2pa.org/manifest">JUMBF</c2pa:manifest>
      </metadata>
      <rect width="10" height="10"/>
    </svg>"#;
    let mut f = Filter::new();
    f.set_preserve_c2pa(true);
    let mut out = Vec::new();
    f.filter(&mut svg.as_bytes(), &mut out).unwrap();
    let out_str = std::str::from_utf8(&out).unwrap();
    assert!(!out_str.contains("alert(1)"), "script survived with preserve_c2pa");
    assert!(out_str.contains("JUMBF"), "c2pa:manifest was dropped");
}

#[test]
fn c2pa_stripped_by_default() {
    let f = Filter::new();
    let mut out = Vec::new();
    f.filter(&mut C2PA_SVG.as_bytes(), &mut out).unwrap();
    let out_str = std::str::from_utf8(&out).unwrap();
    assert!(!out_str.contains("c2pa:manifest"));
    assert!(!out_str.contains("JUMBFDATA"));
}

#[test]
fn c2pa_metadata_preserved() {
    let mut f = Filter::new();
    f.set_preserve_c2pa(true);
    let mut out = Vec::new();
    f.filter(&mut C2PA_SVG.as_bytes(), &mut out).unwrap();
    let out_str = std::str::from_utf8(&out).unwrap();
    assert!(out_str.contains("c2pa:manifest"), "c2pa:manifest was stripped");
    assert!(out_str.contains("JUMBFDATA"), "c2pa manifest content was stripped");
    assert!(out_str.contains("xpacket"), "xpacket PI was stripped");
    assert!(out_str.contains("xmpMM:InstanceID"), "XMP body was stripped");
}

#[test]
fn c2pa_real_fixture_preserved() {
    let svg = std::fs::read("tests/sample1_c2pa.svg").unwrap();
    let mut f = Filter::new();
    f.set_preserve_c2pa(true);
    let mut out = Vec::new();
    f.filter(&mut svg.as_slice(), &mut out).unwrap();
    let out_str = std::str::from_utf8(&out).unwrap();
    assert!(
        out_str.contains("c2pa:manifest"),
        "c2pa:manifest was stripped from real fixture"
    );
    assert!(
        out_str.contains("xpacket"),
        "XMP xpacket was stripped from real fixture"
    );
}
