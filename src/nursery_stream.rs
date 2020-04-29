use crate:: { import::* };

/// Collection of [`JoinHandle`]s of tasks spawned on the nursery. When this is dropped,
/// all spawned tasks are canceled. You can poll the [`Stream`] implementation on this
/// to obtain the outputs of your tasks. You can await the [`Future`] implementation if
/// you don't care about the outputs but just want to wait until all spawned tasks are done.
///
#[ derive( Debug ) ]
//
pub struct NurseryStream<Out>
{
	rx       : UnboundedReceiver<JoinHandle<Out>> ,
	unordered: FuturesUnordered<JoinHandle<Out>>  ,
	rx_closed: bool                               ,
}



impl<Out> NurseryStream<Out>
{
	/// Create a new nursery.
	///
	pub(crate) fn new( rx: UnboundedReceiver<JoinHandle<Out>> ) -> Self

		where Out: 'static
	{
		let unordered = FuturesUnordered::new();

		Self
		{
			unordered         ,
			rx                ,
			rx_closed: false  ,
		}
	}


	/// Close this NurseryStream. Related [`Nursery`](crate::Nursery) will no longer be able to
	/// spawn. This allows the stream to end. Alternatively you can drop all related
	/// [`Nursery`](crate::Nursery) or call [`Nursery::close_nursery`](crate::Nursery::close_nursery).
	///
	pub fn close_nursery( &mut self )
	{
		self.rx.close();
	}
}



impl<Out> Stream for NurseryStream<Out>

	where Out: 'static
{
	type Item = Out;

	fn poll_next( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>
	{
		// Try to get as many JoinHandles as we can to put them in FuturesUnordered.
		//
		if !self.rx_closed { loop
		{
			match Pin::new( &mut self.as_mut().rx ).poll_next(cx)
			{
				Poll::Pending => break,

				Poll::Ready(None) =>
				{
					self.rx_closed = true;
					break;
				}

				Poll::Ready( Some(handle) ) =>
				{
					self.unordered.push( handle );
				}
			}
		}}

		match ready!( Pin::new( &mut self.as_mut().unordered ).poll_next(cx) )
		{
			None =>
			{
				if self.rx_closed
				{
					Poll::Ready(None)
				}

				else
				{
					Poll::Pending
				}
			}

			out =>
			{
				Poll::Ready(out)
			}
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
				return Poll::Ready(())
			}
		}

	}
}



impl<Out> FusedFuture for NurseryStream<Out>

	where Out: 'static

{
	fn is_terminated(&self) -> bool
	{
		self.rx_closed && self.unordered.is_terminated()
	}
}



impl<Out> FusedStream for NurseryStream<Out>

	where Out: 'static

{
	fn is_terminated(&self) -> bool
	{
		self.rx_closed && self.unordered.is_terminated()
	}
}
