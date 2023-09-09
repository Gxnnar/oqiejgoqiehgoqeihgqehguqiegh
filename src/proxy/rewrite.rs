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

use crate::misc::tld;

const SUPPORTED_URL_SCHEMES: &[&str] = &["http", "https"];
const UNSUPPORTED_TLD: &[&str] = &["onion", "i2p"];

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
    for href in attrs
        .iter_mut()
        .filter(|i| ADDRESS_ATTRS.contains(&i.name.local.as_bytes()))
    {
        if href.value.starts_with('#') {
            continue;
        }

        let Ok(url) = current_url.join(&href.value) else {
            continue;
        };

        if !SUPPORTED_URL_SCHEMES.contains(&url.scheme())
            || UNSUPPORTED_TLD.contains(&tld(&url).unwrap_or_default().as_str())
        {
            continue;
        }

        href.value = format!("/~/{}", encoding::url::encode(url.as_str())).into();
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
    for rel in attrs
        .iter_mut()
        .filter(|i| i.name.local.as_bytes() == b"rel")
    {
        let values = rel
            .value
            .split_whitespace()
            .filter(|i| !DISALLOWED_LINK_REL.contains(&i.as_bytes()))
            .collect::<Vec<_>>()
            .join(" ");
        rel.value = values.into();
    }
}
