use crate::base::*;
use crate::functional::*;
use super::super::cloak_session::*;

pub trait SessionInjector
  < Row, C, A >
  : Send
{
  fn inject_session
    ( self: Box < Self >,
      session: PartialSession < C, A >
    ) ->
      AppliedSum <
        Row,
        SessionF < C >
      >
  where
    C: Context,
    A: Protocol,
  ;
}