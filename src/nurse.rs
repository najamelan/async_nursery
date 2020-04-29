use crate::{ import::*, NurseErr };


/// Implementors provide the possiblity to nurse futures. Technically this means
/// you can spawn on this object without the tasks having to return `()` but still
/// you get no [`JoinHandle`](async_executors::JoinHandle).
///
/// Semantically this means it will manage the `JoinHandle`s for you.
///
/// There is a blanket impl extenstion trait [`NurseExt`] so you can spawn futures
/// directly without having to create the `FutureObj` yourself.
//
pub trait Nurse<Out: 'static + Send>
{
	/// Spawn a future and store it's JoinHandle.
	//
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), NurseErr>;
}


/// Extension trait that allows passing in a future directly. Does the conversion to [`FutureObj`]
/// for you.
//
pub trait NurseExt<Out: 'static + Send> : Nurse<Out>
{
	/// Spawn a future and store it's JoinHandle.
	//
	fn nurse( &self, fut: impl Future<Output = Out> + Send + 'static ) -> Result<(), NurseErr>;
}


impl<T, Out> Nurse<Out> for &T

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_obj( future )
	}
}


impl<T, Out> Nurse<Out> for &mut T

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_obj( future )
	}
}


impl<T, Out> Nurse<Out> for Box<T>

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_obj( future )
	}
}


impl<T, Out> Nurse<Out> for Arc<T>

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_obj( future )
	}
}


impl<T, Out> Nurse<Out> for Rc<T>

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_obj( future )
	}
}


impl<T, Out> NurseExt<Out> for T

	where T  : Nurse<Out> + ?Sized ,
	      Out: 'static + Send      ,
{
	fn nurse( &self, future: impl Future<Output = Out> + Send + 'static ) -> Result<(), NurseErr>
	{
		self.nurse_obj( FutureObj::new( Box::new(future) ) )
	}
}

