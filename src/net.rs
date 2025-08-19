use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Network{
	pub weight: Vec<Vec<Vec<f32>>>,
	pub bias: Vec<Vec<f32>>,
	output: Vec<Vec<f32>>
}

impl Network{

	//Not implemented
	pub fn save(&self, name: &str){
		let mut file = File::create(name).expect("Couldn't read file");

		let mut to_write: [u8; 256] = [0u8; 256];
		for i in 0..self.weight[0][0].len(){
			let four = self.weight[0][0][i].to_bits().to_be_bytes();
			for j in 0..4{ to_write[i*4 + j] = four[j]; }
		}

		println!("{:?}", to_write);
		file.write(&to_write).expect("Couldn't write to file");
	}
	//Not implemented
	pub fn load(&mut self, name: &str){
		let file = File::open(name).unwrap();
		let mut buf_reader = BufReader::new(file);

		let mut contents: [u8; 256] = [0u8; 256];
		buf_reader.read(&mut contents).expect("Buffer counldn't read file");

		let mut convert = Vec::new();
		for i in 0..256{ convert.push(contents[i] as f32); }

		println!("{:?}", convert);
	}
	
	pub fn largest_output(&self) -> usize{
		let mut pos = 0;
		for i in 0..self.output[self.output.len()-1].len(){
			if self.output[self.output.len()-1][i] > self.output[self.output.len()-1][pos]{ pos = i; }
		}	
		pos
	}

	pub fn clone(&self) -> Network{
		Network{
			weight: self.weight.clone(),
			bias: self.bias.clone(),
			output: self.output.clone()
		}
	}
	
	pub fn create(brain: &[i32]) -> Network{
			
			//Creating the vectors that will store the weights, biasis and outputs
			let mut w = Vec::new();
			let mut b = Vec::new();
			let mut o = Vec::new();
			let mut rng = rand::thread_rng();
			
			//Initializing the weights
			for i in 1..brain.len(){
				w.push(Vec::new());
				b.push(Vec::new());
				o.push(Vec::new());
				
				for j in 0..brain[i]{
					w[i-1].push(Vec::new());
					b[i-1].push(rng.gen::<f32>()*2f32-1f32);
					o[i-1].push(0f32);
					
					for _k in 0..brain[i-1]{
						//let r: f32 = rng.gen()::<f32>()*2-1
						w[i-1][j as usize].push(rng.gen::<f32>()*2f32-1f32);
					}
					
				}
			}
			
		let n_done = Network{
			weight: w,
			bias: b,
			output: o
		};
		
		n_done			
	}
	
	//The output will be seen in the last output layer
	pub fn forward(&mut self, inp: &Vec<f32>, activation_function: fn(f32) -> f32){
		
		
		//This needs to be moved to a separate function
		
		//The first layer as represented by the zero
		for i in 0..self.weight[0].len(){
			self.output[0][i] = 0_f32;	
			self.output[0][i] += Network::dot_product(inp, &self.weight[0][i]) + self.bias[0][i];
			self.output[0][i] = activation_function(self.output[0][i]);
		}
		
		//Now the layers after the first
		for i in 1..self.weight.len(){
			for j in 0..self.weight[i].len(){
				self.output[i][j] = 0_f32;
				self.output[i][j] += Network::dot_product(&self.output[i-1], &self.weight[i][j]) + self.bias[i][j];
				self.output[i][j] = activation_function(self.output[i][j]);
			} 
		}
			
	}
	
	fn dot_product(l_1: &Vec<f32>, l_2: &Vec<f32>) -> f32{
		let mut sum: f32 = 0_f32;
		
		for i in 0..l_1.len(){
			sum += l_1[i]*l_2[i];
		}

		sum
	}
	
