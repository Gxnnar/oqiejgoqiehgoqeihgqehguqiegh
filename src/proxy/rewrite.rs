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

pub fn rewrite(body: &str, current_url: &Url) -> anyhow::Result<Vec<u8>> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut body.as_bytes())?;

    walk(&dom.document, &current_url);

    let mut bytes = Vec::new();
    let document: SerializableHandle = dom.document.clone().into();
    serialize::serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();

    Ok(bytes)
}

fn walk(handle: &Handle, current_url: &Url) {
    if let NodeData::Element { name, attrs, .. } = &handle.data {
        match name.local.as_ref() {
            "a" => rewrite_a(attrs, current_url),
            _ => {}
        }
    }

    for child in handle.children.borrow().iter() {
        walk(child, current_url);
    }
}

fn rewrite_a(attrs: &RefCell<Vec<Attribute>>, current_url: &Url) {
    let mut attrs = attrs.borrow_mut();
    let Some(href) = attrs.iter_mut().find(|i| i.name.local.eq("href")) else {
        return;
    };

    if let Ok(url) = current_url.join(&href.value) {
        href.value = format!("/p/{}", encoding::url::encode(url.as_str())).into();
    }
}
