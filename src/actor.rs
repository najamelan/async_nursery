use
{
	crate   :: { import::*, Nursery, NurseExt, NurseErr } ,
	thespis :: { *                                      } ,
};



impl<S, Out> Actor for Nursery<S, Out>

	where S  : 'static + Unpin + SpawnHandle<Out> + SpawnHandle<()> + Send,
	      Out: 'static + Send

{}



/// _thespis_ Message type for spawning on this nursery.
//
#[ derive( Debug ) ]
//
pub struct NurseTask<Fut>

	where Fut: Future + 'static + Send

{
	/// The task to be spawned.
	//
	pub task: Fut,
}



impl<Fut> Message for NurseTask<Fut>

	where Fut: Future + 'static + Send

{
	type Return = Result<(), NurseErr>;
}



impl<S, Fut> Handler< NurseTask<Fut> > for Nursery<S, Fut::Output>

	where Fut: 'static + Future + Send,
	      S  : 'static + Unpin + SpawnHandle<Fut::Output> + SpawnHandle<()> + Send,
	      Fut::Output: 'static + Send,

{
	#[async_fn] fn handle( &mut self, msg: NurseTask<Fut> ) -> Result<(), NurseErr>
	{
		self.nurse( msg.task )
	}
}

