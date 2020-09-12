use iced::futures;
use std::hash::Hash;
use iced::futures::stream::BoxStream;

pub fn every(duration: std::time::Duration) -> iced::Subscription<bool> {
    iced::Subscription::from_recipe(Every(duration))
}

struct Every(std::time::Duration);

impl<H, I> iced_native::subscription::Recipe<H, I> for Every
where
    H: std::hash::Hasher,
{
    type Output = bool;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.0.hash(state);
    }

    fn stream(self: Box<Self>, _: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;

        async_std::stream::interval(self.0)
            .map(|_| { true })
            .boxed()
    }
}
