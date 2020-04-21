use crate:: { import::*, Nurse };


/// A handle on which you can spawn tasks that will be sent to the Nursery.
///
#[ derive( Debug ) ]
//
pub struct NurseryHandle<S, Out> where S: SpawnHandle<Out> + Clone + Send, Out: 'static + Send
{
	tx: UnboundedSender<JoinHandle<Out>>,
	spawner: S,
	in_flight: Arc<AtomicUsize>,
}


impl<S, Out> NurseryHandle<S, Out> where S: SpawnHandle<Out> + Clone + Send, Out: 'static + Send
{
	pub(crate) fn new( spawner: S, tx: UnboundedSender<JoinHandle<Out>>, in_flight: Arc<AtomicUsize> ) -> Self
	{
		Self { spawner, tx, in_flight }
	}
}


impl<S, Out> Nurse<Out> for NurseryHandle<S, Out> where S: SpawnHandle<Out> + Clone + Send, Out: 'static + Send
{
	fn nurse_obj( &self, fut: FutureObj<'static, Out> ) -> Result<(), SpawnError>
	{
		let handle = self.spawner.spawn_handle_obj( fut )?;

		self.in_flight.fetch_add( 1, SeqCst );

		self.tx.unbounded_send( handle ).unwrap(); // TODO: remove unwrap
		Ok(())
	}
}

impl<S> Spawn for NurseryHandle<S, ()> where S: SpawnHandle<()> + Clone + Send
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_obj( fut )
	}
}


impl<S, Out> Clone for NurseryHandle<S, Out> where S: SpawnHandle<Out> + Clone + Send, Out: 'static + Send
{
	fn clone( &self ) -> Self
	{
		Self
		{
			tx: self.tx.clone(),
			spawner: self.spawner.clone(),
			in_flight: self.in_flight.clone(),
		}
	}
}

