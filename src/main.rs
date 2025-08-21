//This is for playing against the AI
//use std::io;


use std::{thread, time, env};
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};

use crate::net::Network;
use crate::fir::Four;
use crate::fir::GameState;

pub mod net;
pub mod fir;

//The lists must be the same lenght
//Else the dot product wont work
//Might repair this later but for the neural network it will work
//This will probably be a private funnction

//This tells rust that the pointer is valid for as long as the variable that holds the list is viable

//The program will be best of three where they will be assigned a score
//The score will represent in how many turn they won

//Code for two Ai:s to play the game and if the first one wins true is returned else false is
fn play_game(c1: &mut Network, c2: &mut Network, c: f32) -> bool{
	//Creating a game
	let mut game: Four = Four::new();
	let mut random = rand::thread_rng();

	//c1 will go first then we'll switch who starts
	loop {
		if &c < &random.gen::<f32>(){
			match game.place(&random.gen_range(0..9)){
				GameState::Won => return true,
				GameState::Lost => return false,
				GameState::Unfinished => (),
			}
		}
		else{
			c1.forward(&game.convert(), Network::re_lu);
			match game.place(&c1.largest_output()){
				GameState::Won => return true,
				GameState::Lost => return false,
				GameState::Unfinished => (),
			}
		}
		game.invert_display();

		if &c < &random.gen::<f32>(){
			match game.place(&random.gen_range(0..9)){
				GameState::Won => return true,
				GameState::Lost => return false,
				GameState::Unfinished => (),
			}
		}
		else{
			c2.forward(&game.convert(), Network::re_lu);
			match game.place(&c2.largest_output()) {
				GameState::Won => return false,
				GameState::Lost => return true,
				GameState::Unfinished => (),
			}
		}
		game.invert_display();
	}
}

fn thread_playing_game(set: usize, creature: &Vec<Network>, c: f32) -> thread::JoinHandle<TeamHolder>{
	//Copying the memory of the creatures to some variables
	let c1: Network  = creature[set*2].clone();
	let c2: Network = creature[set*2 + 1].clone();

	let handle: thread::JoinHandle<TeamHolder> = thread::spawn(move|| {
		//Moving the variables into the thread
		let chance: f32 = c;
		let mut team: TeamHolder = TeamHolder{
			net1: c1,
			net2: c2,
			winner: None,
		};

		team.winner = Some(play_game(&mut team.net1, &mut team.net2, chance));

		team
	});
	handle
}

fn play(creature: &Vec<Network>, c: f32) -> (Vec<Network>, Vec<Network>){
	let mut winner: Vec<Network> = Vec::new();
	let mut loser: Vec<Network> = Vec::new();

	//Creating the threads and their handles
	let mut handles = Vec::new();
	for set in 0..(creature.len()/2 as usize){
		handles.push(thread_playing_game(set, &creature, c));
	}

	//Taking care of the results from the threads
	for handle in handles{
		let team = handle.join().unwrap();
		
		//Diving up the winners and losers
		match team.winner{
		Option::Some(w) => { 
			if w == true { winner.push(team.net1); loser.push(team.net2); }
			else if w == false { winner.push(team.net2); loser.push(team.net1); }
		},
		Option::None => unreachable!(),
		}
	}

	(winner, loser)
}

struct TeamHolder{
	net1: Network,
	net2: Network,
	winner: Option<bool>,
}


//Maybe I should min_max the best move and then apply backpropagation
//After the network has got the basics of the game
//Then maybe I should make the AI play against itself ?? 
//Maybe something is wrong with my algorithm

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let b = [54, 100, 50, 9];

	//Chose this amount so that it is a multiple of 2^x
	let amount: usize = usize::pow(2, 9);
	let mut creature = Vec::new();

	//Pushing the creatures
	for _i in 0..amount{	creature.push(Network::create(&b)); }

	//creature[0].save("name.txt");
	//creature[0].load("name.txt");

	//println!("Done");

	for gen in 0..50000{
		//Holding the winners

		let chance: f32;

		if gen < 500 { chance =   ((gen+1) as f32)/500f32  }
		else { chance = 1f32 }

		let mut new: Vec<Network> = Vec::new();
		for _i in 0..(f32::log2(creature.len() as f32) as usize){
			let (winner, loser) = play(&creature, chance);
			
			for l in loser{ new.push(l.clone()) }
			creature = winner;
		}
		for c in creature { new.push(c.clone()); }
		new.reverse();
		
		creature = Network::multiply(&new, 0.25f32, 0.20f32, 0.0001f32, Network::every_not_chosen, 0.01f32);
		creature.shuffle(&mut  thread_rng());
		println!("gen: {gen}");

		if gen % 10 == 0{
			//c1 will go first then we'll switch who starts
			let time_sleep = 500;
			let mut game = Four::new();
			let half = creature.len()/2;
			loop {
				thread::sleep(time::Duration::from_millis(time_sleep));
				game.print();
				creature[0].forward(&game.convert(), Network::re_lu);
				match game.place(&creature[0].largest_output()){
					GameState::Won => break,
					GameState::Lost => break,
					GameState::Unfinished => (),
				}
				game.invert_display();
				
				creature[half].forward(&game.convert(), Network::re_lu);
				match game.place(&creature[1].largest_output()) {
					GameState::Won => {	game.invert_display(); break },
					GameState::Lost => { game.invert_display(); break },
					GameState::Unfinished => (),
				}
				game.invert_display();
			}
			game.print();
		}
	} 
}
