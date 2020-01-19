mod bidding;
mod suits; 
pub struct Game {
    south: Vec<(usize,char)>,
    west: Vec<(usize,char)>,
    north: Vec<(usize,char)>,
    east: Vec<(usize,char)>,
}
impl Game {
    fn with_permutation(permutation: Vec<usize>) -> Self {
	let mut v_s: Vec<(usize,char)> = Vec::new();
	let mut v_w: Vec<(usize,char)> = Vec::new();
	let mut v_n: Vec<(usize,char)> = Vec::new();
	let mut v_e: Vec<(usize,char)> = Vec::new();  
	
	let mut cards: Vec<(usize,char)> = Vec::new(); 
	for val in permutation.iter()
	{
		match val {
		      1...12 => cards.push((*val%13+1,'C')),
		      13 => cards.push((14,'C')),
		      14...25 => cards.push((*val%13+1,'D')),
		      26 => cards.push((14,'D')),
		      27...38 => cards.push((*val%13+1,'H')),
		      39 => cards.push((14,'H')),
		      40...51 => cards.push((*val%13+1,'S')),
		      52 => cards.push((14,'S')),
		      _ => cards.push((99,'X')),
		}
	}

	for i in cards.iter().step_by(4)
	{
		v_w.push(*i)
	}
	cards.remove(0);
	
	for i in cards.iter().step_by(4)
	{
		v_n.push(*i)
	}
	cards.remove(0);
	
	for i in cards.iter().step_by(4)
	{
		v_e.push(*i)
	}
	cards.remove(0);

	for i in cards.iter().step_by(4)
	{
		v_s.push(*i)
	}

	

	Self {
	     south: v_s,
	     west: v_w,
	     north: v_n,
	     east: v_e,
	   }
    }
}
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	f.write_str("Game!\n")
    }
}
pub fn with_permutation(permutation: Vec<usize>) -> Game {
    let g = Game::with_permutation(permutation);
    
    g    
 }


pub fn format_game(game: Game) -> String {
    let mut b = bidding::Bids {
    	          bid_list: [(0,'P'); 8],
                };
		//get opening bids
		for i in 0..4 {
		    b.bid_list[i] = bidding::opening_bid(&game, &b, i); 
		    }
		//get final bids
		for i in 4..8 {
		    b.bid_list[i] = bidding::final_bid(&game, &b, i-4); 
		    }
		let mut bid_str: String = String::from("");
		for i in 0..8 {
		    if b.bid_list[i] == (0, 'P') {
		       bid_str = bid_str + "Pass\t";
		       }
		    else { bid_str = bid_str + &b.bid_list[i].0.to_string() + &b.bid_list[i].1.to_string() + "\t"; }
		    if i == 3 { bid_str = bid_str + "\n"; }  
		    }
//Appending extra passes (didn't work with tests so commented out) 
//		    let mut p_count: i32 = 0;
//		for i in 0..8 {
//		    if b.bid_list[i] == (0,'P') { p_count += 1; }
//		    else { p_count = 0; } 
//		    }
//		    bid_str = bid_str + "\n";
//		for i in 0..=p_count {
//		    bid_str = bid_str + "Pass\t";
//		    }
		    
    //calculate spacing
    let west = game.west.clone(); 
    let nor = max_suit(west)*2+2; 
    let ns_space = num_to_space(nor); 
    
    let north = game.north.clone();
    let south = game.south.clone(); 
    let eas_1 = max_suit(north);
    let eas_2 = max_suit(south);
    let mut eas = 0;
    if eas_1 > eas_2 { eas = eas_1; }
    else { eas = eas_2; }

    let mut string = String::from(format!("{}{}\n",ns_space, "North"));
    let w: suits::Suits = suits::Suits::suit_arrays(game.west);
    let n: suits::Suits = suits::Suits::suit_arrays(game.north);
    let e: suits::Suits = suits::Suits::suit_arrays(game.east);
    let s: suits::Suits = suits::Suits::suit_arrays(game.south);
    
    let e_space = num_to_space(eas*2+nor-2); 
    let e_space_s = num_to_space((eas+2)*2+nor-w.spades.len()-2);
    let e_space_h = num_to_space((eas+2)*2+nor-w.hearts.len()-2);
    let e_space_d = num_to_space((eas+2)*2+nor-w.diamonds.len()-2);
    let e_space_c = num_to_space((eas+2)*2+nor-w.clubs.len()-2);
    string.push_str(&format!("{}{}\n{}{}\n{}{}\n{}{}\n",ns_space,n.spades,ns_space,n.hearts,ns_space,n.diamonds,ns_space,n.clubs));
    string.push_str(&format!("{}{}{}\n", "West",e_space, "East"));
    string.push_str(&format!("{}{}{}\n", w.spades, e_space_s, e.spades)); 
    string.push_str(&format!("{}{}{}\n", w.hearts, e_space_h, e.hearts));
    string.push_str(&format!("{}{}{}\n", w.diamonds, e_space_d, e.diamonds));
    string.push_str(&format!("{}{}{}\n", w.clubs, e_space_c, e.clubs)); 
    string.push_str(&format!("{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",ns_space,"South",ns_space,s.spades,ns_space,s.hearts,ns_space,s.diamonds,ns_space,s.clubs));
    string = string + "\nSouth\tWest\tNorth\tEast\n" + &bid_str;
    string 
}
pub fn max_suit(hand: Vec<(usize,char)>) -> usize
{
	let count = suits::suit_count(hand); 
	let maxt = count.iter().max();
	let max = maxt.unwrap();
	*max
} 
pub fn num_to_space(spaces: usize) -> String
{
	let mut s = String::from("");
	let mut count = spaces;
	while count > 0
	{
	  s.push_str(" ");
	  count-=1;
	}
	s.to_string()

}

fn main() {
    let v = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,39,39,52,52,52,52,52,52,52,25,25,25,18,19,20,21,22,23,24,25];
    let g: Game = Game::with_permutation(v);
    //println!("{}",g);
    //println!("{}",format_game(g));
}

mod testing;
