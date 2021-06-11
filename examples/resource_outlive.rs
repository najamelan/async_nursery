//! The idea of structured concurrency is to create a call graph of asynchronous components. That is
//! when a function spawns a task, that task is joined before the function returns. This is what async_nursery
//! allows you to do. However, sometimes we want to delegate pieces of our code to functions. They might
//! have to spawn but not be important in the call graph hierarchy. By passing a nursery into a function,
//! it can spawn other tasks that outlive itself. These tasks will still be joined at the level in the
//! call stack where the nursery is managed.
//!
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
	common          :: { DynResult, setup_tracing } ,
	futures_timer   :: { Delay                    } ,
	std             :: { time::Duration           } ,
	tracing_crate   :: { info                     } ,
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
	setup_tracing();

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
