// Tested:
//
// ✔ pass nursery to function as reference
// ✔ pass nursery to function as clone
// ✔ pass nursery to spawned subtasks as clone
//
#![ cfg( target_arch = "wasm32" ) ]

mod common;
use common::import   ::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!( run_in_browser );



#[ wasm_bindgen_test ]
//
async fn spawn()
{
	let (nursery, output) = Nursery::new( Bindgen );

	nursery.nurse( async { 5 + 5 } ).unwrap();
	nursery.nurse( async { 5 + 5 } ).unwrap();

	drop(nursery);

	let sum = output.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 20, sum );
}
