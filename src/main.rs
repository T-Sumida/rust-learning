fn main() {
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }
    }

    // 文字列
    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
    }

    // Hash Map
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    {
        use std::collections::HashMap;

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}
