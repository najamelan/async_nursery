// Tested:
//
// ✔ Basic usage within function.
// ✔ A nursery passed in to a function that uses it to spawn.
//
mod common;

use common::{ import::*, DynResult };


// Basic usage within function.
//
fn in_method() -> DynResult
{
	let exec    = TokioCt::try_from( &mut Builder::new() )?;
	let nursery = Nursery::new( exec )?;

	nursery.nurse( async { 5 + 5 } )?;
	nursery.nurse( async { 5 + 5 } )?;

	let sum = block_on( nursery.fold( 0, |acc, x| async move { acc + x } ).await );

	assert_eq!( 20, sum );

	Ok(())
}


// A nursery passed in to a function that uses it to spawn.
//
fn outlive_method() -> DynResult
{
	fn outlive( nursery: &Nursery<AsyncStd, usize> ) -> DynResult
	{
		nursery.nurse( async { 5 + 5 } )?;
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}

	let exec    = TokioCt::try_from( &mut Builder::new() )?;
	let nursery = Nursery::new( exec )?;

	outlive( &nursery )?;

	let sum = block_on( nursery.fold( 0, |acc, x| async move { acc + x } ).await );

	assert_eq!( 20, sum );

	Ok(())
}
