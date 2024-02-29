use crate::messaging::request_types::Request;

pub trait Action {
    fn execute(&self);
    fn from_request() -> Self;
}