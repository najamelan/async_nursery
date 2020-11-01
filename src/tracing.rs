use
{
	crate           :: { Nurse, LocalNurse, NurseErr, import::* } ,
	tracing_futures :: { Instrument, Instrumented, WithDispatch } ,
	futures         :: { FutureExt                              } ,
};



impl<T, Out> Nurse<Out> for Instrumented<T> where T: Nurse<Out>, Out: 'static + Send
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().nurse_obj( FutureObj::new(fut.boxed()) )
	}
}



impl<T, Out> Nurse<Out> for WithDispatch<T> where T: Nurse<Out>, Out: 'static + Send
{
	fn nurse_obj( &self, future: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		let fut = self.with_dispatch( future );

		self.inner().nurse_obj( FutureObj::new(fut.boxed()) )
	}
}



impl<T, Out> LocalNurse<Out> for Instrumented<T> where T: LocalNurse<Out>, Out: 'static
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().nurse_local_obj( LocalFutureObj::new(fut.boxed_local()) )
	}
}




impl<T, Out> LocalNurse<Out> for WithDispatch<T> where T: LocalNurse<Out>, Out: 'static
{
	fn nurse_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		let fut = self.with_dispatch(future);

		self.inner().nurse_local_obj( LocalFutureObj::new(fut.boxed_local()) )
	}
}
