#![ allow( unreachable_code ) ]

use
{
	async_nursery :: { * }
};

type DynError = Box< dyn std::error::Error + Send + Sync + 'static >;


// This basically guarantees that when the connection dies, and this Connection
// object goes away. All the futures currently processing requests will be dropped.
// It doesn't make sense to do work to formulate a response for a connection that
// has died.
//
// Note that there is no integrated support for cooperative cancellation. If dropping
// the futures could leave the system in an inconsistent state, you'll have to implement
// cooperative cancelling in your tasks.
//
// Also you can't start inspecting the output of the tasks if you still want to spawn more.
//
// You can then implement Future or Stream for Connection, which will poll the nursery until all
// subtasks have finished their cleanup.
//
pub struct Connection
{
	nursery: Box< dyn Nurse<Result<(), DynError>> + Send >
}

impl Connection
{
	pub fn process( &self ) -> Result<(), DynError>
	{
		let _disconnect = false;

		while todo!() // let Some( request ) = incoming.next().await
		{
			self.nursery.nurse( async { /*process a request*/ Ok(()) } )?;

			// now if the connection goes away and the Connection object get's
			// dropped, the nursery will be dropped and any pending tasks spawned
			// on it will be dropped, so we don't leak ressources and don't
			// keep processing requests for connections that no longer exist.
			//
			if _disconnect { break }
		}

		Ok(())
	}
}



#[ async_std::main ]
//
async fn main() -> Result<(), DynError>
{
	Ok(())
}
