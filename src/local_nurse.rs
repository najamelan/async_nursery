use crate::import::*;

/// Same as [`Nurse`] but doesn't require the futures to be [`Send`].
//
pub trait LocalNurse<Out: 'static>
{
	/// Spawn a `!Send` future and store it's JoinHandle.
	//
	fn nurse_local_obj( &self, fut: LocalFutureObj<'static, Out> ) -> Result<(), SpawnError>;
}

/// Extension trait that allows passing in a future directly. Does the conversion to [`LocalFutureObj`]
/// for you.
//
pub trait LocalNurseExt<Out: 'static> : LocalNurse<Out>
{
	/// Spawn a `!Send` future and store it's JoinHandle.
	//
	fn nurse_local( &self, fut: impl Future<Output = Out> + 'static ) -> Result<(), SpawnError>;
}


impl<T, Out> LocalNurseExt<Out> for T

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static             ,
{
	fn nurse_local( &self, future: impl Future<Output = Out> + 'static ) -> Result<(), SpawnError>
	{
		self.nurse_local_obj( LocalFutureObj::new( future.boxed_local() ) )
	}
}
