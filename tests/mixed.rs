#![ cfg( feature = "implementation" ) ]

// Tested:
//
// ✔ Mix spawning and consuming.
// ✔ Mix spawning and consuming in concurrent tasks.
//
#![ cfg(not( target_arch = "wasm32" )) ]

mod common;

use common::{ import::*, DynSendResult };



// Mix spawning and consuming.
//
#[ async_std::test ]
//
async fn mixed_spawn_consume() -> DynSendResult<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd );
	let mut accu    = 0;

	nursery.nurse( async { 5 + 5 } )?;
	accu += output.next().await.unwrap();

	nursery.nurse( async { 5 + 5 } )?;
	accu += output.next().await.unwrap();

	assert_eq!( 20, accu );

	drop(nursery);
	assert_eq!( None, output.next().await );

	Ok(())
}


// Mix spawning and consuming in concurrent tasks.
//
#[ async_std::test ]
//
async fn mixed_spawn_consume_concurrent() -> DynSendResult<()>
{
	async fn spawner( nursery: Nursery<AsyncStd, usize> ) -> DynSendResult<()>
	{
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}


	let (nursery, output) = Nursery::new( AsyncStd );

	let handle  = AsyncStd.spawn_handle( spawner( nursery.clone() ) )?;
	let handle2 = AsyncStd.spawn_handle( spawner( nursery.clone() ) )?;

	// if we don't do this, the nursery fold finishes before even a single task has been spawned on it.
	//
	nursery.nurse( async { 5 + 5 } )?;

	assert!( handle .await.is_ok() );
	assert!( handle2.await.is_ok() );
	drop(nursery);

	let sum = output.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 30, sum );

	Ok(())
}