	//I only need to retrive the output for my other programs the rest of the code can remain here
	pub fn get_output(&self) -> &Vec<Vec<f32>>{
		&self.output
	}
	
	
	//Making Children
	pub fn multiply(net: &Vec<Network>, chosen: f32, mutation_chance: f32, mutation_amount: f32, chance_f: fn(f32, usize) -> f32, chance: f32) -> Vec<Network>{
		
		let mut rng = rand::thread_rng();
		let mut parent = Vec::new();
		let mut taken = vec![false; net.len()];

		//The new generation that I will return
		let mut new_gen = Vec::new();
		
		//This is without difference calculation
		//I will have write it in here later
		
		//This needs to be redone by me I kinda dislike it
		//I might need to add some special cases for weird amounts chosen
		while parent.len() < ((net.len() as f32)*chosen) as usize{
			//Severely inefficiend they way that the taken system works
			for i in 0..net.len(){
				if taken[i] == false{
					let calculated_chance: f32 = chance_f(chance, i);

					if calculated_chance == 0f32 { break }
					if calculated_chance > rng.gen::<f32>() {
						taken[i] = true;
						parent.push(net[i].clone());
						//To break the loop if the size is reached
						if parent.len() == (((net.len() as f32)*chosen) as usize){ break; }
					}
				}
			}
		}

		for i in &parent{
			//Elite child
			new_gen.push(i.clone());
			for _ in 1..(1f32/chosen) as usize{
				let other_parent: &Network = &parent[rng.gen_range(0..(parent.len()-1)) as usize];
				
				if rng.gen::<f32>() < mutation_chance{
					new_gen.push(Network::make_child(i, other_parent, mutation_amount));
				}
				else{
					new_gen.push(Network::make_child(i, other_parent, 0f32));						
				}
			}
		}
		new_gen
	}
	

	//Change this to the more performace-friendly way
	fn make_child(alpha: &Network, beta: &Network, mutation_amount: f32) -> Network{
		let mut rng = rand::thread_rng();
		
		//Creating a clone that will be modified so that is is the child of alpha and beta
		let mut clone = alpha.clone();
		for layer in 0..clone.weight.len(){
			for perceptron in 0..clone.weight[layer].len(){

				//Perceptron will be more similar to one parent that the other one
				let simi = rng.gen::<f32>();
				for w in 0..clone.weight[layer][perceptron].len(){
					clone.weight[layer][perceptron][w] = simi*alpha.weight[layer][perceptron][w] + (1f32-simi)*beta.weight[layer][perceptron][w];
				}
			}
			
			for b in 0..clone.bias[layer].len(){
				let simi = rng.gen::<f32>();
				clone.bias[layer][b] = simi*alpha.bias[layer][b] + (1f32-simi)*beta.bias[layer][b];
			}
			
			for o in 0..clone.output[layer].len(){
				clone.output[layer][o] = 0f32;
			}
		}
		
		//The mutation
		//This is probably computation heavy
		let mut rng = rand::thread_rng();

		if mutation_amount == 0f32{ return clone; }

		for layer in 0..clone.weight.len(){
			for perceptron in 0..clone.weight[layer].len(){
				for w in 0..clone.weight[layer][perceptron].len(){
					if rng.gen::<f32>() < mutation_amount{
						clone.weight[layer][perceptron][w] = rng.gen::<f32>()*2f32-1f32;
					}
				}
			}
			for b in 0..clone.bias[layer].len(){
				if rng.gen::<f32>() < mutation_amount{
					clone.bias[layer][b] = rng.gen::<f32>()*2f32-1f32
				}
			}
		}

		
		clone
	}
	
	pub fn every_not_chosen(chance: f32, place: usize) -> f32{
		return chance*f32::powi(1f32-chance, place as i32);
	}

	//Activation functions
	pub fn re_lu(val: f32) -> f32{
		if val < 0f32{
			return 0f32
		}
		
		val
	}

	pub fn sigmoid(val: f32) -> f32{
		return 1f32/(1f32-f32::powf(std::f32::consts::E, val));
	}
}
	
	