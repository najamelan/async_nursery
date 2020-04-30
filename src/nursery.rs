use crate:: { import::*, Nurse, LocalNurse, NurseErr, NurseryStream };


/// The sender part of the nursery. Wraps an unbounded sender. Can be cloned.
/// To manage the spawned tasks and await their output, see [`NurseryStream`].
///
/// Will disconnect on drop. You can close all senders by calling `close_nursery`.
///
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



