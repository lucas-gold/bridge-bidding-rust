use crate::Game;
use crate::suits;
pub struct Bids {
    pub bid_list: [(usize,char); 8],
}


     pub fn opening_bid(game: &Game, bids: &Bids, caller: usize) -> (usize,char) 
     {
      let open_suit = best_suit(&game, caller);
      let min_bid = min_bid(&bids, (1, best_suit(&game,caller)));
      let points = [suits::hand_evals(game.south.clone()), suits::hand_evals(game.west.clone()), suits::hand_evals(game.north.clone()), suits::hand_evals(game.east.clone())];
 
      if points[caller] >= 13 { 
       	 min_bid 
      	} 
      else { (0, 'P') }
      }  

      pub fn final_bid(game: &Game, bids: &Bids, caller: usize) -> (usize, char)
      {
      let points = [suits::hand_evals(game.south.clone()), suits::hand_evals(game.west.clone()), suits::hand_evals(game.north.clone()), suits::hand_evals(game.east.clone())];

      let mut team_points = 0;
      let mut teammate = 0; 

      if caller <= 1 { teammate = caller+2; }
      else { teammate = caller-2; } 
      
      let mut mate_points = 0;
      if points[teammate] < 13 { mate_points = points[teammate]; }
      else { mate_points = 13; } 
      team_points = points[caller] + mate_points; 

      let mut suit_1: char = 'P';
      let mut suit_2: char = 'P';
      
      if caller == 0 || caller == 2 {
      	 suit_1	= bids.bid_list[0].1; suit_2 = bids.bid_list[2].1; }

      else if caller == 1 || caller == 3 {
      	 suit_1	= bids.bid_list[1].1; suit_2 = bids.bid_list[3].1;
      }
	
	let mut suits: [char;4] = ['P','P','P','P'];
      if suit_1 == 'S' || suit_2 == 'S' { suits[3] = 'S'; } 
      if suit_1 == 'H' || suit_2 == 'H' { suits[2] = 'H'; }      
      if suit_1 == 'D' || suit_2 == 'D' { suits[1] = 'D'; } 
      if suit_1 == 'C' || suit_2 == 'C' { suits[0] = 'C'; }
 
	 
	 let mut fin_bid: (usize,char) = (0,'P'); 

      	 if team_points >= 36 && suits[0] == 'C' { fin_bid = (7, 'C'); }
	  else if team_points >= 36 && suits[1] == 'D' { fin_bid = (7, 'D'); }
	   else if team_points >= 36 && suits[2] == 'H' { fin_bid = (7, 'H'); }
	    else if team_points >= 36 && suits[3] == 'S' { fin_bid = (7, 'S'); }
             else if team_points >= 32 && suits[0] == 'C' { fin_bid = (6, 'C'); }
	      else if team_points >= 32 && suits[1] == 'D' { fin_bid = (6, 'D'); }
	       else if team_points >= 32 && suits[2] == 'H' { fin_bid = (6, 'H'); }
	        else if team_points >= 32 && suits[3] == 'S' { fin_bid = (6, 'S'); }
         	 else if team_points >= 29 && suits[0] == 'C' { fin_bid = (5, 'C'); }
		  else if team_points >= 29 && suits[1] == 'D' { fin_bid = (5, 'D'); }
		   else if team_points >= 27 && suits[2] == 'H' { fin_bid = (4, 'H'); }
                    else if team_points >= 27 && suits[3] == 'S' { fin_bid = (4, 'S'); } 
     
	for i in 4..8 {
	    if bids.bid_list[i] == fin_bid { 
	       fin_bid = (0,'P');
	    }
	}
      

	fin_bid
     }

	fn max_weight(bids: &Bids) -> Vec<usize>
	{
		let mut weights: Vec<usize> = Vec::new(); 
		for i in 0..8 {
		    weights.push(bid_weight(bids.bid_list[i])); 
		}
		weights
	}

     fn min_bid(bids: &Bids, curr: (usize,char)) -> (usize,char) 
     {
	let mut weights = max_weight(bids);
	let mut curr_weight = bid_weight(curr);  
	let mut max_weight = 0;
	let mut max_ind = 0;
	let mut min_bid = (0,'P'); 
	for i in weights.iter() {
	    if i > &max_weight { max_weight = *i; }
	    max_ind += 1;
	}
	let diff: isize = max_weight as isize - curr_weight as isize;
	if diff < 0 { min_bid = curr; } 
	else if diff < 10 { min_bid = (curr.0+1, curr.1); }
	else if diff < 20 { min_bid = (curr.0+2, curr.1); }
	else if diff < 30 { min_bid = (curr.0+3, curr.1); } 
	else if diff < 40 { min_bid = (curr.0+4, curr.1); }
	else if diff < 50 { min_bid = (curr.0+5, curr.1); }
	else { min_bid = (curr.0+6, curr.1); }
	
	min_bid

     }

     fn bid_weight (bid: (usize, char)) -> usize
     {
	bid.0 * 10 + suit_weight(bid.1)
     }

     fn suit_weight(suit: char) -> usize
     {
	let mut val: usize = 0;
	match suit {
	      'S' => val = 4,
	      'H' => val = 3,
	      'D' => val = 2,
	      'C' => val = 1,
	      _ => val = 0, 
	      }
	      val

     }

     fn best_suit(game: &Game, caller: usize) -> char
     {
      	let mut hand_count: [usize;4] = [0,0,0,0]; 
	match caller {
                 0 => hand_count = suits::suit_count(game.south.clone()),
                 1 => hand_count = suits::suit_count(game.west.clone()),
                 2 => hand_count = suits::suit_count(game.north.clone()),
                 3 => hand_count = suits::suit_count(game.east.clone()),
		 _ => hand_count = [0,0,0,0],
               }

	//max_len: 0 = S, 1 = H, 2 = D, 3 = C 
	let mut max_len: usize = 99; 
	let mut suit_ind = 0;
	
	for i in hand_count.iter() {
	    if i >= &5 { max_len = suit_ind; }
	    
	    suit_ind += 1; 
	}

	if max_len == 99 {
	   if hand_count[3] >= hand_count[2] { max_len = 3; }
	   else { max_len = 2; } 
	}

	let mut return_suit: char = 'P';
	match max_len {
	      0 => return_suit = 'S',
	      1 => return_suit = 'H',
	      2 => return_suit = 'D',
	      3 => return_suit = 'C',
	      _ => return_suit = 'P',  
	}
	return_suit
     }
