use crate::{ import::* };


/// The error type for errors happening in _async_nursery_.
//
#[ derive( Clone, Copy, PartialEq, Eq, Debug ) ]
//
pub enum NurseErr
{
	/// The executor failed to spawn the provided task. This means the executor returned an error.
	//
	Spawn,

	/// The nursery is closed and no longer accepts new tasks.
	//
	Closed,
}



impl std::error::Error for NurseErr {}


impl std::fmt::Display for NurseErr
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		match &self
		{
			NurseErr::Spawn =>

				write!( f, "The executor failed to spawn the provided task." ),

			NurseErr::Closed =>

				write!( f, "The nursery is closed and no longer accepts new tasks." ),
		}
	}
}




impl From< SpawnError > for NurseErr
{
	fn from( _: SpawnError ) -> NurseErr
	{
		NurseErr::Spawn
	}
}



impl<T> From< TrySendError<T> > for NurseErr
{
	fn from( _: TrySendError<T> ) -> NurseErr
	{
		NurseErr::Closed
	}
}



impl From< NurseErr > for Box<dyn std::any::Any + Send>
{
	fn from( err: NurseErr ) -> Box<dyn std::any::Any + Send>
	{
		Box::new( err )
	}
}

