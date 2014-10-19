#![feature(default_type_params)]

extern crate iron;
extern crate serialize;
extern crate plugin;

use iron::Request;
use iron::typemap::Assoc;

use plugin::{PluginFor, Phantom};

use serialize::{json, Decodable};
use serialize::json::{Decoder, DecoderError};

#[deriving(Clone)]
pub struct StructParser<T: Decodable<Decoder, DecoderError>>;

impl<T: 'static + Decodable<Decoder, DecoderError>> Assoc<T> for StructParser<T> {}

impl<T: Decodable<Decoder, DecoderError>> PluginFor<Request, T> for StructParser<T> {
    fn eval(req: &Request, _: Phantom<StructParser<T>>) -> Option<T> {
        if !req.body.is_empty() {
            let json_object = match json::from_str(req.body.as_slice()).ok() {
                Some(json_object) => json_object,
                None => {return None;},
            };
            let mut decoder = json::Decoder::new(json_object);
            match Decodable::decode(&mut decoder) {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
