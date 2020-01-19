pub struct Suits {
       pub spades: String,
       pub hearts: String,
       pub diamonds: String,
       pub clubs: String,
}

pub fn hand_evals(cards: Vec<(usize,char)>) -> usize
{ 

	 //Accumulate points from face cards
	let mut ptotal: usize = 0;

	 for i in cards.iter()
	 {
	  if i.0 >= 11 { ptotal += i.0%10 } 
	 }
	let suit_counts = suit_count(cards);  
	let stotal = suit_tot(suit_counts);
	ptotal+stotal
	 
}

pub fn suit_count(cards: Vec<(usize,char)>) -> [usize;4]
{
	let mut count: [usize;4] = [0,0,0,0]; 
	
	for i in cards.iter() {
	 if i.1 == 'S' { count[0]+=1; }
	 else if i.1 == 'H' { count[1]+=1; }
	 else if i.1 == 'D' { count[2]+=1; }
	 else { count[3]+=1; }
	}
	count
}

pub fn suit_tot (totals: [usize;4]) -> usize {
     let mut stotal: usize = 0; 
     for i in totals.iter() {
          if *i == 0 { stotal += 3; }
          if *i == 1 { stotal += 2; }
          if *i == 2 { stotal += 1; }
         }
         stotal
}

impl Suits {
     pub fn suit_arrays (mut hand: Vec<(usize,char)>) -> Self
     {
	let mut s = String::from("S ");
	let mut d = String::from("D ");
	let mut h = String::from("H ");
	let mut c = String::from("C ");

	hand.sort_by(|a,b| b.cmp(a));
	let mut cards: Vec<(String,char)> = Vec::new(); 
	for i in hand.iter() {
	    match i.0 {
	     14 => cards.push(("A".to_string(),i.1)),
	     13 => cards.push(("K".to_string(),i.1)),
	     12 => cards.push(("Q".to_string(),i.1)),
	     11 => cards.push(("J".to_string(),i.1)),
	     _ => cards.push((i.0.to_string(),i.1)),
	    } 
	}

	for i in cards.iter() {
	    match i.1 {
	    'S' => s.push_str(&format!("{}{}",&i.0," ")),  
	    'D' => d.push_str(&format!("{}{}",&i.0," ")),
	    'H' => h.push_str(&format!("{}{}",&i.0," ")),
	    'C' => c.push_str(&format!("{}{}",&i.0," ")),
	    _ => c.push_str(" "),
	    }
      	 }
		 Self {
     		      spades: s.trim_end().to_string(),
     		      hearts: h.trim_end().to_string(),
    		      diamonds: d.trim_end().to_string(),
     		      clubs: c.trim_end().to_string(),
   		      }
     	 
     }
}



