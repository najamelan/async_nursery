use crate::{ import::*, NurseErr };

/// Same as [`Nurse`] but doesn't require the futures to be [`Send`].
//
pub trait LocalNurse<Out: 'static>
{
	/// Spawn a `!Send` future and store it's JoinHandle.
	//
	fn nurse_local_obj( &self, fut: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>;
}

/// Extension trait that allows passing in a future directly. Does the conversion to [`LocalFutureObj`]
/// for you.
//
pub trait LocalNurseExt<Out: 'static> : LocalNurse<Out>
{
	/// Spawn a `!Send` future and store it's JoinHandle.
	//
	fn nurse_local( &self, fut: impl Future<Output = Out> + 'static ) -> Result<(), NurseErr>;
}


impl<T, Out> LocalNurse<Out> for &T

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(*self).nurse_local_obj( future )
	}
}


impl<T, Out> LocalNurse<Out> for &mut T

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_local_obj( future )
	}
}


impl<T, Out> LocalNurse<Out> for Box<T>

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_local_obj( future )
	}
}


impl<T, Out> LocalNurse<Out> for Arc<T>

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_local_obj( future )
	}
}


impl<T, Out> LocalNurse<Out> for Rc<T>

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		(**self).nurse_local_obj( future )
	}
}


impl<T, Out> LocalNurseExt<Out> for T

	where T  : LocalNurse<Out> + ?Sized ,
	      Out: 'static                  ,
{
	fn nurse_local( &self, future: impl Future<Output = Out> + 'static ) -> Result<(), NurseErr>
	{
		self.nurse_local_obj( LocalFutureObj::new( Box::new(future) ) )
	}
}

