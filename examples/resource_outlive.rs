//! By passing a nursery into a function, it can spawn other tasks that outlive itself.
//! You should see from the output that the slow tasks end after resource_outlive has ended.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example resource_outlive
//!
//! INFO [resource_outlive] nursery created
//! INFO [resource_outlive] spawned slow: 1
//! INFO [resource_outlive] spawned slow: 2
//! INFO [resource_outlive] spawned slow: 3
//! INFO [resource_outlive] spawned slow: 4
//! INFO [resource_outlive] spawned slow: 5
//! INFO [resource_outlive] end of resource_outlive.
//! INFO [resource_outlive] ended slow: 1
//! INFO [resource_outlive] ended slow: 3
//! INFO [resource_outlive] ended slow: 2
//! INFO [resource_outlive] ended slow: 4
//! INFO [resource_outlive] ended slow: 5
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



fn resource_outlive( amount: usize, nursery: impl Nurse<()> ) -> DynResult<()>
{
	for i in 1..=amount
	{
		nursery.nurse( slow(i) )?;
	}

	info!( "end of resource_outlive." );
	Ok(())
}



// This wants to linger around for an entire 3 seconds...zzz
//
async fn slow( i: usize )
{
	info!( "spawned slow: {}", i );

	Delay::new( Duration::from_secs(3) ).await;

	info!( "ended slow: {}", i );
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	// resource_outlive will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	resource_outlive( 5, nursery.clone() )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	drop(nursery);

	output.await;

	Ok(())
}
