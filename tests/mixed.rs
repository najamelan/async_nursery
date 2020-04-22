// Tested:
//
// ✔ Mix spawning and consuming.
// ✔ Mix spawning and consuming in concurrent tasks.
//
mod common;

use common::{ import::*, DynResult };



// Mix spawning and consuming.
//
#[ async_std::test ]
//
async fn mixed_spawn_consume() -> DynResult
{
	let mut nursery = Nursery::new( AsyncStd )?;
	let mut accu    = 0;

	nursery.nurse( async { 5 + 5 } )?;
	accu += nursery.next().await.unwrap();

	nursery.nurse( async { 5 + 5 } )?;
	accu += nursery.next().await.unwrap();

	assert_eq!( 20, accu );

	nursery.stop();
	assert_eq!( None, nursery.next().await );

	Ok(())
}


// Mix spawning and consuming in concurrent tasks.
//
#[ async_std::test ]
//
async fn mixed_spawn_consume_concurrent() -> DynResult
{
	async fn spawner( nursery: NurseryHandle<AsyncStd, usize> ) -> DynResult
	{
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}


	let mut nursery = Nursery::new( AsyncStd )?;

	let handle  = AsyncStd.spawn_handle( spawner( nursery.handle() ) )?;
	let handle2 = AsyncStd.spawn_handle( spawner( nursery.handle() ) )?;

	// if we don't do this, the nursery fold finishes before even a single task has been spawned on it.
	//
	nursery.nurse( async { 5 + 5 } )?;

	assert!( handle .await.is_ok() );
	assert!( handle2.await.is_ok() );
	nursery.stop();

	let sum = nursery.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 30, sum );

	Ok(())
}
