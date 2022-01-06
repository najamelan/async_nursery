#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

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
mod local_nurse    ;

pub use
{
	error       :: * ,
	nurse       :: * ,
	local_nurse :: * ,
};

#[ cfg( feature = "tracing" ) ] mod tracing;
#[ cfg( feature = "tracing" ) ] pub use tracing::*;

#[ cfg( feature = "implementation" ) ] mod nursery        ;
#[ cfg( feature = "implementation" ) ] mod nursery_stream ;

#[ cfg( feature = "implementation" ) ]
//
pub use { nursery::*, nursery_stream::* };

// External dependencies
//
mod import
{
	pub(crate) use
	{
		futures_channel  :: { mpsc::{ UnboundedSender, UnboundedReceiver, unbounded, TrySendError } } ,
		futures_task     :: { FutureObj, LocalFutureObj, SpawnError                                 } ,
		std              :: { future::Future, sync::Arc, rc::Rc                                     } ,
	};


	#[ cfg( feature = "implementation" ) ]
	//
	pub(crate) use
	{
		async_executors  :: { SpawnHandle, LocalSpawnHandle, JoinHandle, Timer, TokioIo, SpawnBlocking, YieldNow, BlockingHandle, YieldNowFut } ,
		futures          :: { ready, Stream, Sink, future::{ BoxFuture, FusedFuture }, stream::{ FusedStream, FuturesUnordered } } ,
		futures_task     :: { Spawn, LocalSpawn } ,
		std              :: { task::{ Context, Poll }, pin::Pin, time::Duration                                                  } ,
	};
}


