use crate:: { import::* };

/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Debug ) ]
//
pub struct NurseryStream<Out>
{
	rx       : Fuse<UnboundedReceiver<JoinHandle<Out>>> ,
	unordered: FuturesUnordered<JoinHandle<Out>>        ,
}



impl<Out> NurseryStream<Out>
{
	/// Create a new nursery.
	///
	pub fn new( rx: UnboundedReceiver<JoinHandle<Out>> ) -> Self

		where Out: 'static
	{
		let unordered = FuturesUnordered::new();

		Self{ unordered, rx: rx.fuse() }
	}
}



impl<Out> Stream for NurseryStream<Out>

	where Out: 'static
{
	type Item = Out;

	fn poll_next( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>
	{
		debug!( "poll_next called" );

		let mut closed = false;

		// Try to get as many JoinHandles as we can to put them in FuturesUnordered.
		//
		loop
		{
			match Pin::new( &mut self.as_mut().rx ).poll_next(cx)
			{
				Poll::Pending => break,

				Poll::Ready(None) =>
				{
					closed = true;
					break;
				}

				Poll::Ready( Some(handle) ) =>
				{
					self.unordered.push( handle );
				}
			}
		}

		match ready!( Pin::new( &mut self.as_mut().unordered ).poll_next(cx) )
		{
			None        =>
			{
				if closed { Poll::Ready(None) }
				else      { Poll::Pending     }
			}

			out => Poll::Ready(out),
		}
	}


	/// A hint of the number of tasks currently being awaited. There is no upper bound,
	/// because we don't keep track of the tasks that are in the channel between the `Nursery` and
	/// the `NurseryStream`.
	//
	fn size_hint( &self ) -> (usize, Option<usize>)
	{
		// UnboundedReceiver does not have a size hint, so we don't know the upper bound
		// unless we count it ourselves.
		//
		(self.unordered.size_hint().0, None)
	}
}



impl<Out> Future for NurseryStream<Out>

	where Out: 'static

{
	type Output = ();

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		// loop in the case we get an item. Just keep calling until Pending or Done.
		//
		loop
		{
			if ready!( self.as_mut().poll_next(cx) ).is_none()
			{
				trace!( "poll: ready" );
				return Poll::Ready(())
			}
		}

	}
}
