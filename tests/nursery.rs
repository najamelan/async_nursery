// Tested:
//
// ✔ Verify close_nursery works.
// - test Sink impl.
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
