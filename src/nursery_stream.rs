use crate:: { import::* };

/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Debug ) ]
//
pub struct NurseryStream<Out>
{
	channel     : JoinHandle<()>                                     ,
	unordered   : Arc<FutMutex< FuturesUnordered<JoinHandle<Out>> >> ,
	stream_waker: Arc<Mutex<Option<Waker>>>                          ,
	in_flight   : Arc<AtomicUsize>                                   ,
	closed      : Arc<AtomicBool>                                    ,
}




impl<Out> NurseryStream<Out>
{
	/// Create a new nursery.
	///
	pub fn new
	(
		spawner     : &impl SpawnHandle<()>              ,
		rx          : UnboundedReceiver<JoinHandle<Out>> ,
		in_flight   : Arc<AtomicUsize>                   ,
		stream_waker: Arc<Mutex<Option<Waker>>>          ,
	)

		-> Result< Self, SpawnError >

		where Out: 'static + Send
	{
		let unordered = Arc::new( FutMutex::new( FuturesUnordered::new() ) );
		let closed    = Arc::new( AtomicBool::new( false ) );
		let listen    = Self::listen( unordered.clone(), stream_waker.clone(), in_flight.clone(), rx, closed.clone() );
		let channel   = spawner.spawn_handle( listen )?;

		Ok( Self{ unordered, channel, in_flight, closed, stream_waker } )
	}

	/// Create a new nursery.
	///
	pub fn new_local
	(
		spawner     : &impl LocalSpawnHandle<()>         ,
		rx          : UnboundedReceiver<JoinHandle<Out>> ,
		in_flight   : Arc<AtomicUsize>                   ,
		stream_waker: Arc<Mutex<Option<Waker>>>          ,
	)

		-> Result< Self, SpawnError >

		where Out: 'static
	{
		let unordered = Arc::new( FutMutex::new( FuturesUnordered::new() ) );
		let closed    = Arc::new( AtomicBool::new( false ) );
		let listen    = Self::listen( unordered.clone(), stream_waker.clone(), in_flight.clone(), rx, closed.clone() );
		let channel   = spawner.spawn_handle_local( listen )?;

		Ok( Self{ unordered, channel, in_flight, closed, stream_waker } )
	}


	async fn listen
	(
		unordered   : Arc<FutMutex< FuturesUnordered<JoinHandle<Out>> >>,
		stream_waker: Arc<Mutex<Option<Waker>>>,
		in_flight   : Arc<AtomicUsize>,
		mut rx      : UnboundedReceiver<JoinHandle<Out>>,
		closed      : Arc<AtomicBool>                    ,
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

		closed.store( true, SeqCst );
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



impl<Out> Stream for NurseryStream<Out>

	where Out: 'static + Send
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
