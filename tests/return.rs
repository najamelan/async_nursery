#![ cfg( feature = "implementation" ) ]

// Tested:
//
// ✔ return values
// ✔ return values - single thread.
// ✔ early return on error.
// - return catch_unwind
//
#![ cfg(not( target_arch = "wasm32" )) ]

mod common;
use common::{ *, import::* };

// Basic usage within function.
//
#[ async_std::test ]
//
async fn in_method() -> DynSendResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd );

	nursery.nurse( async { 5 + 5 } )?;
	nursery.nurse( async { 5 + 5 } )?;

	drop(nursery);

	let sum = output.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 20, sum );

	Ok(())
}


// Basic usage within function.
//
#[test] fn in_method_local() -> DynResult<()>
{
	let exec              = TokioCt::new()?;
	let (nursery, output) = Nursery::new( exec.clone() );

	nursery.nurse( async { 5 + 5 } )?;
	nursery.nurse( async { 5 + 5 } )?;
	drop(nursery);

	let sum = exec.block_on( output.fold( 0, |acc, x| async move { acc + x } ) );

	assert_eq!( 20, sum );

	Ok(())
}



async fn return_error() -> DynSendResult<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd );

	nursery.nurse( slow()  )?;
	nursery.nurse( wrong() )?;

	drop(nursery);

	while output.try_next().await?.is_some() {};

	unreachable!( "drop Nursery and NurseryStream" );
}


async fn slow() -> DynSendResult<()>
{
	Delay::new( Duration::from_secs(5) ).await;

	unreachable!( "Should never get executed because of error in wrong." );
}


async fn wrong() -> DynSendResult<()>
{
	Err( "I don't like waiting.".into() )
}



// early return on error.
//
#[ async_std::test ]
//
async fn early_return_error() -> DynSendResult<()>
{
	let err = return_error().await;

	assert!( err.is_err() );

	Ok(())
}
