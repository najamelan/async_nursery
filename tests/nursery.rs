#![ cfg( feature = "implementation" ) ]

// Tested:
//
// ✔ Verify close_nursery works.
// - test Sink impl.
// ✔ Verify traits are all available on Nursery.
//
#![ cfg(not( target_arch = "wasm32" )) ]

mod common;
use common::{ *, import::* };



// Verify close_nursery works.
//
#[async_std::test] async fn close_nursery() -> DynResult<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd );

	let nursery2 = nursery.clone();
	nursery2.close_nursery();

	assert!( nursery.nurse( async { 5 + 5 } ).is_err() );

	assert!( output.next().await.is_none() );

	Ok(())
}



// Verify traits are all available on Nursery.
//
#[async_std::test] async fn traits() -> DynResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd );

	let three = nursery.spawn_blocking( ||{ 3 } ).await;

		assert_eq!( three, 3 );

	nursery.yield_now().await;
	nursery.spawn( async {} )?;
	nursery.sleep( std::time::Duration::from_millis(1) ).await;

	drop(nursery);
	output.await;

	Ok(())
}
