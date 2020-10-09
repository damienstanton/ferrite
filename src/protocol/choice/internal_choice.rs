use crate::base::*;
use super::utils::*;
use crate::functional::row::*;

pub struct InternalChoice < Row >
where
  Row : RowCon,
{ pub (crate) field :
    AppliedSum < Row, ReceiverF >
}

impl < Row >
  Protocol for
  InternalChoice < Row >
where
  Row : Send + 'static,
  Row : RowCon,
{ }

impl < Row, A >
  RecApp < A > for
  InternalChoice < Row >
where
  Row : RowCon,
  Row : RecApp < A >,
  Row::Applied : RowCon,
{
  type Applied =
    InternalChoice <
      Row::Applied
    >;
}
