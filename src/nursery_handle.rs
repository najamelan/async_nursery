use crate:: { import::*, Nurse, NurseErr };


/// A handle on which you can spawn tasks that will be sent to the Nursery.
///
#[ derive( Debug ) ]
//
pub struct NurseryHandle<S, Out>
{
	tx       : UnboundedSender<JoinHandle<Out>> ,
	spawner  : S                                ,
	in_flight: Arc<AtomicUsize>                 ,
	closed   : Arc<AtomicBool>                  ,
}


impl<S, Out> NurseryHandle<S, Out>
{
	pub(crate) fn new( spawner: S, tx: UnboundedSender<JoinHandle<Out>>, in_flight: Arc<AtomicUsize>, closed: Arc<AtomicBool> ) -> Self
	{
		Self { spawner, tx, in_flight, closed }
	}
}


impl<S, Out> Nurse<Out> for NurseryHandle<S, Out> where S: SpawnHandle<Out>, Out: 'static + Send
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



impl<S> Spawn for NurseryHandle<S, ()> where S: SpawnHandle<()>
{
	fn spawn_obj( &self, fut: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.nurse_obj( fut ).map_err( |_| SpawnError::shutdown() )
	}
}



impl<S, Out> Clone for NurseryHandle<S, Out> where S: Clone
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

