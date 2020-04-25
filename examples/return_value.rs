//! Capture return values from all spawned tasks in a Nursery.
//!
//! Expected output:
//!
//! $ cargo run --example return_value
//!
//! Total of all concurrent operations is: 30.
//!
mod common;

use
{
	async_executors :: { AsyncStd                 } ,
	async_nursery   :: { Nursery, Nurse, NurseExt } ,
	futures         :: { StreamExt                } ,
	common          :: { DynResult                } ,
};


// This function will spawn tasks, but guarantee that when it returns all
// concurrent tasks have been joined.
//
// This could in principle be achieved with things like `join!` or FuturesUnordered, but it
// quickly becomes unwieldy if it has to scale. This nursery can be passed
// down arbitrarily into function calls to add further concurrent operations.
//
async fn return_value() -> DynResult<usize>
{
	let (nursery, output) = Nursery::new( AsyncStd );

	nursery.nurse( produce_value () )?;
	nursery.nurse( produce_value2() )?;


	produce_value3( &nursery )?;

	drop(nursery);

	// Accumulate all the values from spawned tasks.
	//
	let sum = output.fold( 0, |acc, x| async move {	acc + x } ).await;

	Ok( sum )
}


async fn produce_value () -> usize {  5 }
async fn produce_value2() -> usize { 10 }

fn produce_value3( nursery: &(impl Nurse<usize> + Send + 'static) ) -> DynResult<()>
{
	nursery.nurse( produce_value () )?;
	nursery.nurse( produce_value2() )?;
	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	let sum = return_value().await?;

	assert_eq!( sum, 30 );

	println!( "Total of all concurrent operations is: {}.", sum );

	Ok(())
}
