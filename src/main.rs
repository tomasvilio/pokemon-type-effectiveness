const TYPE_MATCHUP: [[f32; 18]; 18] = 
[
    // NORMAL, FIGHT,  FLYING, POISON, GROUND, ROCK,   BUG,    GHOST,  STEEL,  FIRE,   WATER,  GRASS,  ELECTR, PSYCHC, ICE,    DRAGON, DARK,   FAIRY
    [  1.0,    1.0,    1.0,    1.0,    1.0,    0.5,   1.0,    0.0,    0.5,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0  ], // NORMAL
    [  2.0,    1.0,    0.5,    0.5,    1.0,    2.0,   0.5,    0.0,    2.0,    1.0,    1.0,    1.0,    1.0,    0.5,    2.0,    1.0,    2.0,    0.5  ], // FIGHT
    [  1.0,    2.0,    1.0,    1.0,    1.0,    0.5,   2.0,    1.0,    0.5,    1.0,    1.0,    2.0,    0.5,    1.0,    1.0,    1.0,    1.0,    1.0  ], // FLYING
    [  1.0,    1.0,    1.0,    0.5,    0.5,    0.5,   1.0,    0.5,    0.0,    1.0,    1.0,    2.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0  ], // POISON
    [  1.0,    1.0,    0.0,    2.0,    1.0,    2.0,   0.5,    1.0,    2.0,    2.0,    1.0,    0.5,    2.0,    1.0,    1.0,    1.0,    1.0,    1.0  ], // GROUND
    [  1.0,    0.5,    2.0,    1.0,    0.5,    1.0,   2.0,    1.0,    0.5,    2.0,    1.0,    1.0,    1.0,    1.0,    2.0,    1.0,    1.0,    1.0  ], // ROCK
    [  1.0,    0.5,    0.5,    0.5,    1.0,    1.0,   1.0,    0.5,    0.5,    0.5,    1.0,    2.0,    1.0,    2.0,    1.0,    1.0,    2.0,    0.5  ], // BUG
    [  0.0,    1.0,    1.0,    1.0,    1.0,    1.0,   1.0,    2.0,    1.0,    1.0,    1.0,    1.0,    1.0,    2.0,    1.0,    1.0,    0.5,    1.0  ], // GHOST
    [  1.0,    1.0,    1.0,    1.0,    1.0,    2.0,   1.0,    1.0,    0.5,    0.5,    0.5,    1.0,    0.5,    1.0,    2.0,    1.0,    1.0,    2.0  ], // STEEL
    [  1.0,    1.0,    1.0,    1.0,    1.0,    0.5,   2.0,    1.0,    2.0,    0.5,    0.5,    2.0,    1.0,    1.0,    2.0,    0.5,    1.0,    1.0  ], // FIRE
    [  1.0,    1.0,    1.0,    1.0,    2.0,    2.0,   1.0,    1.0,    1.0,    2.0,    0.5,    0.5,    1.0,    1.0,    1.0,    0.5,    1.0,    1.0  ], // WATER
    [  1.0,    1.0,    0.5,    0.5,    2.0,    2.0,   0.5,    1.0,    0.5,    0.5,    2.0,    0.5,    1.0,    1.0,    1.0,    0.5,    1.0,    1.0  ], // GRASS
    [  1.0,    1.0,    2.0,    1.0,    0.0,    1.0,   1.0,    1.0,    1.0,    1.0,    2.0,    0.5,    0.5,    1.0,    1.0,    1.0,    1.0,    1.0  ], // ELECTRIC
    [  1.0,    2.0,    1.0,    1.0,    1.0,    1.0,   1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    0.5,    1.0,    1.0,    0.0,    1.0  ], // PSYCHIC
    [  1.0,    1.0,    2.0,    1.0,    1.0,    1.0,   1.0,    1.0,    0.5,    0.5,    0.5,    2.0,    1.0,    1.0,    0.5,    2.0,    1.0,    1.0  ], // ICE
    [  1.0,    1.0,    1.0,    1.0,    1.0,    1.0,   1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    2.0,    1.0,    0.0  ], // DRAGON
    [  1.0,    0.5,    1.0,    1.0,    1.0,    1.0,   1.0,    2.0,    1.0,    1.0,    1.0,    1.0,    1.0,    2.0,    1.0,    1.0,    0.5,    0.5  ], // DARK
    [  1.0,    2.0,    1.0,    2.0,    1.0,    1.0,   1.0,    1.0,    0.5,    1.0,    1.0,    1.0,    1.0,    1.0,    1.0,    2.0,    2.0,    1.0  ], // FAIRY
];
//this is a 2D array
//here we declare the type matchup of the pokemon world.

//this typechart is based on the 18 types.
//each type is a vector that contains the type effectiveness of that type against 
//all other types.
//for example, the type effectiveness of water against grass would be 0.5


