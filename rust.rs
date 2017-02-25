use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
	println!("Hello world");

	let num = 10;

	let mut age: i32 = 20;


	println!("i8 MAX{0}, MIN{1}", i8::MAX, i8::MIN);
	println!("i16 MAX{0}, MIN{1}", i16::MAX, i16::MIN);
	println!("i32 MAX{0}, MIN{1}", i32::MAX, i32::MIN);
	println!("i64 MAX{0}, MIN{1}", i64::MAX, i64::MIN);
	println!("u8 MAX{0}, MIN{1}", u8::MAX, u8::MIN);
	println!("u16 MAX{0}, MIN{1}", u16::MAX, u16::MIN);
	println!("u32 MAX{0}, MIN{1}", u32::MAX, u32::MIN);
	println!("u64 MAX{0}, MIN{1}", u64::MAX, u64::MIN);
	println!("isize MAX{0}, MIN{1}", isize::MAX, isize::MIN);
	println!("usize MAX{0}, MIN{1}", usize::MAX, usize::MIN);
	println!("f32 MAX{0}, MIN{1}", f32::MAX, f32::MIN);
	println!("f64 MAX{0}, MIN{1}", f64::MAX, f64::MIN);

	let is_it_true: bool = true;
	let let_x: char = 'x';

	println!("I am {} y/old", age );

	let (f_name, l_name) = ("Joe", "Black");

	println!("Is it {0} that {1} is {0}", is_it_true, let_x);

	//format
	println!("{:.2}", 1.234);

	println!("B: {:b} H: {:x} O {:o}", 10, 10, 10);

	println!("{ten:>ws$}", ten=10, ws=5);
	println!("{ten:>0ws$}", ten=10, ws=5);

	//math functions
	println!("5 + 4 ={}", 5 + 4); // -, *, /, %

	let mut neg_4 = -4i32;

	println!("abs(-4) = {}", neg_4.abs());
	// pow, sqrt, cbrt, round, floot, ceil, exp, ln, log19, to_radians, to_degrees, max, min

	println!("Sin 3.14 = {}", 3.14f64.sin());

	//conditionals
	let age_old = 6;

	if age_old == 6 {
	    println!("Kinder");
	}else if (age_old > 5) && (age_old <=18){
		println!("Go to grade {}", age_old -5);
	}else if (age_old <=25) && (age_old > 18){
		println!("College");
	}else{
		println!("Go somewhere");
	}


	println!("!true= {}", !true);
	println!("true || false = {}", true || false);
	println!("true != false: {}", (true != false));

	let can_vote = if age_old >= 18 {true} else {false};

	println!("Can vote {}", can_vote);

	//loops

	let mut x = 1;

	loop {
	    if (x % 2) == 0 {
	    	println!("{}", x);
	    	x += 1;

	    	continue;
	    }

	    if (x > 10){
	    	break;
	    }

	    x+=1;
	    continue;
	}

	let mut y = 1;

	while y <=10 {
	    println!("y = {}", y);
		y += 1; 
	}

	for z in 1..10 {
		println!("For {}", z);
	}

	//strings

	let rand_string = "I am a random string";

	println!("Length {}", rand_string.len());

	let (first, second)  = rand_string.split_at(6);

	println!("first {}, and second {}", first, second);

	let count = rand_string.chars().count();
	let mut chars = rand_string.chars();

	let mut indiv_char = chars.next();

	loop {
	    match indiv_char {
	    	Some(x) => println!("{}", x),
	    	None => break,
	    }

	    indiv_char = chars.next();
	}

	let mut iter = rand_string.split_whitespace();
	let mut indiv_word = iter.next();

	loop {
	    match indiv_word {
	    	Some(x) => println!("{}", x),
	    	None => break,
	    }

	    indiv_word = iter.next();
	}

	let rand_string2 = "I a ma random \nStrign og text";

	let mut lines = rand_string2.lines();
	let mut indiv_line = lines.next();

	loop {
	    match indiv_line {
	    	Some(x) => println!("line={}", x),
	    	None => break,
	    }

	    indiv_line = lines.next();
	}


	println!("Find text {}", rand_string2.contains("text"));



	// number guessing game

	println!("***Pick a number game***");

	'outer: loop {
	    let number : i32 = 10;
	    println!("Pick a number (answer=10):");

	    loop {
	        let mut line = String::new();

	        // line by reference
	        let input = stdin().read_line(&mut line);

	        //ok(): checks reader at end of input
	        //map_or()': apply default value or functions to a value

	        let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
	    
	        match guess {
	            Some(n) if n == number => {
	            	println!("You Guessed it");
	            	break 'outer;
	            },
	            Some(n) if n < number => {
	            	println!("Too low");
	            },
	            Some(n) if n > number => {
	            	println!("Too high");
	            },
	            Some(_) => println!("Error"),	            
	            None => println!("Enter a number: "),
	        }
	    }
	}


	// arrays

	let rand_array = [1,2,3];

	println!("{}", rand_array[0]);
	println!("{}", rand_array.len());
	println!("Second 2 {:?}", &rand_array[1..3]);



	// vectors (grow in size)

	let mut vect1 = vec![1,2,3,4,5];

	println!("Item 2 : {}", vect1[1]);

	for i in &vect1 {
	    println!("Vect: {}", i );
	}

	vect1.push(6);
	vect1.pop();


	// tuples (fixed sized lists, keys as indexes)

	let rand_tuple = ("Jim", 30) ;
	let rand_tuple_2:  (&str, i8) = ("Jim", 30);

	println!("Name: {}", rand_tuple_2.0);

	// functions

	say_hello("Jim");
	
	println!("5 + 4 = {}", get_sum(5,4));

	let sum = get_sum;
	println!("6 + 4 = {}", sum(6,4));

	//closures

	let sum_nums = |x: i32, y: i32| x + y;
	println!("7 + 8 = {}", sum_nums(7,8));

	let num_ten = 10;

	let add_10 = |x: i32| x + num_ten;
	println!("5 + 10 = {}", add_10(5));


	//ownership

	let vect1 = vec![1,2,3];
	let vect2 = vect1;

	//wont work, must be used by reference, see below
	//println!("vect1[0] : {}", vect1[0]);

	let prim_val = 1;
	let prim_val2 = prim_val;

	//works with primitives only
	println!("prim_val: {}", prim_val);

	println!("Sum of Vect: {}", sum_vects(&vect2));

	println!("Vect : {:?}", vect2);

	//structs

	let mut circle1 = Circle {
		x: 10.0, y: 10.0, radius: 10.0
	};

	println!("X :{}, Y: {}, R: {}", circle1.x, circle1.y, circle1.radius);
	println!("R: {}", get_radius(&circle1));
	println!("Circle x: {}", circle1.get_x());


	//traits

	println!("Circle area: {}", circle1.area());

	let mut rect1 = Rectangle {
		height: 10.0, width: 10.0
	};

	println!("Rectangle Area: {}", rect1.area());


	//enumerate

	let hulk = Hero::Strong(100);
	let quicksilver = Hero::Fast;
	let spidey = Hero::Info{name: "Spiderman".to_owned(), secret: "Peter".to_owned()};

	print!("hulk"); get_info(hulk);
	print!("quicksilver"); get_info(quicksilver);
	get_info(spidey);


}

fn say_hello (name: &str){

	println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32{
	num1 + num2
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
	// fold appy func to values
	let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
	return sum;
}

struct Circle {
	x: f64,
	y: f64,
	radius: f64,	
}

fn get_radius(circle: &Circle) -> f64 {
	circle.radius
}

//recommended
impl Circle {
	pub fn get_x(&self) -> f64 {
		self.x
	} 
}

struct Rectangle {
	height: f64,
	width: f64,
}

trait HasArea {
	fn area(&self) -> f64;
}

impl HasArea for Circle{
	fn area(&self) -> f64 {
		3.14159 * (self.radius * self.radius)
	}
}

impl HasArea for Rectangle{
	fn area(&self) -> f64 {
		self.height * self.width
	}
}

enum Hero {
	Fast, 
	Strong(i32),
	Info{name: String, secret: String}
}

fn get_info(h: Hero){
	match h {
	    Hero::Fast => println!(" is Fast"),
	   	Hero::Strong(i) => println!(" lifts {} tons", i),
	   	Hero::Info{name, secret} => {
	   		println!("{} is {}", name, secret);
	   	},
	}
}