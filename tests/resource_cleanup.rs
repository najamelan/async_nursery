#![ cfg( feature = "implementation" ) ]

// Tested:
//
// ✔ Spawned tasks have finished when awaited. Multi thread.
// ✔ Spawned tasks have finished when awaited. Single thread.
// ✔ Spawned tasks have finished the stream is finished. Multi thread.
// ✔ Spawned tasks have finished the stream is finished. Single thread.
// ✔ Spawned tasks are dropped when the nursery and stream are dropped. Multi Thread.
// ✔ Spawned tasks are dropped when the nursery and stream are dropped. Single Thread.
//
#![ cfg(not( target_arch = "wasm32" )) ]

mod common;
use common::{ *, import::* };


async fn cleanup_await_mt_inner( proofs: Vec<Arc<AtomicBool>> ) -> DynResult<()>
{
	let (nursery, output) = Nursery::new( AsyncStd );

	for proof in proofs.iter()
	{
		nursery.nurse( prove( proof.clone() ) )?;
	}

	drop(nursery);
	output.await;

	Ok(())
}


async fn prove( p: Arc<AtomicBool> )
{
	// make sure there is at least some await point
	//
	Delay::new( Duration::from_millis( 20 ) ).await;
	p.store( true, SeqCst );
}



// Spawned tasks have finished when awaited. Multi thread.
//
#[ async_std::test ]
//
async fn cleanup_await_mt() -> DynResult<()>
{
	let num_tasks = 5;
	let mut proofs = Vec::new();

	for _ in 0..num_tasks
	{
		proofs.push( Arc::new( AtomicBool::new(false) ) );
	}

	cleanup_await_mt_inner( proofs.clone() ).await?;

	for proof in proofs.iter()
	{
		assert!( proof.load(SeqCst) );
	}

	Ok(())
}



async fn cleanup_await_st_inner( proofs: Vec<Arc<AtomicBool>>, exec: TokioCt ) -> DynResult<()>
{
	let (nursery, output) = Nursery::new( exec );

	for proof in proofs.iter()
	{
		nursery.nurse( prove( proof.clone() ) )?;
	}

	drop(nursery);
	output.await;

	Ok(())
}


// Spawned tasks have finished when awaited. Single thread.
//
#[test] fn cleanup_await_st() -> DynResult<()>
{
	let exec = TokioCtBuilder::new().build()?;

	let num_tasks = 5;
	let mut proofs = Vec::new();

	for _ in 0..num_tasks
	{
		proofs.push( Arc::new( AtomicBool::new(false) ) );
	}

	exec.block_on( cleanup_await_st_inner( proofs.clone(), exec.clone() ) )?;

	for proof in proofs.iter()
	{
		assert!( proof.load(SeqCst) );
	}

	Ok(())
}



async fn cleanup_stream_mt_inner( proofs: Vec<Arc<AtomicBool>> ) -> DynResult<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd );

	for proof in proofs.iter()
	{
		nursery.nurse( prove( proof.clone() ) )?;
	}

	drop(nursery);
	while output.next().await.is_some() {}

	Ok(())
}



// Spawned tasks have finished the stream is finished. Multi thread.
//
#[ async_std::test ]
//
async fn cleanup_stream_mt() -> DynResult<()>
{
	let num_tasks = 5;
	let mut proofs = Vec::new();

	for _ in 0..num_tasks
	{
		proofs.push( Arc::new( AtomicBool::new(false) ) );
	}

	cleanup_stream_mt_inner( proofs.clone() ).await?;

	for proof in proofs.iter()
	{
		assert!( proof.load(SeqCst) );
	}

	Ok(())
}



async fn cleanup_stream_st_inner( proofs: Vec<Arc<AtomicBool>>, exec: TokioCt ) -> DynResult<()>
{
	let (nursery, mut output) = Nursery::new( exec );

	for proof in proofs.iter()
	{
		nursery.nurse( prove( proof.clone() ) )?;
	}

	drop(nursery);
	while output.next().await.is_some() {}

	Ok(())
}


// Spawned tasks have finished when the stream is finished. Single thread.
//
#[test] fn cleanup_stream_st() -> DynResult<()>
{
	let exec = TokioCtBuilder::new().build()?;

	let num_tasks = 5;
	let mut proofs = Vec::new();

	for _ in 0..num_tasks
	{
		proofs.push( Arc::new( AtomicBool::new(false) ) );
	}

	exec.block_on( cleanup_stream_st_inner( proofs.clone(), exec.clone() ) )?;

	for proof in proofs.iter()
	{
		assert!( proof.load(SeqCst) );
	}

	Ok(())
}



async fn resource_drop_mt_inner( senders: Vec<mpsc::UnboundedSender<()>> ) -> DynResult<()>
{
	let (nursery, _output) = Nursery::new( AsyncStd );

	for tx in senders.into_iter()
	{
		nursery.nurse( slow(tx) )?;
	}

	// Don't drop them before they are spawned.
	//
	Delay::new( Duration::from_millis(10) ).await;

	Ok(())
}



// This wants to linger around for an entire minute...zzz
//
async fn slow( tx: mpsc::UnboundedSender<()> ) -> DynResult<()>
{
	Delay::new( Duration::from_secs(60) ).await;

	tx.unbounded_send(())?;

	Ok(())
}



// Spawned tasks are dropped when the nursery and stream are dropped. Multi Thread.
//
#[ async_std::test ]
//
async fn resource_drop_mt() -> DynResult<()>
{
	let (tx , mut rx ) = mpsc::unbounded();
	let (tx2, mut rx2) = mpsc::unbounded();

	resource_drop_mt_inner( vec![tx, tx2] ).await?;

	assert_eq!( rx .next().await, None );
	assert_eq!( rx2.next().await, None );

	Ok(())
}



async fn resource_drop_st_inner( senders: Vec<mpsc::UnboundedSender<()>>, exec: TokioCt ) -> DynResult<()>
{
	let (nursery, _output) = Nursery::new( exec );

	for tx in senders.into_iter()
	{
		nursery.nurse( slow(tx) )?;
	}

	// Don't drop them before they are spawned.
	//
	Delay::new( Duration::from_millis(10) ).await;

	Ok(())
}



// Spawned tasks are dropped when the nursery and stream are dropped. Single Thread.
//
#[test] fn resource_drop_st() -> DynResult<()>
{
	let exec = TokioCtBuilder::new().build()?;

	let (tx , mut rx ) = mpsc::unbounded();
	let (tx2, mut rx2) = mpsc::unbounded();

	#[ allow(clippy::redundant_clone) ] // false positive
	//
	exec.clone().block_on( async move
	{
		resource_drop_st_inner( vec![tx, tx2], exec ).await.unwrap();

		assert_eq!( rx .next().await, None );
		assert_eq!( rx2.next().await, None );
	});

	Ok(())
}
