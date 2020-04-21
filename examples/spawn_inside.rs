use
{
	async_executors :: { * } ,
	async_nursery   :: { * } ,
	log             :: { * } ,
	futures         :: { StreamExt }
};

type DynError = Box< dyn std::error::Error + Send + Sync + 'static >;



// This function will spawn tasks, but guarantee that when it returns all
// concurrent tasks have been joined.
//
// This could in principle be achieved with things like `join!`, but it
// quickly becomes unwieldy if it has to scale. This nursery can be passed
// down arbitrarily into function calls to add further concurrent operations.
//
async fn spawns_inside() -> Result<usize, DynError>
{
	let nursery = Nursery::new( AsyncStd )?;
	debug!( "nursery created" );
	nursery.nurse( produce_value () )?; 	debug!( "spawn produce_value" );
	nursery.nurse( produce_value2() )?;	   debug!( "spawn produce_value2" );


	// Pass the nursery down the stack. Could also be async and be awaited.
	// In this particular case we can't nurse it, because the nursery takes
	// futures that return a usize, which is not the return type of produce_value3.
	// The most generic way of using the nursery is to return Result<(), DynError>
	// and not use it for returning values.
	//
	produce_value3( &nursery )?; debug!( "call produce_value3" );

	Ok( nursery.fold(0, |acc, x| async move
	{
		debug!( "fold, acc: {}", acc );
		acc + x

	} ).await )

	// If we do not care about the return values, but just want to make sure
	// everything has finished we could do something like:
	//
	// while nursery.next().await.is_some() {}

	// Or leveraging the functionality of TryStreamExt to bail out if an error
	// happens when the nursery's out parameter is a Result.
	//
	// This will cancel the whole operation and return early when an error happens.
	// When the nursery get's dropped, all the tasks in it get dropped. No ressources
	// are leaked after this function ends.
	//
	// while nursery.try_next().await?.is_some() {}
}


async fn produce_value () -> usize {  5 }
async fn produce_value2() -> usize { 10 }

fn produce_value3( nursery: &(impl Nurse<usize> + Send + 'static) ) -> Result<(), DynError>
{
	nursery.nurse( produce_value () )?; debug!( "spawn produce_value in produce_value3" );
	nursery.nurse( produce_value2() )?; debug!( "spawn produce_value2 in produce_value3" );
	Ok(())
}



#[ async_std::main ]
//
async fn main() -> Result<(), DynError>
{
	// flexi_logger::Logger::with_str( "trace, async_std=warn" ).start().unwrap();

	loop
	{
		let sum = spawns_inside().await?;

		assert_eq!( sum, 30 );

		println!( "Total of all concurrent operations is: {}.", sum );
	}

	Ok(())
}
