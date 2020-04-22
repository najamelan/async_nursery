use crate:: { import::*, LocalNurse };


/// A handle on which you can spawn tasks that will be sent to the Nursery.
///
#[ derive( Debug ) ]
//
pub struct LocalNurseryHandle<S, Out> where Out: 'static
{
	tx       : UnboundedSender<JoinHandle<Out>> ,
	spawner  : S                                ,
	in_flight: Arc<AtomicUsize>                 ,
	closed   : Arc<AtomicBool>                  ,
}


impl<S, Out> LocalNurseryHandle<S, Out> where S: LocalSpawnHandle<Out> + Clone, Out: 'static
{
	pub(crate) fn new( spawner: S, tx: UnboundedSender<JoinHandle<Out>>, in_flight: Arc<AtomicUsize>, closed: Arc<AtomicBool> ) -> Self
	{
		Self { spawner, tx, in_flight, closed }
	}
}


impl<S, Out> LocalNurse<Out> for LocalNurseryHandle<S, Out> where S: LocalSpawnHandle<Out>, Out: 'static
{
	fn nurse_local_obj( &self, fut: LocalFutureObj<'static, Out> ) -> Result<(), SpawnError>
	{
		let handle = self.spawner.spawn_handle_local_obj( fut )?;

		self.in_flight.fetch_add( 1, SeqCst );

		self.tx.unbounded_send( handle ).unwrap(); // TODO: remove unwrap
		Ok(())
	}
}

impl<S> LocalSpawn for LocalNurseryHandle<S, ()> where S: LocalSpawnHandle<()> + Clone
{
	fn spawn_local_obj( &self, fut: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_local_obj( fut )
	}
}


impl<S, Out> Clone for LocalNurseryHandle<S, Out> where S: Clone
{
	fn clone( &self ) -> Self
	{
		Self
		{
			tx        : self.tx        .clone() ,
			spawner   : self.spawner   .clone() ,
			in_flight : self.in_flight .clone() ,
			closed    : self.closed    .clone() ,
		}
	}
}

