//! By passing a nursery into an async task we spawned, it can spawn other tasks that outlive itself.
//! You should see from the output that the slow tasks end after subtask_spawn has ended.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example subtask_spawn
//!
//! INFO [subtask_spawn] nursery created
//! INFO [subtask_spawn] spawned slow: 2
//! INFO [subtask_spawn] end of subtask_spawn.
//! INFO [subtask_spawn] spawned slow: 5
//! INFO [subtask_spawn] spawned slow: 1
//! INFO [subtask_spawn] spawned slow: 4
//! INFO [subtask_spawn] spawned slow: 3
//! INFO [subtask_spawn] ended slow: 1
//! INFO [subtask_spawn] ended slow: 5
//! INFO [subtask_spawn] ended slow: 2
//! INFO [subtask_spawn] ended slow: 3
//! INFO [subtask_spawn] ended slow: 4
//!
mod common;

use
{
	async_executors :: { AsyncStd                 } ,
	async_nursery   :: { Nursery, Nurse, NurseExt } ,
	common          :: { DynResult, setup_tracing } ,
	futures_timer   :: { Delay                    } ,
	std             :: { time::Duration           } ,
	tracing_crate   :: { info                     } ,
};



async fn subtask_spawn( amount: usize, nursery: impl Nurse<DynResult<()>> ) -> DynResult<()>
{
	for i in 1..=amount
	{
		nursery.nurse( slow(i) )?;
	}

	info!( "end of subtask_spawn." );
	Ok(())
}



// This wants to linger around for an entire minute...zzz
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

	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	// subtask_spawn will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	nursery.nurse( subtask_spawn(5, nursery.clone()) )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	drop(nursery);

	output.await;

	Ok(())
}
