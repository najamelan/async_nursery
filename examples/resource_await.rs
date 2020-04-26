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
	async_executors :: { AsyncStd          } ,
	async_nursery   :: { Nursery, NurseExt } ,
	log             :: { info              } ,
	std             :: { time::Duration    } ,
	futures_timer   :: { Delay             } ,
	common          :: { DynResult         } ,
};



async fn resource_await( amount: usize ) -> DynResult<()>
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



// This wants to linger around for an entire minute...zzz
//
async fn slow() -> DynResult<()>
{
	info!( "spawned slow" );

	Delay::new( Duration::from_secs(3) ).await;

	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	resource_await( 5 ).await?;

	Ok(())
}
