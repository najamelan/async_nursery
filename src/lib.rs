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

#[ cfg( feature = "thespis" ) ] mod actor;
#[ cfg( feature = "thespis" ) ] pub use actor::*;

pub use
{
	nurse::*,
	nursery::*,
};



// External dependencies
//
mod import
{
	pub(crate) use
	{
		async_executors:: { SpawnHandle, JoinHandle },
		futures :: { Stream },
		futures :: { task::{ FutureObj, LocalFutureObj, SpawnError, Spawn } },
		futures :: { stream::FuturesUnordered, FutureExt },
		std :: { task::{ Context, Poll }, pin::Pin, future::Future } ,
	};


	// #[ cfg( test ) ]
	// //
	// pub(crate) use
	// {
	// 	pretty_assertions :: { assert_eq } ,
	// };
}


