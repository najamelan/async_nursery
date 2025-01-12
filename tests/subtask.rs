#![ cfg( feature = "implementation" ) ]

// Tested:
//
// ✔ pass nursery to a function as reference
// ✔ pass nursery to a function as reference - single thread.
// ✔ pass nursery to spawned subtasks as clone.
// ✔ pass nursery to spawned subtasks as clone - single thread.
//
#![ cfg(not( target_arch = "wasm32" )) ]

mod common;
use common::{ *, import::* };



// pass nursery to a function as reference.
//
#[ async_std::test ]
//
async fn outlive_method() -> DynSendResult<()>
{
	fn outlive( nursery: &Nursery<AsyncStd, usize> ) -> DynSendResult<()>
	{
		nursery.nurse( async { 5 + 5 } )?;
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}

	let (nursery, output) = Nursery::new( AsyncStd );

	outlive( &nursery )?;
	drop(nursery);

	let sum = output.fold( 0, |acc, x| async move { acc + x } ).await;

	assert_eq!( 20, sum );

	Ok(())
}


// pass nursery to a function as reference - single thread.
//
#[test] fn outlive_method_local() -> DynResult<()>
{
	fn outlive( nursery: &Nursery<TokioCt, usize> ) -> DynResult<()>
	{
		nursery.nurse( async { 5 + 5 } )?;
		nursery.nurse( async { 5 + 5 } )?;

		Ok(())
	}

	let exec              = TokioCt::new()?;
	let (nursery, output) = Nursery::new( exec.clone() );

	outlive( &nursery )?;
	drop(nursery);

	let sum = exec.block_on( output.fold( 0, |acc, x| async move { acc + x } ) );

	assert_eq!( 20, sum );

	Ok(())
}



// pass nursery to spawned subtasks as clone.
//
#[ async_std::test ]
//
async fn outlive_spawn() -> DynSendResult<()>
{
	async fn subtask( value: Arc<AtomicUsize> ) -> DynSendResult<()>
	{
		Delay::new( Duration::from_millis(10) ).await;

		value.fetch_add( 1, SeqCst );

		Ok(())
	}

	async fn outlive( value: Arc<AtomicUsize>, nursery: impl Nurse< DynSendResult<()> > + Send + 'static ) -> DynSendResult<()>
	{
		nursery.nurse( subtask( value.clone() ) )?;
		nursery.nurse( subtask( value.clone() ) )?;
		nursery.nurse( subtask( value.clone() ) )?;
		nursery.nurse( subtask( value.clone() ) )?;
		nursery.nurse( subtask( value         ) )?;

		Ok(())
	}

	let sum = Arc::new( AtomicUsize::new(0) );
	let (nursery, output) = Nursery::new( AsyncStd );

	nursery.nurse( outlive( sum.clone(), nursery.clone() ) )?;
	drop(nursery);

	output.await;

	assert_eq!( 5, sum.load( SeqCst ) );

	Ok(())
}


// pass nursery to spawned subtasks as clone - single thread.
//
#[test] fn outlive_spawn_local() -> DynResult<()>
{
	async fn subtask( value: Rc<AtomicUsize> ) -> DynSendResult<()>
	{
		Delay::new( Duration::from_millis(10) ).await;

		value.fetch_add( 1, SeqCst );

		Ok(())
	}

	async fn outlive( value: Rc<AtomicUsize>, nursery: impl LocalNurse< DynSendResult<()> > + 'static ) -> DynSendResult<()>
	{
		nursery.nurse_local( subtask( value.clone() ) )?;
		nursery.nurse_local( subtask( value.clone() ) )?;
		nursery.nurse_local( subtask( value.clone() ) )?;
		nursery.nurse_local( subtask( value.clone() ) )?;
		nursery.nurse_local( subtask( value         ) )?;

		Ok(())
	}

	let sum               = Rc::new( AtomicUsize::new(0) );
	let exec              = TokioCt::new()?;
	let (nursery, output) = Nursery::new( exec.clone() );

	nursery.nurse_local( outlive( sum.clone(), nursery.clone() ) )?;
	drop(nursery);

	exec.block_on( output );

	assert_eq!( 5, sum.load( SeqCst ) );

	Ok(())
}
