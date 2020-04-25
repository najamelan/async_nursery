//! See the return_catch_unwind example. If you don't want to call catch_unwind on each
//! individual future, you can also use catch_unwind on the NurseryStream or on the function
//! in which it lives.
//!
//! You have to assert that your tasks are unwind safe before doing this!
//!
//! In this example we still bail out of sibling tasks if any spawned task panics.
//!
//! Expected output:
//!
//! $ cargo run --example return_catch_unwind_all
//!
//! INFO [return_catch_unwind_all] nursery created
//! INFO [return_catch_unwind_all] spawned slow.
//! INFO [return_catch_unwind_all] spawned wrong.
//! ERROR [return_catch_unwind_all] panicked at 'I don't like waiting.', examples/return_catch_unwind_all.rs:72:5
//! INFO [return_catch_unwind_all] nursery created
//! INFO [return_catch_unwind_all] spawned slow.
//! INFO [return_catch_unwind_all] spawned wrong.
//! ERROR [return_catch_unwind_all] panicked at 'I don't like waiting.', examples/return_catch_unwind_all.rs:72:5
//! INFO [return_catch_unwind_all] nursery created
//! INFO [return_catch_unwind_all] spawned slow.
//! INFO [return_catch_unwind_all] spawned wrong.
//! ... a 100 times.
//!
use
{
	async_executors :: { AsyncStd                } ,
	async_nursery   :: { Nursery, NurseExt       } ,
	log             :: { info, error             } ,
	std             :: { time::Duration, panic::AssertUnwindSafe          } ,
	futures_timer   :: { Delay                   } ,
	futures         :: { FutureExt } ,
};


async fn return_catch_unwind_all() -> std::thread::Result<()>
{
	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	nursery.nurse( slow()  )?;
	nursery.nurse( wrong() )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	// Of course if an error happens, it wouldn't matter, but if no error happens it would
	// make the while loop below hang indefinitely.
	//
	drop(nursery);

	// If any task panics, other tasks will be cancelled and this will return an error.
	// It will not panic the main thread, however the executor might lose working threads
	// and you might loose other spawned tasks.
	//
	AssertUnwindSafe( output ).catch_unwind().await?;

	unreachable!( "drop Nursery and NurseryStream" );
}



// Linger.
//
async fn slow()
{
	info!( "spawned slow." );

	Delay::new( Duration::from_secs(5) ).await;

	error!( "I managed to stall you all." );
}



// This will return an error.
//
async fn wrong()
{
	info!( "spawned wrong." );

	panic!( "I don't like waiting." );
}



#[ async_std::main ]
//
async fn main()
{
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	// log errors instead of returning them on the stderr.
	//
	std::panic::set_hook( Box::new( |e| error!( "{}", e ) ) );

	for _ in 0..100
	{
		let err = return_catch_unwind_all().await;
		assert!( err.is_err() );
	}
}
