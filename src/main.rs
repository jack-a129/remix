#[derive(Debug)]
enum Message{
	Add,
	Sub,
	Mul,
	Div,
	Push,
	Pop,
	Print,
	Line,
}
#[derive(Debug)]
enum Res{
	Stack,
	NomalA,
	NomalB,
	NomalC,
	Ret,
}

#[derive(Debug)]
enum Opecode{
	Num(i64),
	Point(usize),
	Res(Res),
	Nil,
}

fn error(mes :&str){
	println!("{}",mes);
	std::process::exit(-1);
}

fn atoi(x :&str) -> i64{
	let num :Result<i64, std::num::ParseIntError> = x.to_string().trim().parse();
	if let Ok(n) = num{
	   return n as i64;
	}
	error("num error");
	return 0
}

fn opecode_match(line :&String) -> Option<(Message,Opecode,Opecode)>{
	let split :Vec<&str> = line.split_whitespace().collect();
	let mes = match split[0]{
		"add" => Message::Add,
		"sub" => Message::Sub,
		"mul" => Message::Mul,
		"div" => Message::Div,
		"push" => Message::Push,
		"pop" => Message::Pop,
		"ioprint" => Message::Print,
		"P" => Message::Line,
		_ => return None,
	};
	Some((mes,Opecode::Nil,Opecode::Nil))
}

fn maketree(code :&str) -> Vec<Vec<(Message, Opecode, Opecode)>>{
	let code = code.to_string();
	let lines :Vec<String> = code.split("\n").map(|x|{
		x.to_string()
	}).collect();
	let mut area :Vec<Vec<(Message,Opecode,Opecode)>> = Vec::new();
	for mut line in lines{
		line = line.replace("\t", "");
		if line == String::from(""){continue;}
		let opecode = opecode_match(&line);
		if let Some(n) = opecode{
			match n.0{
				Message::Line => area.push(Vec::new()),
				_ => area.last_mut().unwrap().push(n),
			}
		}
	}
	area
}

fn eval(code :&str){
	let tree = maketree(code);
	for x in tree{
		println!("{:?}",x);
	}
}

fn main() {
	let code = r#"
		P one
		push 12
		push 20
		add stack 1
		ioprint [stack]
		P two
		pop nomalA
		ioprint nomalA
		P main
		pop 10
	"#;
	eval(code);
}
