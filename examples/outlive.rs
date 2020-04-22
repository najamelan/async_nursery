use
{
	async_executors :: { * } ,
	async_nursery   :: { * } ,
	futures         :: { StreamExt }
};

type DynError = Box< dyn std::error::Error + Send + Sync + 'static >;



// This function can spawn things that outlive it's runtime, but that will be captured and errors
// bubbled up anyway.
//
// This would be impossible with FuturesUnordered, because that is generic over the type of the future,
// and here we are passing futures that are 2 different opaque types.
//
// It would have been possible here to do this by returning a Vec or a FuturesUnordered of JoinHandles,
// but FuturesUnordered itself doesn't spawn and it doesn't necessarily scale very well.
//
fn needs_to_spawn( nursery: &(impl Nurse<usize> + Send + 'static) ) -> Result<(), DynError>
{
	nursery.nurse( produce_value () )?;
	nursery.nurse( produce_value2() )?;

	Ok(())
}


async fn produce_value () -> usize {  5 }
async fn produce_value2() -> usize { 10 }




#[ async_std::main ]
//
async fn main() -> Result<(), DynError>
{
	let (nursery, output) = Nursery::new( AsyncStd )?;

	needs_to_spawn( &nursery )?;

	nursery.stop();
	let sum = output.fold(0, |acc, x| async move { acc + x } ).await;

	assert_eq!( sum, 15 );

	println!( "Total of all concurrent operations is: {}.", sum );

	Ok(())
}
