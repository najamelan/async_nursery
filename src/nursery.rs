use crate:: { import::*, Nurse };


/// A nursery allows you to spawn futures yet adhere to structured concurrency principles.
///
#[ derive( Debug ) ]
//
pub struct Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
{
	spawner  : S                                 ,
	unordered: FuturesUnordered<JoinHandle<Out>> ,
}

impl<S, Out> Unpin for Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send {}



impl<S, Out> Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
{
	/// Create a new nursery.
	///
	pub fn new( spawner: S ) -> Self
	{
		Self
		{
			spawner                            ,
			unordered: FuturesUnordered::new() ,
		}
	}
}


impl<S, Out> Nurse<Out> for Nursery<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
{
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), SpawnError>
	{
		self.unordered.push( self.spawner.spawn_handle_obj( fut )? );

		Ok(())
	}
}

impl<S> Spawn for Nursery<S, ()> where S: SpawnHandle<()>
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.unordered.push( self.spawner.spawn_handle_obj( fut )? );

		Ok(())
	}
}



impl<S, Out> Stream for Nursery<S, Out>

	where S: SpawnHandle<Out>, Out: 'static + Send
{
	type Item = Out;

	fn poll_next( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>
	{
		Pin::new( &mut self.as_mut().unordered ).poll_next( cx )
	}

	fn size_hint( &self ) -> (usize, Option<usize>)
	{
		self.unordered.size_hint()
	}
}
