fn main() {
    // 3.1 - 3.2
    {
        let x = 5;

        let x = x + 1;

        let x = x * 2;

        println!("The value of x is: {}", x);

        let spaces = "   ";
        let spaces = spaces.len();
        println!("spaces {}", spaces);

        let mut spaces = "   ";
        spaces = spaces.len();


        // データ型について
        let guess: u32 = "42".parse().expect("Not a number!"); 


        // 浮動小数点について
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32


        // 数値演算
        // 足し算
        let sum = 5 + 10;

        // 引き算
        let difference = 95.5 - 4.3;

        // 掛け算
        let product = 4 * 30;

        // 割り算
        let quotient = 56.7 / 32.2;

        // 余り
        let remainder = 43 % 5;
        println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);


        // 論理値型
        let t = true;

        let f: bool = false; // 明示的型注釈付きで
        println!("{}, {}",t, f );


        // 文字型
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻'; 
        println!("{}, {}, {}", c, z , heart_eyed_cat);

        // 複合型:タプル
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
        println!("{}, {}, {}", five_hundred, six_point_four, one);

        
        // 複合型：配列
        let a = [1, 2, 3, 4, 5];
        let index = 10;

        let element = a[index];

        println!("The value of element is: {}", element);   // 要素の値は{}です
    }

    // 3.3
    {
        println!("Hello, world!");
        another_function(5, 6);
        
        let x = 5;
        // 式は終端にセミコロンは不必要
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);

        let x = plus_one(5);

        println!("The value of x is: {}", x);
    }

    // 3.5
    {
        let number = 3;

        if number < 5 {
            println!("condition was true");       // 条件は真でした
        } else {
            println!("condition was false");      // 条件は偽でした
        }

        if
        let number = 3;

        if number != 0 {
            println!("number was something other than zero");   // 数値は0以外の何かです
        }


        // 複数条件
        let number = 7;

        if number % 4 == 0 {
            // 数値は4で割り切れます
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            // 数値は3で割り切れます
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            // 数値は2で割り切れます
            println!("number is divisible by 2");
        } else {
            // 数値は4、3、2で割り切れません
            println!("number is not divisible by 4, 3, or 2");
        }

        
        // let文内でのif
        let condition = true;
        let number = if condition {
                5
            } else {
                6
            };

        // // numberの値は、{}です
        println!("The value of number is: {}", number);


        // 無限loop
        loop {
            println!("again!");   // また
        }


        // whileの条件付きループ
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number = number - 1;
        }

        // 発射！
        println!("LIFTOFF!!!");


        // コレクションループ
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            // 値は{}です
            println!("the value is: {}", element);
        }

        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}