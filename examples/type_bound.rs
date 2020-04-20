#![ allow( unreachable_code ) ]

use
{
	async_nursery :: { * }
};

type DynError = Box< dyn std::error::Error + Send + Sync + 'static >;


// This basically guarantees that when the connection dies, and this HttpConnection
// object goes away. All the futures currently processing requests will be dropped.
// It doesn't make sense to do work to formulate a response for a connection that
// has died.
//
// Note that there is no integrated support for cooperative cancellation. If dropping
// the futures could leave the system in an inconsistent state, you'll have to implement
// cooperative cancelling in your tasks.
//
// You can then implement Future for HttpConnection, which will poll the nursery until all
// subtasks have finished their cleanup.
//
pub struct HttpConnection
{
	nursery: Box< dyn Nurse<Result<(), DynError>> + Send >
}

impl HttpConnection
{
	pub fn process( &self ) -> Result<(), DynError>
	{
		loop // generally while loop over incoming messages...
		{
			self.nursery.nurse( async { /*process a request*/ Ok(()) } )?;
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
