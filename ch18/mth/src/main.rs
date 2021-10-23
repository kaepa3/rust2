fn main() {
    {
        println!("------------------------------");
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        println!("------------------------------");
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Defaut case, x = {:?}", x),
        }
        println!("at the end :x {:?}, y {:?}", x, y)
    }
    {
        println!("------------------------------");
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        println!("------------------------------");
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    {
        println!("------------------------------");
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    {
        println!("------------------------------");
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("using your fav color{}", color);
        } else if is_tuesday {
            println!("tuesday is green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("using perple as ....");
            } else {
                println!("using orange as ....")
            }
        } else {
            println!("using blue");
        }
    }
    {
        println!("------------------------------");
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("somthing else"),
        }
    }
    {
        let x = 'c';
        match x {
            'a'..='j' => println!("early"),
            'k'..='z' => println!("late"),
            _ => println!("something else"),
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }
    {
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("on the x axis at {}", x),
            Point { x: 0, y } => println!("on the y axis at {}", y),
            Point { x, y } => println!("on the y axis at ({}:{})", x, y),
        }
    }
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("quit!!!!");
            }
            Message::Move { x, y } => {
                println!("move x:{} y:{}", x, y);
            }
            Message::Write(text) => println!("message:{}", text),
            Message::ChangeColor(r, g, b) => {
                println!("r:{} g:{} b:{}", r, g, b);
            }
        }
    }
    {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let some_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        println!("sum:{}", some_of_squares);
    }
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    {
        let origin = Point3D { x: 0, y: 0, z: 0 };
        match origin {
            Point3D { x, .. } => println!("x is {}", x),
        }
    }
    {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Sobe numbers: {}, {}", first, last);
            }
        }
        match numbers {
            (_, second, ..) => {
                println!("second numbers:{}", second);
            }
        }
    }
    {
        let robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }
    {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five:{}", x),
            Some(x) => println!("val:{}", x),
            None => (),
        }
    }
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("match n={:?}", n),
            _ => println!("def x={:?}", x),
        }
        println!("at the end: x={:?}, y={:?}", x, y);
    }
    {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
    {
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello {
                id: id_variable @ 3...7,
            } => {
                println!("found an id  in range {}", id_variable)
            }
            Message::Hello { id: 10...12 } => {
                println!("found an id in anothre range ")
            }
            Message::Hello { id } => {
                println!("found some othre id :{}", id)
            }
        }
    }
}
