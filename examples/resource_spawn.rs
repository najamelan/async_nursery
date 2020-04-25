//! By passing a nursery into an async task we spawned, it can spawn other tasks that outlive itself.
//! You should see from the output that the slow tasks end after resource_spawn has ended.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example resource_spawn
//!
//! INFO [resource_spawn] nursery created
//! INFO [resource_spawn] spawned slow: 2
//! INFO [resource_spawn] end of resource_spawn.
//! INFO [resource_spawn] spawned slow: 5
//! INFO [resource_spawn] spawned slow: 1
//! INFO [resource_spawn] spawned slow: 4
//! INFO [resource_spawn] spawned slow: 3
//! INFO [resource_spawn] ended slow: 1
//! INFO [resource_spawn] ended slow: 5
//! INFO [resource_spawn] ended slow: 2
//! INFO [resource_spawn] ended slow: 3
//! INFO [resource_spawn] ended slow: 4
//!
mod common;

use
{
	async_executors :: { AsyncStd                 } ,
	async_nursery   :: { Nursery, Nurse, NurseExt } ,
	log             :: { info                     } ,
	std             :: { time::Duration           } ,
	futures_timer   :: { Delay                    } ,
	common          :: { DynResult                } ,
};



async fn resource_spawn( amount: usize, nursery: impl Nurse<DynResult<()>> ) -> DynResult<()>
{
	for i in 1..=amount
	{
		nursery.nurse( slow(i) )?;
	}

	info!( "end of resource_spawn." );
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
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	// resource_spawn will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	nursery.nurse( resource_spawn(5, nursery.clone()) )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	drop(nursery);

	output.await;

	Ok(())
}
