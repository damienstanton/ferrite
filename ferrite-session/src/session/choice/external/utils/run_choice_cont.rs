use std::pin::Pin;
use std::future::Future;
use std::marker::PhantomData;
use async_std::sync::{ Sender, Receiver, channel };

use crate::base::*;
use crate::protocol::*;
use crate::functional::*;

use super::super::cloak_session::*;

pub async fn run_choice_cont
  < Row, C >
  ( ctx: C::Endpoints,
    sender: Sender < AppliedSum < Row, ReceiverF > >,
    cont1: AppliedSum < Row, SessionF < C > >,
  )
where
  C: Context,
  Row: ElimSum,
  Row: SplitRow,
  Row: SumFunctorInject,
{
  let res =
    lift_sum_inject (
      RunSession { ctx: ctx },
      cont1
    );

  let (receiver_sum, cont6) = Row::split_row( res );

  sender.send( receiver_sum ).await;
  Row::elim_sum ( ElimConst{}, cont6 ).await;
}

struct RunSession < C >
where
  C : Context
{
  ctx: C::Endpoints,
}

struct SessionRunner < C, A >
where
  C : Context
{
  ctx: C::Endpoints,
  phantom: PhantomData < A >
}

impl < C, A >
  NeedPartialSession <
    C, A,
    ( Receiver < A >,
      Pin < Box < dyn
        Future < Output=() >
        + Send + 'static
      > >
    )
  >
  for SessionRunner < C, A >
where
  C : Context
{
  fn on_partial_session
    ( self: Box < Self >,
      cont: PartialSession < C, A >
    ) ->
      ( Receiver < A >,
        Pin < Box < dyn
          Future < Output=() >
          + Send + 'static
        > >
      )
  where
    C: Context,
    A: Protocol,
  {
    let (sender, receiver) = channel(1);
    let future = Box::pin(async move {
      unsafe_run_session(
        cont,
        self.ctx,
        sender
      ).await;
    });

    ( receiver, future )
  }
}

impl
  < Root, C >
  InjectLift < Root >
  for RunSession < C >
where
  C : Context,
{
  type SourceF = SessionF < C >;

  type TargetF = ();

  type InjectF =
    Merge <
      ReceiverF,
      Const <
        Pin < Box < dyn
          Future < Output=() >
          + Send + 'static
        > > > >
  ;

  fn lift_field < A >
  ( self,
    _inject:
      impl Fn
        ( Applied < Self::TargetF, A > )
        -> Root
      + Send + 'static,
    cont1:
      Applied < Self::SourceF, A >
  ) ->
    Applied < Self::InjectF, A >
  where
    A: Send + 'static,
  {
    let cont2 : CloakedSession < C, A >
      = get_applied( cont1 );

    let runner: SessionRunner < C, A >
      = SessionRunner {
        ctx: self.ctx,
        phantom: PhantomData,
      };

    let (receiver, future) = *with_session (
      cont2,
      Box::new( runner )
    );

    cloak_applied( (
      cloak_applied(receiver),
      cloak_applied(future),
    ) )
  }
}