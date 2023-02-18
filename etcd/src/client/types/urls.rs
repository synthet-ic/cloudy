/*!
<https://github.com/etcd-io/etcd/blob/main/client/pkg/types/urls.go>
*/

use std::error::Error;

use http::Uri as URI;

pub fn new_urls(strs: Vec<String>) -> Result<Vec<URI>, dyn Error> {
    if strs.is_empty() {
        return Err("No valid URLs given")
    }
    let urls = Vec::new();
    for (i, s) in strs.iter() {
        let s = s.trim();
        let uri = s.parse::<URI>(s)?;
        match uri.scheme {
           "http" | "https" => {
                if uri.port().is_none() {
                    return Err(format!("URL address does not have the form \"host:port\": {}", s))
                }

                if !uri.path().is_empty() {
                    return Err(format!("URL must not contain a path: {}", s))
                }
           },
           "unix" | "unixs" => { break },
           _ => {
                return Err(format!("URL scheme must be http, https, unix, or unixs: {}", s))
           }
        }
        urls.push(uri);
    }
    urls.sort();
    Ok(urls)
}
