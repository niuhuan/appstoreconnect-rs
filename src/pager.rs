use std::marker::PhantomData;

use crate::client::Client;
use crate::entities::PageResponse;
use crate::error::Result;

pub struct Pager<'a, T> {
    client: &'a Client,
    pending_first: Option<PageResponse<T>>,
    next_url: Option<String>,
    current_items: Option<std::vec::IntoIter<T>>,
    _phantom: PhantomData<T>,
}

impl<'a, T> Pager<'a, T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    pub fn new(client: &'a Client, first: PageResponse<T>) -> Self {
        let next_url = first.links.next.clone();
        Self {
            client,
            pending_first: Some(first),
            next_url,
            current_items: None,
            _phantom: PhantomData,
        }
    }

    pub async fn next_page(&mut self) -> Result<Option<PageResponse<T>>> {
        if let Some(first) = self.pending_first.take() {
            return Ok(Some(first));
        }
        let Some(url) = self.next_url.clone() else {
            return Ok(None);
        };
        let page: PageResponse<T> = self.client.raw().get(url).send_json().await?;
        self.next_url = page.links.next.clone();
        Ok(Some(page))
    }

    pub async fn next_item(&mut self) -> Result<Option<T>> {
        loop {
            if let Some(iter) = self.current_items.as_mut() {
                if let Some(item) = iter.next() {
                    return Ok(Some(item));
                }
            }

            let Some(page) = self.next_page().await? else {
                return Ok(None);
            };
            self.current_items = Some(page.data.into_iter());
        }
    }

    #[cfg(feature = "stream")]
    pub fn stream_items(self) -> impl futures_core::Stream<Item = Result<T>> + 'a {
        futures_util::stream::unfold(Some(self), |state| async move {
            let mut pager = state?;
            match pager.next_item().await {
                Ok(Some(item)) => Some((Ok(item), Some(pager))),
                Ok(None) => None,
                Err(err) => Some((Err(err), None)),
            }
        })
    }
}
