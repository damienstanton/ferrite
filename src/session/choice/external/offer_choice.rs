use async_std::sync::{ channel };

use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

use super::utils::*;
use super::cloak_session::*;
use super::inject_session::*;

pub fn offer_choice
  < C, Row >
  ( cont1 : impl FnOnce
      ( Row::Uncloaked )
      ->
        AppliedSum <
          Row,
          SessionF < C >
        >
    + Send + 'static
  ) ->
    PartialSession < C, ExternalChoice < Row > >
where
  C : Context,
  Row : RowCon,
  Row : ElimSum,
  Row : SplitRow,
  Row : SumFunctor,
  Row : SumFunctorInject,
  Row : UncloakRow < InjectSessionF < Row, C > >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver2) = channel(1);

      let payload = ExternalChoice::< Row >
        { sender: sender2 };

      sender1.send(payload).await;

      let (choice, sender3) = receiver2.recv().await.unwrap();

      let cont3 = selector_to_inject_session( choice );

      let cont4 = Row::full_uncloak_row( cont3 );

      let cont5 = cont1 ( cont4 );

      run_choice_cont( ctx, sender3, cont5 ).await;
    })
}
