use crate::{ import::* };


/// The error type for errors happening in _async_nursery_.
//
#[ derive( Clone, Copy, PartialEq, Eq, Debug, Error ) ]
//
pub enum NurseErr
{
	/// The executor failed to spawn the provided task.
	//
	#[ error( "The executor failed to spawn the provided task." )]
	//
	Spawn,

	/// The nursery is closed and no longer accepts new tasks.
	//
	#[ error( "The nursery is closed and no longer accepts new tasks." )]
	//
	Closed,
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

