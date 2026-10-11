//! The little XML the S3 API needs: a handful of flat elements read, three small documents written.

/// The text of the first `<tag>..</tag>`, unescaped.
pub(crate) fn element(xml: &str, tag: &str) -> Option<String> {
    blocks(xml, tag).next().map(unescape)
}

/// The inner text of every `<tag>..</tag>`, in order, as written.
pub(crate) fn blocks<'a>(xml: &'a str, tag: &str) -> impl Iterator<Item = &'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut rest = xml;
    std::iter::from_fn(move || {
        let start = rest.find(&open)? + open.len();
        let end = start + rest[start..].find(&close)?;
        let inner = &rest[start..end];
        rest = &rest[end + close.len()..];
        Some(inner)
    })
}

pub(crate) fn unescape(text: &str) -> String {
    if !text.contains('&') {
        return text.to_owned();
    }
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#34;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING: &str = "<ListVersionsResult xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">\
        <Name>akr-evidence</Name><IsTruncated>false</IsTruncated>\
        <Version><Key>a&amp;b.txt</Key><VersionId>v2</VersionId><IsLatest>true</IsLatest><Size>5</Size></Version>\
        <Version><Key>a&amp;b.txt</Key><VersionId>v1</VersionId><IsLatest>false</IsLatest><Size>3</Size></Version>\
        <DeleteMarker><Key>gone</Key><VersionId>v9</VersionId><IsLatest>true</IsLatest></DeleteMarker>\
        </ListVersionsResult>";

    #[test]
    fn the_first_element_is_read_unescaped() {
        assert_eq!(element(LISTING, "Name").as_deref(), Some("akr-evidence"));
        assert_eq!(element(LISTING, "Key").as_deref(), Some("a&b.txt"));
        assert_eq!(element(LISTING, "NextKeyMarker"), None);
    }

    #[test]
    fn every_block_is_found_in_order() {
        let versions: Vec<_> = blocks(LISTING, "Version")
            .map(|block| element(block, "VersionId"))
            .collect();
        assert_eq!(versions, [Some("v2".to_owned()), Some("v1".to_owned())]);
        assert_eq!(blocks(LISTING, "DeleteMarker").count(), 1);
    }

    #[test]
    fn an_escaped_ampersand_is_not_unescaped_twice() {
        assert_eq!(unescape("a&amp;lt;b"), "a&lt;b");
        assert_eq!(unescape("&lt;&gt;&quot;&apos;"), "<>\"'");
    }
}
