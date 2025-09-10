fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    let x: u8 = 255;
    println!("x is {}", x);

    let x = 10.1234567898765456765434567;
    println!("x is {}", x);

    let x: f32 = 10.1234567898765456765434567;
    println!("x is {}", x);

    let a = 10;
    let b = 3.0; 
    let c = a as f64 / b;
    print!("c is {0:08.3}\na is {1}\noncee again, c is {0}", c, a);

    let mut value = 0b1111_0101u8;
    println!("\nvalue is {}", value);
    println!("value is {:08b}", value);

    value =!value;
    println!("value is {:08b}", value);

    value = value ^ 0b11110101;
    println!("value is {:08b}", value);

    value = value >> 4;
    println!("value is {:08b}", value);

    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c = (a ^ b) || (a & b);
    println!("c is {}", c);

    let a =  1;
    let b =  2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("letter is {}\nnumber is {}\nfinger is {}", letter, number, finger);

    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("last number is {}", numbers[4]);

    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let number = parking_lot[0][1];
    println!("number is {}", number);

    let garage= [[[0; 100]; 20];5];
    
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);

    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    let result = square(13);
    println!("result is {:?}", result);

    let x = 3;
    let y = 5;

    if x + 1 != 3 {
        println!("x + 1  is NOT 3!");
    } 

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
       
    } 
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };
    println!("x is {}", x);

    let mut count = 0;

   let result =  loop{
        if count == 10 {
            break count *  10;
        }
        count +=1;
        println!("count is {}", count);
    };
    println!("After the loop!");
    println!("result is {}", result);

    let mut count = 0;
    let letters = ['a', 'b', 'c'];
    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }

    let message = ['h', 'e', 'l', 'l', 'o'];
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }

    let mut matrix = [[1, 2, 3], 
                  [4, 5, 6], 
                  [7, 8, 9]];
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t ", num);
        }
        println!();  
    }

    
} 
fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
}