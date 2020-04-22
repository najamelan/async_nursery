use crate::import::*;


/// Implementors provide the possiblity to nurse futures. This means that they
/// accept futures with the `nurse` method and implement `Stream` over the output
/// type of the futures.
//
pub trait Nurse<Out: 'static + Send>
{
	/// Spawn a future and store it's JoinHandle.
	//
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), SpawnError>;
}


/// Extension trait that allows passing in a future directly. Does the conversion to [`LocalFutureObj`]
/// for you.
//
pub trait NurseExt<Out: 'static + Send> : Nurse<Out>
{
	/// Spawn a future and store it's JoinHandle.
	//
	fn nurse( &self, fut: impl Future<Output = Out> + Send + 'static ) -> Result<(), SpawnError>;
}


impl<T, Out> NurseExt<Out> for T

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse( &self, future: impl Future<Output = Out> + Send + 'static ) -> Result<(), SpawnError>
	{
		self.nurse_obj( FutureObj::new( future.boxed() ) )
	}
}

