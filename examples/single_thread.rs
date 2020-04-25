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
	async_executors :: { TokioCt, LocalSpawnHandle                } ,
	tokio           :: { runtime::Builder                         } ,
	async_nursery   :: { Nursery, LocalNurseExt                   } ,
	log             :: { info, error                              } ,
	std             :: { time::Duration, convert::TryFrom, rc::Rc } ,
	futures_timer   :: { Delay                                    } ,
	futures         :: { TryStreamExt                             } ,
	common          :: { DynResult                                } ,
};


async fn return_error( exec: impl LocalSpawnHandle<DynResult<()>> ) -> DynResult<()>
{
	let (nursery, mut output) = Nursery::new( exec ); info!( "nursery created" );

	nursery.nurse_local( slow()  )?;
	nursery.nurse_local( wrong() )?;

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
	while output.try_next().await?.is_some() {};

	unreachable!( "drop Nursery and NurseryStream" );
}



// Linger.
//
async fn slow() -> DynResult<()>
{
	info!( "spawned slow." );

	let delay = Rc::new(5);
	Delay::new( Duration::from_secs( *delay) ).await;

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



fn main() -> DynResult<()>
{
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	let exec = TokioCt::try_from( &mut Builder::new() )?;

	let err = exec.block_on( return_error( exec.clone() ) );

	assert!( err.is_err() );

	Ok(())
}
