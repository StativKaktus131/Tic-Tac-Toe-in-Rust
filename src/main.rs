//	*=====================================*
//	| !!! Main function at the bottom !!! |
//	*=====================================*


// use io libraries
use std::io::{stdin, stdout, Write};


// this will prompt the player (prompt parameter) and save the given input in the reference string
fn get_input(prompt: &str, reference: &mut String, exception: Option<&str>) {
	reference.clear();							// clear the reference string first
	print!("{}", prompt);							// prompt the player
	let _ = stdout().flush();						// flushing the console shows the prompt
	stdin().read_line(reference).expect(exception.unwrap_or(""));		// reads an empty string if the input is flawed

	// removes string trails
	if let Some('\n') = reference.chars().next_back() {
		reference.pop();
	}
	if let Some('\r') = reference.chars().next_back() {
		reference.pop();
	}
}


/* prints out a representation of the board, e.g.:

| X | _ | _ |
| X | O | _ |
| O | _ | _ |

*/
fn print_board(table: [[u8; 3]; 3]) {
	println!("");

	for col in 0..3 {
		for row in 0..3 {
			let c = match table[col][row] {
				1 => 'X',
				2 => 'O',
				_ => '_'
			};
			print!("| {} ", c);
		}
		print!("|\n");
	}
	
	println!("");
}


/*
 * I made this function rather to test out my abilities than to improve anything. 
 * The only benefit I could imagine is making it able to handle 'exceptions' (wrong user input) in a single line.
 */
fn try_expr(expr: bool, expected: bool, statement: &str) -> bool {
	if expr != expected {
		println!("\n!!! {} !!!", statement);
		return false;
	}
	true
}


// this checks the board for any winner and returns the winning player (0 if there is no winner)
fn board_winner(table: [[u8; 3]; 3]) -> u8 {
	let mut mark: u8 = table[1][1];						// mark will always be the value other fields will get compared to

	if mark != 0 {								// if mark (value at center) is zero, there can't be any winning diagonal rows
		// 1st diagonal
		if table[0][0] == mark && table[2][2] == mark {
			return mark;
		}
		// 2nd diagonal
		if table[2][0] == mark && table[0][2] == mark {
			return mark;
		}
	}
	
	// I don't use col / row here because in the second inner for loop they will get switched around I don't have two nested for loops
	for i in 0..=2 {

		// check rows first

		mark = table[i][0];
		if mark != 0 {
			let mut winning: bool = true;
			for j in 1..=2 {
				if table[i][j] != mark {
					winning = false;
				}
			}
			if winning {
				return mark;
			}
		}
		
		// then columns

		mark = table[0][i];
		if mark != 0 {
			let mut winning: bool = true;
			for j in 1..=2 {
				if table[j][i] != mark {
					winning = false;
				}
			}
			if winning {
				return mark;
			}
		}
	}
	// if none of the above is winning, return 0
	0
}


// this function queries the player for a restart with a custom message. true if they want to restart
fn query_restart(table: &mut [[u8; 3]; 3], message: &str) -> bool {
	println!("");

	let mut input = String::new();						// it's 2 a.m., I don't want to figure out how to use a reference to the previous input so I just create a new variable
	get_input(message, &mut input, None);
	
	match input.to_ascii_uppercase().as_ref() {				// y and Y are acceptable
		"Y" => {
			// resetting the board
			for i in 0..=2 {
				for j in 0..=2 {
					table[i][j] = 0;
				}
			}
			return true
		},
		// no "N" case, because it would just be the same as the '&_' block
		&_ => false
	}
}


fn main() {
	
	let mut table: [[u8; 3]; 3] = [[0; 3]; 3];				// setup the game table
	let mut input = String::new();						// general input variable
	let mut players_turn = 1;						// indicates whose turn it is at the moment

	// friendly hint
	println!("\nYou can exit this game any time by typing 'exit' instead of your coordinates");

	loop {
		print_board(table); 						// print the board every turn

		get_input(format!("Player {players_turn}, Where do you want to place your mark? (X Y): ").as_ref(), &mut input, None);

		// ends the program if the user types 'exit'
		if input == "exit" { break; }
		
		// the input should look like 'X Y', so that numbers can be a vec of u8 of size 2 with the x and y coordinate
		let numbers: Vec<u8> = { 
			let parse_result = input
				.split(' ')
				.map(str::parse::<u8>)
				.collect();
			match parse_result {
				Err(_) => {
					println!("\n!!! Only input two numbers separated by a whitespace (1-3) !!! \n");
					continue;
				},
				Ok(parsed) => parsed
			}
		};

		/*
		 * will restart the turn if:
		 * 	1. the coordinates are out of range (1 indexed, 1 to 3)
		 *	2. the field is already occupied
		 */
		if !try_expr((1..=3).contains(&numbers[0]) && (1..=3).contains(&numbers[1]), true, "Only enter numbers between 1 and 3.") { continue; }
		if !try_expr(table[numbers[1] as usize - 1][numbers[0] as usize - 1] > 0, false, "This field is already occupied, try again!") { continue; }


		// put the players mark
		table[numbers[1] as usize - 1][numbers[0] as usize - 1] = players_turn;
		
		players_turn = (players_turn) % 2 + 1; 				// cycle through players

		let winner = board_winner(table);				// either 0 or whichever player won

		if winner != 0 {
			print_board(table);					// show the winning table and ask for a restart
			if !query_restart(&mut table, format!("Congrats Player {winner}, you won the game! do you want to play another round? (Y/N) ").as_ref()) { break; }
		} else {
			// check if the table is full
			let mut table_full = true;
			for col in 0..=2 {
				for row in 0..=2 {
					if table[col][row] == 0 {
						table_full = false;
					}
				}
			}
			if table_full {
				print_board(table);				// show full board and ask for restart
				if !query_restart(&mut table, format!("It's a Draw! do you want to play another round? (Y/N) ").as_ref()) { break; }
			}
		}

		println!("");
	}
}

