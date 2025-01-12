//! By awaiting the `NurseryStream` we keep the resource_await method from returning until
//! all spawned tasks have completed.
//!
//! The example shouldn't return immediately, rather wait for 3 seconds for the tasks to finish.
//! Can be done in a sync method by using block_on, or of course it's possible to return the `NurseryStream`
//! to the caller, but I would suggest that in that case you pass in the nursery from the caller.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example resource_await
//!
//! INFO [resource_await] nursery created
//! INFO [resource_await] spawned slow
//! INFO [resource_await] spawned slow
//! INFO [resource_await] spawned slow
//! INFO [resource_await] spawned slow
//! INFO [resource_await] spawned slow
//! INFO [resource_await] drop Nursery and NurseryStream
//!
mod common;

use
{
	async_executors :: { AsyncStd                     } ,
	async_nursery   :: { Nursery, NurseExt            } ,
	common          :: { DynSendResult, setup_tracing } ,
	futures_timer   :: { Delay                        } ,
	std             :: { time::Duration               } ,
	tracing_crate   :: { info                         } ,
};



async fn resource_await( amount: usize ) -> DynSendResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	for _ in 0..amount
	{
		nursery.nurse( slow() )?;
	}

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`. If we don't, the await below deadlocks.
	//
	drop(nursery);

	// Resolves when all spawned tasks are done.
	//
	output.await;

	info!( "drop Nursery and NurseryStream" );
	Ok(())
}



async fn slow() -> DynSendResult<()>
{
	info!( "spawned slow" );

	Delay::new( Duration::from_secs(3) ).await;

	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynSendResult<()>
{
	setup_tracing();

	resource_await( 5 ).await?;

	Ok(())
}
