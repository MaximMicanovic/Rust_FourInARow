
pub enum GameState{
	Won,
	Lost,
	Unfinished,
}


pub struct Four{
	//Must be signed
	//1 and -1 for the other player
	pub space: [[i8; 6]; 9]
}

impl Four{
	
	pub fn convert(&self) -> Vec<f32>{
		let mut conv = Vec::new();
		for x in self.space{
			for y in x{	conv.push(y as f32); }	
		}
		conv
	}

	pub fn invert_display(&mut self){
		for x in &mut self.space{
			for y in x{ *y *= -1; }
		}
	}

	pub fn new() -> Four{
		let s: [[i8; 6]; 9] = [[0i8; 6]; 9];
		return Four{
			space: s,
		}
	} 
	
	pub fn print(&self){
		println!();
		for i in 0..6{
			for j in 0..9{
				if self.space[j][i] == 0 { print!(" {}", self.space[j][i]); }
				else if self.space[j][i] == 1 { print!(" {}", 'X'); }
				else if self.space[j][i] == -1 { print!(" {}", 'Z'); }
				else { print!("{}", self.space[j][i]); }
			}
			println!();
		}
	}
	
	//Returns if its won or if its lost where
	//Its an automatic lost if you place it on a full column
	pub fn place(&mut self, spot: &usize) -> GameState{
		
		//If the first space is full
		if self.space[*spot][0] != 0{ return GameState::Lost; }
		
		//Checking the depth
		//This need to be 5 if it dosent find a value
		let mut empty_space_y: usize = 5;
		for i in 1..self.space[*spot].len(){
			if self.space[*spot][i as usize] == -1 || self.space[*spot][i as usize] == 1{
				empty_space_y = i-1;
				break;
			}			
		}
		self.space[*spot][empty_space_y] = 1i8;
		

		return self.win(spot, &empty_space_y);
	}
	
	fn win(&self, x: &usize, y: &usize) -> GameState{	
		
		let mut counter = 0;
		//First we check from up to down
		for i in *y..self.space[*x].len(){
			if self.space[*x][i] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
		}
		if counter >= 4{
			return GameState::Won;
		}
		
		counter = 0;
		//Checking to the right
		for i in *x..self.space.len(){
			if self.space[i][*y] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
		}
		
		//Checking to the left
		for i in (0..(*x as isize)).rev(){		
			if self.space[i as usize][*y] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
		}
		if counter >= 4{
			return GameState::Won;
		}
		
		//Checking the Diagonals lets begin with /
		let mut x_pos: isize = *x as isize;
		let mut y_pos: isize = *y as isize;
		counter = 0;
		while x_pos < self.space.len() as isize && y_pos >= 0isize{
			
			if self.space[x_pos as usize][y_pos as usize] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
			
			x_pos += 1;
			y_pos -= 1;
		}

		let mut x_pos: isize = (*x as isize)-1;
		let mut y_pos: isize = (*y as isize)+1;
		while x_pos >= 0isize && y_pos < self.space[*x].len() as isize{
			
			if self.space[x_pos as usize][y_pos as usize] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
			
			x_pos -= 1;
			y_pos += 1;
		}
		if counter >= 4{
			return GameState::Won;
		}

		//Now I'm checking \ direction		
		let mut x_pos: isize = *x as isize;
		let mut y_pos: isize = *y as isize;
		counter = 0;
		while x_pos >= 0isize && y_pos >= 0isize{
			if self.space[x_pos as usize][y_pos as usize] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
			
			x_pos -= 1;
			y_pos -= 1;
		}
		
		
		let mut x_pos = *x+1;
		let mut y_pos = *y+1;
		while x_pos < self.space.len() && y_pos < self.space[*x].len(){
			if self.space[x_pos][y_pos] == 1i8{
				counter += 1;
			}
			else{
				break;
			}
			
			x_pos += 1;
			y_pos += 1;
		}
		if counter >= 4{
			return GameState::Won;
		}
		
		
		return GameState::Unfinished;
	}
}