//this function returns the type effectiveness of type1 against type2
fn type_effectiveness(type1: i32, type2: i32) -> Vec<f32>
{
    let mut numbers_type1 = Vec::new(); //vector that contains the type effectiveness of the first type against
    //all the other types on the type chart
    let mut numbers_type2 = Vec::new();//vector that contains the type effectiveness of the second type against
    //all the other types on the type chart
    let mut matchup = Vec::new(); //vector that contains the type effectiveness of the first and second type
    //the values of this vector are achieved by multiplying the type effectiveness of the first type
    //against all the other types and the type effectiveness of the second type
    //against all the other types

    //for loop that loops through the type chart(0 to 17)
    for i in 0..18
    {
        //we add the effectiveness of all types against the first type (type1)
        numbers_type1.push(TYPE_MATCHUP[i][type1 as usize]);

        //we add the effectiveness of all types against the second type (type2)
        //he also chekc if the second type is out of bounds becuase if the user selects a singular typing 
        //we dont wann be out of bounds so we give it a value that is non existant in the type chart
        // and set all of its effectivnees to 1 so when put togheter with the first type it will only give
        //info about the first type(witch in this case is the only type the user selected)
        numbers_type2.push(if type2 < 0 || type2 > 17 { 1.0 } else { TYPE_MATCHUP[i][type2 as usize] });

        //here we calculate the type effectiveness witch is the product of the first type and the second type
        matchup.push(numbers_type1[i] * numbers_type2[i]);
    }

    //here we return the type effectiveness of the first and second type(if there exists a second type)
    matchup //no ; character means return in rust
}

//this function returns a vector that contains the types that are neutral against the pokemon
fn type_neutral(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut neutral = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 1.0
        {
            neutral.push(i);
        }
    }

    neutral
}

//this function returns a vector that contains the types that this pokemon is weak to
fn type_weakness(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut weakness = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 2.0
        {
            weakness.push(i);
        }
    }

    weakness
}

//this function returns a vector that contains the types that this pokemon is quad weak to
fn type_quadWeakness(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut weakness = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 4.0
        {
            weakness.push(i);
        }
    }

    weakness
}

//this function returns a vector that contains the types that are resisted by the pokemon
fn type_resistance(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut resistance = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 0.5
        {
            resistance.push(i);
        }
    }

    resistance
}

//this function returns a vector that contains the types that are double resisted by the pokemon
fn type_doubleResistance(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut resistance = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 0.25
        {
            resistance.push(i);
        }
    }

    resistance
}

//this function returns a vector that contains the types that the pokemon is immune to
fn type_immunity(type1: i32, type2: i32) -> Vec<usize>
{
    let mut matchup = type_effectiveness(type1, type2);
    let mut immunity = Vec::new();

    for i in 0..18
    {
        if matchup[i] == 0.0
        {
            immunity.push(i);
        }
    }

    immunity
}

//this function returns a string that contains the name of the type(it converts a numeric value to a string)
fn get_type(type1: i32) -> String
{
    match type1
    {
        0 => String::from("NORMAL"),
        1 => String::from("FIGHTING"),
        2 => String::from("FLYING"),
        3 => String::from("POISON"),
        4 => String::from("GROUND"),
        5 => String::from("ROCK"),
        6 => String::from("BUG"),
        7 => String::from("GHOST"),
        8 => String::from("STEEL"),
        9 => String::from("FIRE"),
        10 => String::from("WATER"),
        11 => String::from("GRASS"),
        12 => String::from("ELECTRIC"),
        13 => String::from("PSYCHIC"),
        14 => String::from("ICE"),
        15 => String::from("DRAGON"),
        16 => String::from("DARK"),
        17 => String::from("FAIRY"),
        _ => String::from("TYPELESS")
    }
}

//this function prints the text explaining what the typings refer to and separates them with commas
//this was initially in the pokemon_type function, but i made it a separate function for cleaner code and less
//redundancy
fn print_effectiveness(message: String, vec: Vec<usize>)
{

    if(vec.len() == 0)
    {
        println!("{} none", message);
        return;
    }

    let mut counter: usize = 0;
    let mut length: usize = vec.len();

    print!("{}", message);
    for i in vec
    {
        counter += 1;
        if counter == length
        {
            print!("{}", get_type(i as i32));
            break;
        }
        print!("{}", get_type(i as i32) + ", ");
    }
    println!();
    counter = 0;
}

//here we call the functions and print the results of the type effectiveness
fn pokemon_type(type1: i32, type2: i32)
{
    if type2 < 0 || type2 > 17
    {
        println!("Pokemon's Type: {}", get_type(type1));
    }else
    {
        println!("Pokemon's Type: {} and {}", get_type(type1), get_type(type2));
    }

    let neutral: Vec<usize> = type_neutral(type1, type2);
    let weak: Vec<usize> = type_weakness(type1, type2);
    let quad_weak: Vec<usize> = type_quadWeakness(type1, type2);
    let resistance: Vec<usize> = type_resistance(type1, type2);
    let double_resistance: Vec<usize> = type_doubleResistance(type1, type2);
    let immunity: Vec<usize> = type_immunity(type1, type2);

    print_effectiveness("Types that are neutral against this pokemon(x1): ".to_string(), neutral);
    print_effectiveness("Types that this pokemon is weak to(x2): ".to_string(), weak);
    print_effectiveness("Types that this pokemon is quad weak to(x4): ".to_string(), quad_weak);
    print_effectiveness("Types that this pokemon resists(x0.5): ".to_string(), resistance);
    print_effectiveness("Types that this pokemon double resists(x0.25): ".to_string(), double_resistance);
    print_effectiveness("Types that this pokemon is immune to(x0): ".to_string(), immunity);
}


//here we call the pokemon_type function that shows us the results of the type effectiveness of the chosen type/s
fn main() 
{
    
    //example of a single type check(normal in this case): pokemon_type(13, 20);
    //example of a dual type check(Psychic and Ice in this case): pokemon_type(13, 14);

    pokemon_type(13, 20); //because 20 is an out of bounds value our function checks 
    //just a singular type to allow for singular type checking instead of a dual type checking
    //(this is on purpose to allow single type checks)
}