use crate:: { import::*, Nurse, LocalNurse, NurseErr };


/// A handle on which you can spawn tasks that will be sent to the Nursery.
///
#[ derive( Debug ) ]
//
pub struct NurseryHandle<S, Out>
{
	tx       : UnboundedSender<JoinHandle<Out>> ,
	spawner  : S                                ,
	in_flight: Arc<AtomicUsize>                 ,
	closed   : Arc<AtomicBool>                  ,
}


impl<S, Out> NurseryHandle<S, Out>
{
	pub(crate) fn new( spawner: S, tx: UnboundedSender<JoinHandle<Out>>, in_flight: Arc<AtomicUsize>, closed: Arc<AtomicBool> ) -> Self
	{
		Self { spawner, tx, in_flight, closed }
	}
}


impl<S, Out> Nurse<Out> for NurseryHandle<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
{
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ) }

		let handle = self.spawner.spawn_handle_obj( fut )?;

		self.in_flight.fetch_add( 1, SeqCst );

		self.tx.unbounded_send( handle )?;
		Ok(())
	}
}



impl<S, Out> LocalNurse<Out> for NurseryHandle<S, Out> where S: LocalSpawnHandle<Out>, Out: 'static
{
	fn nurse_local_obj( &self, fut: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ) }

		let handle = self.spawner.spawn_handle_local_obj( fut )?;

		self.in_flight.fetch_add( 1, SeqCst );

		self.tx.unbounded_send( handle )?;
		Ok(())
	}
}



impl<S> LocalSpawn for NurseryHandle<S, ()> where S: LocalSpawnHandle<()> + Clone
{
	fn spawn_local_obj( &self, fut: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_local_obj( fut ).map_err( |_| SpawnError::shutdown() )
	}
}



impl<S> Spawn for NurseryHandle<S, ()> where S: SpawnHandle<()>
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_obj( fut ).map_err( |_| SpawnError::shutdown() )
	}
}



impl<S, Out> Clone for NurseryHandle<S, Out> where S: Clone
{
	fn clone( &self ) -> Self
	{
		Self
		{
			tx        : self.tx        .clone() ,
			spawner   : self.spawner   .clone() ,
			in_flight : self.in_flight .clone() ,
			closed    : self.closed    .clone() ,
		}
	}
}



impl<S, Out> Sink<FutureObj<'static, Out>> for NurseryHandle<S, Out>

	where S: SpawnHandle<Out>, Out: 'static + Send

{
	type Error = NurseErr;

	fn poll_ready( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ).into() }

		Poll::Ready( Ok(()) )
	}


	fn start_send( self: Pin<&mut Self>, fut: FutureObj<'static, Out> ) -> Result<(), Self::Error>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ).into() }

		self.nurse_obj( fut )
	}


	fn poll_flush( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready( Ok(()) )
	}


	/// This is a no-op. The address can only really close when dropped. Close has no meaning before that.
	//
	fn poll_close( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		// TODO: wait for in_flight?
		//
		self.closed.store( true, SeqCst );

		Poll::Ready( Ok(()) )
	}
}



impl<S, Out> Sink<LocalFutureObj<'static, Out>> for NurseryHandle<S, Out>

	where S: LocalSpawnHandle<Out>, Out: 'static

{
	type Error = NurseErr;

	fn poll_ready( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ).into() }

		Poll::Ready( Ok(()) )
	}


	fn start_send( self: Pin<&mut Self>, fut: LocalFutureObj<'static, Out> ) -> Result<(), Self::Error>
	{
		if self.closed.load( SeqCst ) { return Err( NurseErr::Closed ).into() }

		self.nurse_local_obj( fut )
	}


	fn poll_flush( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready( Ok(()) )
	}


	/// This is a no-op. The address can only really close when dropped. Close has no meaning before that.
	//
	fn poll_close( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		self.closed.store( true, SeqCst );

		Poll::Ready( Ok(()) )
	}
}
