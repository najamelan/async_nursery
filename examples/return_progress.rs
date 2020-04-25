//! Run a number of tasks concurrently that each advance at their own speed and are assigned
//! a random number of jobs to process.
//!
//! Shows a progress bar of the number of jobs done.
//!
//! Expected output:
//!
//! $ cargo run --example return_progress
//!
//! ███████████████████████████████████████████████████████████████████████████████████████████████████▏1000/1000 jobs done.
//!
mod common;

use
{
	async_executors :: { AsyncStd                                                  } ,
	async_nursery   :: { Nursery, NurseExt                                         } ,
	futures         :: { StreamExt, future::ready                                  } ,
	common          :: { DynResult                                                 } ,
	std             :: { time::Duration                                            } ,
	futures_timer   :: { Delay                                                     } ,
	indicatif       :: { ProgressBar, ProgressStyle                                } ,
	rand            :: { distributions::{ Distribution, Uniform }, Rng, thread_rng } ,
};



async fn task( units_of_work: u64 ) -> u64
{
	let speed = thread_rng().gen_range( 10, 1000 );
	Delay::new( Duration::from_millis( units_of_work*speed ) ).await;

	units_of_work
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	const UNITS: u64 = 1000;
	let between   = Uniform::from( 1..=10 );
	let mut rng   = rand::thread_rng();
	let mut pool  = UNITS;
	let mut units_of_work;

	let (nursery, output) = Nursery::new( AsyncStd );

	while pool != 0
	{
		units_of_work = if pool <= 10 { pool } else { between.sample(&mut rng) };

		nursery.nurse( task(units_of_work) )?;
		pool -= units_of_work;
	}

	let pb = ProgressBar::new( UNITS );

	pb.set_style
	(
		ProgressStyle::default_bar()

			.template( &format!("{{prefix:.bold}}▕{{wide_bar:.green}}▏{{msg}}" ) )
	);

	// don't forget.
	//
	drop(nursery);

	output.for_each( |x|
	{
		pb.inc(x);
		pb.set_message( &format!( "{}/{} jobs done.", pb.position(), UNITS ) );
		ready(())

	}).await;


	pb.finish();

	Ok(())
}
