//! All tasks get canceled when the `NurseryStream` goes out of scope.
//!
//! This program should take a less than a second, even though we have spawned tasks that want to
//! hang around for a minute.
//!
//! Expected output in less than a second:
//!
//! $ cargo run --example resource_drop
//!
//! INFO [resource_drop] nursery created
//! INFO [resource_drop] spawned slow
//! INFO [resource_drop] spawned slow
//! INFO [resource_drop] drop Nursery and NurseryStream
//!
//! When the channel senders get dropped, the awaits in main will return None.
//!
//! We have to sleep because it's so fast the tasks don't get spawned otherwise...
//!
mod common;

use
{
	async_executors :: { AsyncStd                     } ,
	async_nursery   :: { Nursery, NurseExt            } ,
	common          :: { DynSendResult, setup_tracing } ,
	futures         :: { channel::mpsc, StreamExt     } ,
	futures_timer   :: { Delay                        } ,
	std             :: { time::Duration               } ,
	tracing_crate   :: { info                         } ,
};



async fn resource_drop( senders: Vec<mpsc::UnboundedSender<()>> ) -> DynSendResult<()>
{
	let (nursery, _output) = Nursery::new( AsyncStd );
	info!( "nursery created" );

	for tx in senders.into_iter()
	{
		nursery.nurse( slow( tx ) )?;
	}

	// Don't drop them before they are spawned.
	//
	Delay::new( Duration::from_millis(10) ).await;

	info!( "drop Nursery and NurseryStream" );
	Ok(())
}



// This wants to linger around for an entire minute...zzz
//
async fn slow( tx: mpsc::UnboundedSender<()> ) -> DynSendResult<()>
{
	info!( "spawned slow" );

	Delay::new( Duration::from_secs(60) ).await;

	tx.unbounded_send(())?;

	Ok(())
}



#[ async_std::main ]
//
async fn main() -> DynSendResult<()>
{
	setup_tracing();

	let (tx , mut rx ) = mpsc::unbounded();
	let (tx2, mut rx2) = mpsc::unbounded();

	resource_drop( vec![tx, tx2] ).await?;

	assert_eq!( rx .next().await, None );
	assert_eq!( rx2.next().await, None );

	Ok(())
}
