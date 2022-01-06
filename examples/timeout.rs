//! With an executor from async_executors you can use the timeout method to limit the life
//! time of spawned tasks. Tasks that time out are dropped and no result is sent to the
//! output stream.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example resource_await
//!
//! INFO timeout: nursery created
//! INFO timeout: spawned slow 0
//! INFO timeout: spawned slow 3
//! INFO timeout: spawned slow 4
//! INFO timeout: spawned slow 1
//! INFO timeout: spawned slow 2
//! INFO timeout: completed slow 0
//! INFO timeout: completed slow 1
//! INFO timeout: completed slow 2
//! INFO timeout: drop Nursery and NurseryStream
//!
mod common;

use
{
	async_executors :: { AsyncStd, TimerExt       } ,
	async_nursery   :: { Nursery, NurseExt        } ,
	common          :: { DynResult, setup_tracing } ,
	futures_timer   :: { Delay                    } ,
	std             :: { time::Duration           } ,
	tracing_crate   :: { info                     } ,
};



async fn resource_await( amount: usize ) -> DynResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );
	let delay = Duration::from_secs(2);

	for i in 0..amount
	{
		let task = nursery.timeout( delay, slow(i) );
		nursery.nurse( task )?;
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



async fn slow( seconds: usize ) -> DynResult<()>
{
	info!( "spawned slow {}", seconds );

	Delay::new( Duration::from_secs(seconds as u64) ).await;

	info!( "completed slow {}", seconds );

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
