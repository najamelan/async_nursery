use crate:: { import::*, Nurse, LocalNurse, NurseryHandle, NurseErr };


/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Debug ) ]
//
pub struct Nursery<S, Out>
{
	spawner     : S                                                  ,
	tx          : UnboundedSender<JoinHandle<Out>>                   ,
	channel     : JoinHandle<()>                                     ,
	unordered   : Arc<FutMutex< FuturesUnordered<JoinHandle<Out>> >> ,
	stream_waker: Arc<Mutex<Option<Waker>>>                          ,
	in_flight   : Arc<AtomicUsize>                                   ,
	closed      : Arc<AtomicBool>                                    ,
}



impl<S, Out> Nursery<S, Out>
{
	/// Create a new nursery.
	///
	pub fn new( spawner: S ) -> Result< Self, SpawnError >

		where S: 'static + SpawnHandle<()>, Out: 'static + Send
	{
		let unordered    = Arc::new( FutMutex::new( FuturesUnordered::new() ) );
		let in_flight    = Arc::new( AtomicUsize::new(0) );
		let closed       = Arc::new( AtomicBool::new( false ) );
		let stream_waker = Arc::new( Mutex::new( None ) );
		let (tx, rx)     = unbounded();

		let listen = Self::listen( unordered.clone(), stream_waker.clone(), in_flight.clone(), rx );

		let channel = spawner.spawn_handle( listen )?;

		Ok( Self
		{
			spawner     ,
			unordered   ,
			tx          ,
			channel     ,
			in_flight   ,
			closed      ,
			stream_waker,
		})
	}

	/// Create a new nursery.
	///
	pub fn new_local( spawner: S ) -> Result< Self, SpawnError >

		where S: 'static + LocalSpawnHandle<()>, Out: 'static
	{
		let unordered    = Arc::new( FutMutex::new( FuturesUnordered::new() ) );
		let in_flight    = Arc::new( AtomicUsize::new(0) );
		let closed       = Arc::new( AtomicBool::new( false ) );
		let stream_waker = Arc::new( Mutex::new( None ) );
		let (tx, rx)     = unbounded();

		let listen = Self::listen( unordered.clone(), stream_waker.clone(), in_flight.clone(), rx );

		let channel = spawner.spawn_handle_local( listen )?;

		Ok( Self
		{
			spawner     ,
			unordered   ,
			tx          ,
			channel     ,
			in_flight   ,
			closed      ,
			stream_waker,
		})
	}


	async fn listen
	(
		unordered   : Arc<FutMutex< FuturesUnordered<JoinHandle<Out>> >>,
		stream_waker: Arc<Mutex<Option<Waker>>>,
		in_flight   : Arc<AtomicUsize>,
		mut rx      : UnboundedReceiver<JoinHandle<Out>>,
	)
	{
		while let Some(handle) = rx.next().await
		{
			{
				warn!( "--> locking in while rx.next().await loop" );
				unordered.lock().await.push( handle );
				warn!( "<-- unlocked in while rx.next().await loop" );
			}

			// TODO: checked sub?
			// there are no provided checked operations for atomics. But bad things will happen here if this overflows...
			// for now, add an assert.
			//
			let check = in_flight.fetch_sub( 1, SeqCst );
			assert!( check > 0 );


			stream_waker.lock().take().map( |w: Waker| { error!( "waking waker" ); w.wake(); } ); // TODO: get rid of unwrap.
			warn!( "end of while rx.next().await loop" );
		}
	}


	/// Obtain a handle that can be used to spawn on this nursery. This allows
	/// passing the handle into subtasks that are spawned on this nursery as those
	/// cannot take a referenc (have to be `'static`).
	///
	/// When spawning on the handle, if the nursery no longer exists you will
	/// get an error.
	//
	pub fn handle( &mut self ) -> NurseryHandle<S, Out> where S: Clone
	{
 		// We should always have a tx, so unwrap should be fine.
 		//
 		let tx = self.tx.clone();

		NurseryHandle::new( self.spawner.clone(), tx, self.in_flight.clone(), self.closed.clone() )
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



impl<S, Out> Stream for Nursery<S, Out>

	where S: Unpin, Out: 'static + Send
{
	type Item = Out;

	fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>
	{
		debug!( "poll_next called" );

		let this = self.get_mut();
		let in_flight;

		let poll_stream =
		{
			debug!( "locking in poll_next" );

			// futures mutex will drop our waker with the future we got from the lock() method,
			// so we have to manually wake up. When the pusher is done with the lock it anyways
			// checks to see if there is a waker, so just set it before we try to unlock.
			//
			this.stream_waker.lock().replace( cx.waker().clone() );

			// Since they won't wake us up, poll is useless, just use try_lock. If we were pre-empted
			// just before this, it would be fine, since we are in a &mut self method, so there is no
			// other code elsewhere running this, and the wake will make sure we will get called another
			// time. If the try_lock succeeds, we unset the waker we just set.
			//
			match this.unordered.try_lock()
			{
				Some(mut guard) =>
				{
					*this.stream_waker.lock() = None;

					// We have to check it before we poll futures unordered. Otherwise it might
					// be set to zero between this poll and the moment we check and act on it.
					//
					in_flight = this.in_flight.load( SeqCst );

					// Now we have the guard, so if this returns pending, FuturesUnordered
					// will wake us up.
					//
					let result = Pin::new( &mut *guard ).poll_next( cx );
					debug!( "unlocking in poll_next" );
					result
				}

				None =>
				{
					debug!( "failed to lock in poll_next, return pending" );

					return Poll::Pending
				},
			}
		};


		match poll_stream
		{
			Poll::Ready( None ) =>
			{
				// if none in flight, return None, otherwise return Pending and wake the task later.
				// We have to check both the value of in_flight just before we polled FuturesUnordered
				// and now, because we might have been pre-empted at an inopportune moment and have the
				// value changed from underneath us.
				//
				if    0    == in_flight
				   && 0    == this.in_flight.load( SeqCst )
				   && true == this.closed   .load( SeqCst )

				{
					debug!( "return None from stream" );
					Poll::Ready( None )
				}

				else
				{
					// FuturesUnordered returned None even though there was still in flight tasks.
					// Just spin.
					//
					cx.waker().wake_by_ref();

					Poll::Pending
				}
			}

			Poll::Ready(some) =>
			{
				// Unset any wakers, since we got an item.
				//
				*this.stream_waker.lock() = None;
				debug!( "return some from stream" ); Poll::Ready(some)
			},

			Poll::Pending =>
			{
				debug!( "return pending from stream" );

				Poll::Pending
			},
		}
	}


	/// This can deadlock!
	//
	fn size_hint( &self ) -> (usize, Option<usize>)
	{
		block_on( self.unordered.lock() ).size_hint() // TODO: get rid of block_on
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
		self.closed.store( true, SeqCst );

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
		self.closed.store( true, SeqCst );

		Poll::Ready( Ok(()) )
	}
}



