use wasm_bindgen::prelude::*;

use
{
	async_executors :: { Bindgen                  } ,
	async_nursery   :: { Nursery, Nurse, NurseExt } ,
	log             :: { info                     } ,
	std             :: { time::Duration           } ,
	futures_timer   :: { Delay                    } ,
	web_sys         :: { HtmlElement              } ,
	wasm_bindgen    :: { JsCast                   } ,
};

pub type DynResult<T> = Result<T, Box< dyn std::error::Error + Send + Sync + 'static >>;


fn resource_outlive_wasm( amount: usize, nursery: impl Nurse<()> ) -> DynResult<()>
{
	for i in 1..=amount
	{
		nursery.nurse( slow(i) )?;
	}

	info!( "end of resource_outlive_wasm." );
	Ok(())
}



// This wants to linger around for an entire 3 seconds...zzz
//
async fn slow( i: usize )
{
	info!( "spawned slow: {}", i );

	Delay::new( Duration::from_secs(2) ).await;

	info!( "ended slow: {}", i );
}



// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub async fn main()
{
	console_log::init_with_level( log::Level::Info ).expect( "initialize logger" );

	let window   = web_sys::window  ().expect( "no global `window` exists"        );
	let document = window  .document().expect( "should have a document on window" );
	let body     = document.body    ().expect( "document should have a body"      );

	// Manufacture the element we're gonna append
	//
	let val: HtmlElement = document.create_element( "pre" ).expect( "Failed to create pre" ).unchecked_into();

	val.set_inner_text( &format!( "
		Please check the console to verify the program output.
		By passing a nursery into a function, it can spawn other tasks that outlive itself.
		You should see from the output that the slow tasks end after resource_outlive has ended.

		Expected output in 3 seconds:

		nursery created
		spawned slow: 1
		spawned slow: 2
		spawned slow: 3
		spawned slow: 4
		spawned slow: 5
		end of resource_outlive.
		ended slow: 1
		ended slow: 3
		ended slow: 2
		ended slow: 4
		ended slow: 5
		" ) );

	body.append_child( &val ).expect( "Coundn't append child" );


	let (nursery, output) = Nursery::new( Bindgen ); info!( "nursery created" );

	// resource_outlive_wasm will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	resource_outlive_wasm( 5, nursery.clone() ).expect( "run resource_outlive_wasm" );

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	drop(nursery);

	output.await;
}
