//! Modify HTML responses to fix links and other things.

use std::cell::RefCell;

use afire::internal::encoding;
use html5ever::{
    driver::parse_document,
    serialize::{self, SerializeOpts},
    tendril::TendrilSink,
    Attribute,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use url::Url;

const SUPPORTED_URL_SCHEMES: &[&str] = &["http", "https"];

pub fn rewrite(body: &str, current_url: &Url) -> anyhow::Result<Vec<u8>> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut body.as_bytes())?;

    walk(&dom.document, current_url);

    let mut bytes = Vec::new();
    let document: SerializableHandle = dom.document.into();
    serialize::serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();

    Ok(bytes)
}

fn walk(handle: &Handle, current_url: &Url) {
    for child in handle.children.borrow().iter() {
        walk(child, current_url);
    }

    if let NodeData::Element { name, attrs, .. } = &handle.data {
        rewrite_all(attrs, current_url);

        match name.local.as_bytes() {
            b"link" => rewrite_link(attrs),
            _ => {}
        }
    }
}

const ADDRESS_ATTRS: &[&[u8]] = &[b"src", b"href", b"srcset", b"action"];

fn rewrite_all(attrs: &RefCell<Vec<Attribute>>, current_url: &Url) {
    let mut attrs = attrs.borrow_mut();
    let Some(href) = attrs.iter_mut().find(|i| ADDRESS_ATTRS.contains(&i.name.local.as_bytes())) else {
            return;
        };

    if href.value.starts_with('#') {
        return;
    }

    if let Ok(url) = current_url.join(&href.value) {
        if !SUPPORTED_URL_SCHEMES.contains(&url.scheme()) {
            return;
        }

        href.value = format!("/p/{}", encoding::url::encode(url.as_str())).into();
    }
}

const DISALLOWED_LINK_REL: &[&[u8]] = &[
    b"dns-prefetch",
    b"modulepreload",
    b"next",
    b"noreferrer",
    b"noopener",
    b"preconnect",
    b"prefetch",
    b"preload",
    b"prerender",
    b"prev",
];

fn rewrite_link(attrs: &RefCell<Vec<Attribute>>) {
    let mut attrs = attrs.borrow_mut();
    let Some(rel) = attrs.iter_mut().find(|i| i.name.local.as_bytes() == b"rel") else {
        return;
    };

    let values = rel
        .value
        .split_whitespace()
        .filter(|i| !DISALLOWED_LINK_REL.contains(&i.as_bytes()))
        .collect::<Vec<_>>()
        .join(" ");
    rel.value = values.into();
}
