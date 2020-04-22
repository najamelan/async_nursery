use crate:: { import::*, Nurse, LocalNurse, NurseErr, NurseryStream };


/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Clone, Debug ) ]
//
pub struct Nursery<S, Out>
{
	spawner     : S                                ,
	tx          : UnboundedSender<JoinHandle<Out>> ,
	in_flight   : Arc<AtomicUsize>                 ,
	closed      : Arc<AtomicBool>                  ,
}



impl<S, Out> Nursery<S, Out>
{

	/// Create a new nursery.
	///
	pub fn new( spawner: S ) -> Result< (Self, NurseryStream<Out>), SpawnError >

		where S: SpawnHandle<()>, Out: 'static + Send
	{
		let in_flight    = Arc::new( AtomicUsize::new(0) );
		let closed       = Arc::new( AtomicBool::new( false ) );
		let stream_waker = Arc::new( Mutex::new( None ) );
		let (tx, rx)     = unbounded();

		let stream = NurseryStream::new
		(
			&spawner          ,
			rx                ,
			in_flight.clone() ,
			stream_waker      ,
			closed.clone()    ,
		)?;

		Ok
		((
			Self{ spawner, tx, in_flight, closed } ,
			stream                                 ,
		))
	}



	/// Create a new nursery.
	///
	pub fn new_local( spawner: S ) -> Result< (Self, NurseryStream<Out>), SpawnError >

		where S: LocalSpawnHandle<()>, Out: 'static
	{
		let in_flight    = Arc::new( AtomicUsize::new(0) );
		let closed       = Arc::new( AtomicBool::new( false ) );
		let stream_waker = Arc::new( Mutex::new( None ) );
		let (tx, rx)     = unbounded();

		let stream = NurseryStream::new_local
		(
			&spawner          ,
			rx                ,
			in_flight.clone() ,
			stream_waker      ,
			closed.clone()    ,
		)?;

		Ok
		((
			Self{ spawner, tx, in_flight, closed } ,
			stream                                 ,
		))
	}


	/// Stop accepting new futures. You need to call this for the stream to finish.
	/// The same effect can be achieved by calling `SinkExt::close`, however since that is
	/// an async fn, this method is provided for convenience.
	//
	pub fn stop( &self )
	{
		self.closed.store( true, SeqCst );
	}
}



impl<S, Out> Nurse<Out> for Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
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



impl<S, Out> LocalNurse<Out> for Nursery<S, Out> where S: LocalSpawnHandle<Out>, Out: 'static
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



impl<S> Spawn for Nursery<S, ()> where S: SpawnHandle<()>
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_obj( fut ).map_err( |_| SpawnError::shutdown() )
	}
}



impl<S> LocalSpawn for Nursery<S, ()> where S: LocalSpawnHandle<()>
{
	fn spawn_local_obj( &self, fut: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_local_obj( fut ).map_err( |_| SpawnError::shutdown() )
	}
}



impl<S, Out> Sink<FutureObj<'static, Out>> for Nursery<S, Out>

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
		self.stop();

		Poll::Ready( Ok(()) )
	}
}



impl<S, Out> Sink<LocalFutureObj<'static, Out>> for Nursery<S, Out>

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
		self.stop();

		Poll::Ready( Ok(()) )
	}
}



