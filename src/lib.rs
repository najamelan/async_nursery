// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ cfg_attr( nightly, cfg_attr( nightly, doc = include_str!("../README.md") )) ]
#![doc = ""] // empty doc line to handle missing doc warning when the feature is missing.

#![ doc    ( html_root_url = "https://docs.rs/async_nursery" ) ]
#![ forbid ( unsafe_code                                     ) ]
#![ allow  ( clippy::suspicious_else_formatting              ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]


mod error          ;
mod nurse          ;
mod nursery        ;
mod nursery_stream ;
mod local_nurse    ;

pub use
{
	error          :: * ,
	nurse          :: * ,
	nursery        :: * ,
	nursery_stream :: * ,
	local_nurse    :: * ,
};

#[ cfg( feature = "tracing" ) ] mod tracing;
#[ cfg( feature = "tracing" ) ] pub use tracing::*;

// External dependencies
//
mod import
{
	pub(crate) use
	{
		async_executors  :: { SpawnHandle, LocalSpawnHandle, JoinHandle                             } ,
		futures          :: { ready, Stream, Sink, future::FusedFuture, stream::FusedStream         } ,
		futures::channel :: { mpsc::{ UnboundedSender, UnboundedReceiver, unbounded, TrySendError } } ,
		futures::task    :: { FutureObj, LocalFutureObj, SpawnError, Spawn, LocalSpawn              } ,
		futures          :: { stream::FuturesUnordered                                              } ,
		std              :: { task::{ Context, Poll }, pin::Pin, future::Future, sync::Arc, rc::Rc  } ,
	};
}


