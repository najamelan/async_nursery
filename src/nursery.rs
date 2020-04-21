use crate:: { import::*, Nurse, NurseryHandle };


/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Debug ) ]
//
pub struct Nursery<S, Out> where S: Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send, Out: 'static + Send
{
	spawner     : S                                               ,
	unordered   : Arc<Mutex< FuturesUnordered<JoinHandle<Out>> >> ,
	tx          : UnboundedSender<JoinHandle<Out>>                ,
	channel     : JoinHandle<()>                                  ,
	stream_waker: Arc<std::sync::Mutex<Option<Waker>>>            ,
	in_flight   : Arc<AtomicUsize>                                ,
}


/// The pinness of the Out parameter shouldn't really define our Unpin status, because
/// we don't really hold it. FuturesUnordered does, but they claim to be unpin in any case.
///
/// We do directly hold S, so we require that to be unpin.
// TODO: test thoroughly our assumptions here and those of FuturesUnordered.
//
impl<S, Out> Unpin for Nursery<S, Out> where S: Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send, Out: 'static + Send {}



impl<S, Out> Nursery<S, Out> where S: Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send, Out: 'static + Send
{
	/// Create a new nursery.
	///
	pub fn new( spawner: S ) -> Result< Self, SpawnError >
	{
		let unordered    = Arc::new(Mutex::new( FuturesUnordered::new() ));
		let in_flight    = Arc::new( AtomicUsize::new(0) );
		let stream_waker = Arc::new( std::sync::Mutex::new( None ) );

		let (tx, mut rx)  = unbounded();
		let unordered2    = unordered.clone();
		let in_flight2    = in_flight.clone();
		let stream_waker2 = stream_waker.clone();

		let listen = async move
		{
			while let Some(handle) = rx.next().await
			{
				warn!( "locking in while rx.next().await loop" );
				unordered2.lock().await.push( handle );
				in_flight2.fetch_sub( 1, SeqCst ); // TODO: checked sub?

				stream_waker2.lock().unwrap().take().map( |w: Waker| { error!( "waking waker" ); w.wake(); } ); // TODO: get rid of unwrap.
				warn!( "unlocking in while rx.next().await loop" );
			}
		};

		let channel = spawner.spawn_handle( listen )?;

		Ok( Self
		{
			spawner   ,
			unordered ,
			tx        ,
			channel   ,
			in_flight ,

			stream_waker,
		})
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

		NurseryHandle::new( self.spawner.clone(), tx, self.in_flight.clone() )
	}
}



impl<S, Out> Nurse<Out> for Nursery<S, Out> where S: Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send, Out: 'static + Send
{
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), SpawnError>
	{
		let handle = self.spawner.spawn_handle_obj( fut )?;

		self.in_flight.fetch_add( 1, SeqCst );

		self.tx.unbounded_send( handle ).unwrap(); // TODO: get rid of unwrap.

		Ok(())
	}
}



impl<S> Spawn for Nursery<S, ()> where S: Unpin + SpawnHandle<()> + Send
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_obj( fut )
	}
}



impl<S, Out> Stream for Nursery<S, Out>

	where S: Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send, Out: 'static + Send
{
	type Item = Out;

	fn poll_next( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>
	{
		debug!( "poll_next called" );

		let this = self.get_mut();

		let poll_stream =
		{
			warn!( "locking in poll_next" );

			match Pin::new( &mut this.unordered.lock() ).poll( cx )
			{
				Poll::Ready(mut guard) =>
				{
					let result = Pin::new( &mut *guard ).poll_next( cx );
					warn!( "unlocking in poll_next" );
					result
				}

				Poll::Pending => { warn!( "failed to lock in poll_next, return pending" ); return Poll::Pending },
			}
		};


		match poll_stream
		{
			Poll::Ready( None ) =>
			{
				// if none in flight, return None, otherwise return Pending and wake the task later.
				//
				match this.in_flight.load( SeqCst )
				{
					0 => { debug!( "return None from stream" ); Poll::Ready( None ) }

					_ =>
					{
						this.stream_waker.lock().unwrap().replace( cx.waker().clone() ); // TODO: get rid of unwrap.
						error!( "storing waker" );
						Poll::Pending
					}
				}
			}

			Poll::Ready(some) => { debug!( "return some from stream" ); Poll::Ready(some) },
			Poll::Pending     => { debug!( "return pending from stream" ); Poll::Pending },
		}
	}

	fn size_hint( &self ) -> (usize, Option<usize>)
	{
		block_on( self.unordered.lock() ).size_hint() // TODO: get rid of block_on
	}
}
