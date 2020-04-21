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


mod nurse;
mod nursery;
mod nursery_handle;

#[ cfg( feature = "thespis" ) ] mod actor;
#[ cfg( feature = "thespis" ) ] pub use actor::*;

pub use
{
	nurse::*,
	nursery::*,
	nursery_handle::*,
};



// External dependencies
//
mod import
{
	pub(crate) use
	{
		async_executors:: { SpawnHandle, SpawnHandleExt, JoinHandle },
		futures :: { Stream, StreamExt, channel::mpsc::{ UnboundedSender, unbounded } },
		futures :: { task::{ FutureObj, LocalFutureObj, SpawnError, Spawn } },
		futures :: { stream::FuturesUnordered, FutureExt, lock::Mutex as FutMutex, executor::block_on },
		std :: { task::{ Context, Poll, Waker }, pin::Pin, future::Future, sync::{ Arc, atomic::{ AtomicUsize, Ordering::SeqCst } } } ,
		log :: { * },
		parking_lot:: { Mutex } ,
	};


	// #[ cfg( test ) ]
	// //
	// pub(crate) use
	// {
	// 	pretty_assertions :: { assert_eq } ,
	// };
}


