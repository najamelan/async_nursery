use crate:: { import::*, Nurse, LocalNurse, NurseErr, NurseryStream };


/// The sender part of the nursery. Wraps an unbounded sender. Can be cloned.
/// To manage the spawned tasks and await their output, see [`NurseryStream`].
///
/// Will disconnect on drop. You can close all senders by calling `close_nursery`.
///
/// Will implement async_executor traits if the executor does. Forwards [`Timer`], [`TokioIo`],
/// [`YieldNow`] and [`SpawnBlocking`]. Note that the nursery doesn't actually manage the
/// tasks spawned via `SpawnBlocking`. It just let's you use that functionality of the wrapped
/// executor.
//
#[ cfg_attr( nightly, doc(cfg( feature = "implementation" )) ) ]
//
#[ derive( Debug ) ]
//
pub struct Nursery<S, Out>
{
	spawner     : S                                ,
	tx          : UnboundedSender<JoinHandle<Out>> ,
}


impl<S, Out> Clone for Nursery<S, Out> where S: Clone
{
	fn clone( &self ) -> Self
	{
		Self
		{
			spawner: self.spawner.clone() ,
			tx     : self.tx     .clone() ,
		}
	}
}



impl<S, Out> Nursery<S, Out>
{
	/// Create a new nursery. Returns a tuple of the sender part
	/// and the stream of outputs.
	///
	pub fn new( spawner: S ) -> (Self, NurseryStream<Out>)

		where Out: 'static
	{
		let (tx, rx) = unbounded();

		(
			Self{ spawner, tx }      ,
			NurseryStream::new( rx ) ,
		)
	}


	/// When dealing with an API that takes `SpawnHandle` and returns you a `JoinHandle`, you can use this
	/// method to add the `JoinHandle` to your nursery.
	//
	pub fn nurse_handle( &self, handle: JoinHandle<Out> ) -> Result<(), NurseErr>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ) }

		self.tx.unbounded_send( handle )?;

		Ok(())
	}


	/// Stop this nursery and any clones from accepting any more tasks. Calling this or
	/// dropping all `Nursery` is necessary for the stream impl of `NurseryStream` to end
	/// and return `None`.
	//
	pub fn close_nursery( &self )
	{
		self.tx.close_channel();
	}
}



impl<S, Out> Nurse<Out> for Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
{
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ) }

		let handle = self.spawner.spawn_handle_obj( fut )?;

		self.tx.unbounded_send( handle )?;

		Ok(())
	}
}



impl<S, Out> LocalNurse<Out> for Nursery<S, Out> where S: LocalSpawnHandle<Out>, Out: 'static
{
	fn nurse_local_obj( &self, fut: LocalFutureObj<'static, Out> ) -> Result<(), NurseErr>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ) }

		let handle = self.spawner.spawn_handle_local_obj( fut )?;

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
		if self.tx.is_closed() { return Err( NurseErr::Closed ).into() }

		Poll::Ready( Ok(()) )
	}


	fn start_send( self: Pin<&mut Self>, fut: FutureObj<'static, Out> ) -> Result<(), Self::Error>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ) }

		self.nurse_obj( fut )
	}


	fn poll_flush( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready( Ok(()) )
	}


	/// This is a no-op. If you want to disconnect, just drop this `Nursery`. If you want to
	/// close the NurseryStream, call [`Nursery::close_nursery`].
	//
	fn poll_close( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		self.close_nursery();

		Poll::Ready( Ok(()) )
	}
}



impl<S, Out> Sink<LocalFutureObj<'static, Out>> for Nursery<S, Out>

	where S: LocalSpawnHandle<Out>, Out: 'static

{
	type Error = NurseErr;

	fn poll_ready( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ).into() }

		Poll::Ready( Ok(()) )
	}


	fn start_send( self: Pin<&mut Self>, fut: LocalFutureObj<'static, Out> ) -> Result<(), Self::Error>
	{
		if self.tx.is_closed() { return Err( NurseErr::Closed ) }

		self.nurse_local_obj( fut )
	}


	fn poll_flush( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		Poll::Ready( Ok(()) )
	}


	/// This is a no-op. If you want to disconnect, just drop this `Nursery`. If you want to
	/// close the NurseryStream, call [`Nursery::close_nursery`].
	//
	fn poll_close( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Result<(), Self::Error>>
	{
		self.close_nursery();

		Poll::Ready( Ok(()) )
	}
}



impl<S, Out> Timer for Nursery<S, Out> where S: Timer
{
	fn sleep( &self, dur: Duration ) -> BoxFuture<'static, ()>
	{
		self.spawner.sleep( dur )
	}
}



impl<S, Out> TokioIo for Nursery<S, Out> where S: TokioIo {}



impl<S, Out, R> SpawnBlocking<R> for Nursery<S, Out> where R: Send + 'static, S: SpawnBlocking<R>
{
	fn spawn_blocking<F>( &self, f: F ) -> BlockingHandle<R>

		where F   : FnOnce() -> R + Send + 'static ,
	         Self: Sized                          ,
	{
		self.spawner.spawn_blocking(f)
	}


	fn spawn_blocking_dyn( &self, f: Box< dyn FnOnce()->R + Send > ) -> BlockingHandle<R>
	{
		self.spawner.spawn_blocking_dyn(f)
	}
}


impl<S, Out> YieldNow for Nursery<S, Out> where S: YieldNow
{
	fn yield_now( &self ) -> YieldNowFut
	{
		self.spawner.yield_now()
	}
}
