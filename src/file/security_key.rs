use deku::prelude::*;

#[derive(DekuRead, DekuWrite, Debug, Clone, PartialEq)]
pub struct SecurityKey {
  // TODO: not sure if u128 is available on all archs
  pub key: u128,

}