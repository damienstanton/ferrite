
use std::pin::Pin;
use std::marker::PhantomData;
use async_std::task;
use async_macros::join;
use std::future::Future;
use async_std::sync::{ Sender, channel };

pub use crate::base::{
  Nat,
  Z,
  TyApp,
  Protocol,
  Context,
  PartialSession,
  ContextLens,
};

pub use crate::processes::*;
pub use crate::process::choice2::*;

struct SessionCon < I >
  ( PhantomData < I > );

struct ContextCon < N, I, P, Row >
  ( PhantomData <( N, I, P, Row )> );

struct InternalCont < N, I, P, Row, Root >
  ( PhantomData <( N, I, P, Row, Root )> );

struct MakeCont {}

impl < I, P >
  TyApp < P > for
  SessionCon < I >
where
  P : Protocol,
  I : Context,
{
  type Type =
    PartialSession < I, P >;
}

impl < N, I, P, Q, Row >
  TyApp < P > for
  ContextCon < N, I, Q, Row >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >
{
  type Type =
    PartialSession <
      N :: Target,
      Q
    >;
}

impl < N, I, P, Q, Row, Root >
  TyApp < P > for
  InternalCont < N, I, Q, Row, Root >
where
  P : Protocol,
  Q : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      P
    >,
{
  type Type =
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          Q
        >
      ) ->
        Root
      + Send
    >;
}

impl
  < A, Root, N, I, P, Row >
  LiftField2
  < (),
    InternalCont < N, I, P, Row, Root >,
    A,
    ContextCon < N, I, P, Row >,
    Root
  > for
  MakeCont
where
  A : Protocol,
  P : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  N :
    ContextLens <
      I,
      InternalChoice < Row >,
      A
    >
{

  fn lift_field (
    inject :
      impl Fn (
        PartialSession <
          N :: Target,
          P
        >
      ) ->
        Root
      + Send + 'static,
    field : ()
  ) ->
    Box <
      dyn FnOnce (
        PartialSession <
          N :: Target,
          P
        >
      ) ->
        Root
      + Send
    >
  {
    Box::new ( inject )
  }
}

fn id < A > (a : A) -> A {
  a
}

fn make_cont_sum
  < N, I, P, Row >
  ( selector :
      < Row as
        SumRow < () >
      > :: Field
  ) ->
    < Row as
      SumRow <
        InternalCont <
          N, I, P, Row,
          < Row as
            SumRow <
              ContextCon < N, I, P, Row >
            >
          > :: Field
        >
      >
    > :: Field
where
  P : Protocol,
  I : Context,
  InternalChoice < Row > :
    Protocol,
  Row : SumRow < () >,
  Row :
    SumRow <
      ContextCon < N, I, P, Row >
    >,
  Row :
    SumRow <
      InternalCont <
        N, I, P, Row,
        < Row as
          SumRow <
            ContextCon < N, I, P, Row >
          >
        > :: Field
      >
    >,
  Row :
    LiftSum2 <
      (),
      InternalCont <
        N, I, P, Row,
        < Row as
          SumRow <
            ContextCon < N, I, P, Row >
          >
        > :: Field
      >,
      MakeCont,
      ContextCon < N, I, P, Row >,
      < Row as
        SumRow <
          ContextCon < N, I, P, Row >
        >
      > :: Field
    >,
  < Row as
    SumRow <
      ContextCon < N, I, P, Row >
    >
  > :: Field :
    'static,
{
  Row :: lift_sum (
    id,
    selector
  )
}

type TestSum < A, B, P > =
  Sum <
    PartialSession <
      (A, ()),
      P
    >,
    Sum <
      PartialSession <
        (B, ()),
        P
      >,
      Bottom
    >
  >;

fn make_test_sum
  < A, B, P >
  () ->
    Sum <
      Box <
        dyn FnOnce (
          PartialSession <
            (A, ()),
            P
          >
        ) ->
          TestSum < A, B, P >
        + Send
      >,
      Sum <
        Box <
          dyn FnOnce (
            PartialSession <
              (B, ()),
              P
            >
          ) ->
            TestSum < A, B, P >
          + Send
        >,
        Bottom
      >
    >
where
  A : Protocol,
  B : Protocol,
  P : Protocol,
{
  make_cont_sum ::
    < Z,
      ( InternalChoice <
        ( A, ( B, () ))
      >,
      () ),
      P,
      ( A, ( B, () ))
    >
    (Sum::Inl(()))
}