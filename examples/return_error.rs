//! We can leverage TryStreamExt in order to cancel concurrent tasks early if one
//! runs into an error.
//!
//! Expected output:
//!
//! $ cargo run --example return_error
//!
//! INFO [return_error] nursery created
//! INFO [return_error] spawned slow.
//! INFO [return_error] spawned wrong.
//!
mod common;

use
{
	async_executors :: { AsyncStd                 } ,
	async_nursery   :: { Nursery, NurseExt        } ,
	tracing_crate   :: { info, error              } ,
	std             :: { time::Duration           } ,
	futures_timer   :: { Delay                    } ,
	futures         :: { TryStreamExt             } ,
	common          :: { DynResult, setup_tracing } ,
};


async fn return_error() -> DynResult<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	nursery.nurse( slow()  )?;
	nursery.nurse( wrong() )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	// Of course if an error happens, it wouldn't matter, but if no error happens it would
	// make the while loop below hang indefinitely.
	//
	drop(nursery);

	// Can't do it with TryFuture because we would need specialization.
	// Use TryStreamExt. This will return as soon as an error happens in
	// any spawned task and thus drop all the others when the current function
	// returns.
	//
	// Warning, that last phrase is important. It cancels all tasks when it get's
	// dropped. Not when try_next stops. I will in this example because we use
	// the `?` operator to return early and when the function returns the NurseryStream
	// get's dropped.
	//
	while output.try_next().await?.is_some() {};

	unreachable!( "drop Nursery and NurseryStream" );
}



// Linger.
//
async fn slow() -> DynResult<()>
{
	info!( "spawned slow." );

	Delay::new( Duration::from_secs(5) ).await;

	error!( "I managed to stall you all." );

	Ok(())
}



// This will return an error.
//
async fn wrong() -> DynResult<()>
{
	info!( "spawned wrong." );

	Err( "I don't like waiting.".into() )
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	setup_tracing();

	let err = return_error().await;

	assert!( err.is_err() );

	Ok(())
}
