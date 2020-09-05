fn print_coordinates(&(x, y): &(i32, i32)) {
    // 現在の位置: ({}, {})
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = true;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            // あなたのお気に入りの色、{}を背景色に使用します
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            // 火曜日は緑の日！
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                // 紫を背景色に使用します
                println!("Using purple as the background color");
            } else {
                // オレンジを背景色に使用します
                println!("Using orange as the background color");
            }
        } else {
            // 青を背景色に使用します
            println!("Using blue as the background color");
        }
    }
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    {
        let point = (3, 5);
        print_coordinates(&point);
    }
    {
        let x = Some(5);
        let y = 10;

        match x {
            // 50だったよ
            Some(50) => println!("Got 50"),
            // マッチしたよ
            Some(y) => println!("Matched, y = {:?}", y),
            // 既定のケース
            _ => println!("Default case, x = {:?}", x),
        }

        // 最後にはx = {}, y = {}
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
}
