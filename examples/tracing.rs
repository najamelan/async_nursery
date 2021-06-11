//! This example is a copy of the resource_await example, but demonstrates how to use tracing to instrument
//! the nursery.
//!
mod common;

use
{
	async_executors :: { AsyncStd                 } ,
	async_nursery   :: { Nursery, NurseExt        } ,
	common          :: { DynResult, setup_tracing } ,
	futures_timer   :: { Delay                    } ,
	std             :: { time::Duration           } ,
	tracing_futures :: { Instrument               } ,

	// just because we use tracing as a feature name, you don't have to do this.
	//
	tracing_crate   as tracing   ,
	tracing         :: { info  } ,
};



async fn resource_await( amount: usize ) -> DynResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );
	let nursery = nursery.instrument( tracing::info_span!( "tracing-example" ) );

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



async fn slow() -> DynResult<()>
{
	info!( "spawned slow" );

	Delay::new( Duration::from_secs(1) ).await;

	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	setup_tracing();

	resource_await( 5 ).await?;

	Ok(())
}
