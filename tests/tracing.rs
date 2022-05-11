#![ cfg(all( feature = "tracing", feature = "implementation" )) ]

// Tested:
//
// ✔ Verify close_nursery works.
// - test Sink impl.
// ✔ test forwarding of traits on tracing types.
//
#![ cfg(all( not(target_arch = "wasm32"), feature = "tracing" )) ]

mod common;
use common::{ *, import::* };
use tracing_futures::Instrument;
use tracing_crate::info_span;


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


// Verify traits are all available on Instrumented.
//
#[async_std::test] async fn traits() -> DynResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd );

	let nursery = nursery.instrument( info_span!( "instrumented" ) );
	let three = nursery.spawn_blocking( ||{ 3 } ).await;

		assert_eq!( three, 3 );


	nursery.yield_now().await;
	nursery.spawn( async {} )?;
	nursery.sleep( std::time::Duration::from_millis(1) ).await;

	drop(nursery);
	output.await;

	Ok(())
}
