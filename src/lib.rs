// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( nightly, feature(doc_cfg, external_doc) )]
#![cfg_attr( nightly, doc(include = "../README.md")  )]
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

#[ cfg( feature = "thespis" ) ] mod actor;
#[ cfg( feature = "thespis" ) ] pub use actor::*;



// External dependencies
//
mod import
{
	pub(crate) use
	{
		async_executors :: { SpawnHandle, SpawnHandleExt, LocalSpawnHandle, LocalSpawnHandleExt, JoinHandle },
		futures         :: { Stream, Sink, StreamExt, channel::mpsc::{ UnboundedSender, UnboundedReceiver, unbounded, TrySendError } },
		futures         :: { task::{ FutureObj, LocalFutureObj, SpawnError, Spawn, LocalSpawn } },
		futures         :: { stream::FuturesUnordered, FutureExt, lock::Mutex as FutMutex, executor::block_on },
		std             :: { task::{ Context, Poll, Waker }, pin::Pin, future::Future, sync::{ Arc, atomic::{ AtomicUsize, AtomicBool, Ordering::SeqCst } } } ,
		log             :: { * } ,
		parking_lot     :: { Mutex } ,
		thiserror       :: { * } ,
	};


	// #[ cfg( test ) ]
	// //
	// pub(crate) use
	// {
	// 	pretty_assertions :: { assert_eq } ,
	// };
}


