use iced::futures;
use notify::{DebouncedEvent, Watcher, watcher};
use std;
use std::task::Poll;
use std::{time::Duration};

pub fn watch<T: ToString>(path: T) -> iced::Subscription<FileEvent> {
    iced::Subscription::from_recipe(FileNotification::new(path.to_string()))
}

/// ファイル監視用Subscription
pub struct FileNotification {
    root: String,
}

impl FileNotification {
    fn new(path: String) -> Self {
        FileNotification {
            root: path,
        }
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for FileNotification
where
    H: std::hash::Hasher,
{
    type Output = FileEvent;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.root.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
        watcher
            .watch(self.root, notify::RecursiveMode::Recursive)
            .unwrap();
        Box::pin(futures::stream::poll_fn(move |_| {
            match rx.recv() {
                Ok(event) => Poll::Ready(Some(FileEvent::Changed)),
                Err(_) => Poll::Ready(None),
            }
        }))
    }
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Changed,
}
