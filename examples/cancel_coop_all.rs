//! A form of cooperative cancellation supported by Nursery. If you close the nursery,
//! trying to spawn on it will cause an error, so in the case of a task that spawns in a
//! loop, you can let it detect the cancellation by closing the nursery.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example cancel_coop_all
//!
//! INFO [cancel_coop_all] nursery created
//! INFO [cancel_coop_all] spawned slow: 1
//! INFO [cancel_coop_all] spawned slow: 2
//! INFO [cancel_coop_all] canceling
//! INFO [cancel_coop_all] cancel_coop_all doing it's cleanup before cancelling
//! INFO [cancel_coop_all] ended slow: 1
//! INFO [cancel_coop_all] ended slow: 2
//!
mod common;

use
{
	async_executors :: { AsyncStd                           } ,
	async_nursery   :: { Nursery, Nurse, NurseExt, NurseErr } ,
	common          :: { DynResult, setup_tracing           } ,
	futures_timer   :: { Delay                              } ,
	std             :: { time::Duration                     } ,
	tracing_crate   :: { info                               } ,
};



async fn cancel_coop_all( amount: usize, nursery: impl Nurse<DynResult<()>> ) -> DynResult<()>
{
	// We will stop spawning new subtasks when the nursery is closed.
	//
	for i in 1..=amount
	{
		match nursery.nurse( slow(i) )
		{
			Ok(()) => Delay::new( Duration::from_secs(1) ).await,

			Err(e) =>
			{
				assert!(matches!( e, NurseErr::Closed ) );

				info!( "cancel_coop_all doing it's cleanup before cancelling" );

				return Ok(());
			}
		}
	}

	info!( "end of cancel_coop_all." );
	Ok(())
}



// This wants to linger around for an entire 3 seconds...zzz
//
async fn slow( i: usize ) -> DynResult<()>
{
	info!( "spawned slow: {}", i );

	Delay::new( Duration::from_secs(3) ).await;

	info!( "ended slow: {}", i );

	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	setup_tracing();

	let (nursery, mut output) = Nursery::new( AsyncStd );
	info!( "nursery created" );

	// cancel_coop_all will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	nursery.nurse( cancel_coop_all( 5, nursery.clone() ) )?;

	Delay::new( Duration::from_secs(2) ).await;


	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	info!( "canceling" );

	// Each `Nursery` also has a `close_nursery` method, which will close everything and
	// prevent other `Nursery`s from spawning.
	//
	output.close_nursery().await;

	Ok(())
}
