use std::marker::PhantomData;
use std::ops::Deref;

use rocket::{
    data::{FromDataFuture, FromTransformedData, Outcome, TransformFuture, Transformed},
    Data, Request,
};
use rocket_contrib::json::Json;

use super::ExternallyExposedIncoming;

pub struct IncomingModel<'de, T: ExternallyExposedIncoming<'de>>(T, PhantomData<&'de ()>);

impl<'a, T: ExternallyExposedIncoming<'a>> Deref for IncomingModel<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, T: ExternallyExposedIncoming<'de>> FromTransformedData<'de> for IncomingModel<'de, T> {
    type Error = <Json<T> as FromTransformedData<'de>>::Error;
    type Owned = <Json<T> as FromTransformedData<'de>>::Owned;
    type Borrowed = <Json<T> as FromTransformedData<'de>>::Borrowed;

    fn transform(
        request: &'de Request<'_>,
        data: Data,
    ) -> TransformFuture<'de, Self::Owned, Self::Error> {
        <Json<T> as FromTransformedData<'de>>::transform(request, data)
    }

    fn from_data(
        request: &'de Request<'_>,
        outcome: Transformed<'de, Self>,
    ) -> FromDataFuture<'de, Self, Self::Error> {
        Box::pin(async move {
            let json = try_outcome!(
                <Json<T> as FromTransformedData<'de>>::from_data(request, outcome).await
            );

            Outcome::Success(IncomingModel(json.into_inner(), PhantomData))
        })
    }
}
