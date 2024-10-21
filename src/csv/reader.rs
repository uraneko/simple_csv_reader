use std::fs::File;
use std::io::Read;
// use std::mem;

pub const PLACEHOLDER_FILE: &str = r"./placeholder.csv";

fn open_file(path: &str) -> File {
    let f = File::open(path);
    match f {
        Ok(file) => file,
        Err(e) => panic!("not such file"),
    }
}
fn read_file_to_buffer(mut file: File) -> String {
    let mut b = String::new();
    let res = file.read_to_string(&mut b);
    match res {
        Ok(n) => (),
        Err(e) => panic!("file contains non utf8 characters"),
    }
    // println!("{:#?}", &b);

    b.replace("\"\"", "\"")
}

fn read_buffer_to_table(buffer: String) -> (usize, Vec<String>) {
    let cols = break_line_and_sort(buffer.lines().next().unwrap().to_string()).len();
    let mut lines = buffer.lines();
    let mut tab: Vec<String> = Vec::new();
    for line in lines {
        // let cells = line.multi_split(vec![",\"", "\",", ","]);
        let mut line_s = String::from(line);
        let mut cells = break_line_and_sort(line_s);
        tab.append(&mut cells);
    }
    // println!("{:#?}", tab);

    (cols, tab)
}

pub fn full_read(path: &str) -> (usize, Vec<String>) {
    let mut f = open_file(path);
    let b = read_file_to_buffer(f);
    let t = read_buffer_to_table(b);

    t
}

fn break_line_and_sort(mut line: String) -> Vec<String> {
    // println!("{}", line);
    let sorter = String::from(&line[..]);
    let mut dbl: Vec<String> = Vec::new();
    while let (Some(s), Some(e)) = (line.find(",\""), line.find("\",")) {
        if s < e {
            dbl.push(String::from(&line[s + 2..e]));
            line = line.replacen(&line[s..e + 2], ",", 1);
        }
    }
    let mut sgl: Vec<String> = line
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut vals: Vec<(usize, String)> = Vec::with_capacity(dbl.len() + sgl.len());
    // println!("{:?}", dbl);
    // println!("{:?}", sgl);
    let mut all = sgl.clone();
    sgl.iter()
        .enumerate()
        .map(|(i, s)| {
            for val in &dbl {
                let pat = s.to_string() + ",\"" + &val;
                if let Some(_) = sorter.find(&pat) {
                    // println!("{}", &pat[..]);
                    // + diff of lens since the len of all changes making the i of s actually i + 1
                    // with every value added
                    all.insert(i + 1 + all.len() - sgl.len(), val.clone());
                }
            }
        })
        .collect::<()>();
    // println!("{:?}\n", all);

    all
}

// fn break_line<'a>(line: &'a str) -> Vec<&'a str> {
//     let mut dbl_q: Vec<(usize, usize)> = Vec::new();
//     let mut pos = 0;
//     while let (Some(start), Some(end)) = (line[pos..].find(",\""), line[pos..].find("\","))
//         && start < end
//     {
//         dbl_q.push((start + pos, end + pos));
//         pos = dbl_q[dbl_q.len() - 1].1 + 1;
//     }
//     // let new_line = &mut line;
//     // for (s, e) in dbl_q {
//     //     new_line = new_line[..s].to_string() + new_line[e..];
//     // }
//     // println!("{}", 0);
//     let mut sgl_q: Vec<(usize, usize)> = Vec::new();
//     let mut pos = 0;
//     while let Some(index) = line[pos..].find(",")
//         && dbl_q.iter().any(|(s, e)| !(s..e).contains(&&index))
//     {
//         let next_i = if let Some(i) = line[index + pos + 1..].find(",") {
//             i + index + pos + 1
//         } else {
//             line.len() - 1
//         };
//         sgl_q.push((index + pos, next_i));
//         pos = sgl_q[sgl_q.len() - 1].0 + 1;
//         // println!("{:?}", sgl_q);
//     }
//     // println!("{}", 2);
//     let mut all_q = Vec::new();
//     all_q.append(&mut dbl_q);
//     all_q.append(&mut sgl_q);
//     all_q.sort_by(|(s, e), (s1, e1)| s.partial_cmp(s1).unwrap());
//     let mut col: Vec<&'a str> = Vec::with_capacity(all_q.len());
//     for (s, e) in all_q {
//         let val = &line[s..e];
//         col.push(val);
//     }
//     println!("--- column: {:?}", col);
//
//     col
//
// }

trait StringUtils {
    fn multi_split(&self, args: Vec<&str>) -> Vec<String>;
    // fn mirror(&mut self, ori: String) -> Self;
}

impl StringUtils for &str {
    fn multi_split(&self, args: Vec<&str>) -> Vec<String> {
        let mut vec: Vec<&str> = self.split(args[0]).collect();
        // args[1..].iter().map(|a| {
        // vec = vec.iter().map(|s| s.split(a).map(|ss| ss.to_string()).collect::<Vec<String>>()).collect::<Vec<String>>();
        // });
        // println!("{:?}", vec);
        for arg in &args[1..] {
            // println!("{:?}", vec);
            let mut v: Vec<&str> = Vec::new();
            vec.iter().for_each(|s| {
                s.split(arg).for_each(|ss| v.push(ss));
            });
            vec = v;
            // println!("{:?}", vec);
        }

        vec.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }
    //
    // fn mirror(&mut self, ori: String) -> Self {
    //     let chs: Vec<char> = Vec::new();
    //     for (c, c1) in std::iter::zip(self.chars(), ori.chars()) {
    //         if c != c1 {
    //
    //         }
    //     }
    //
    //     &String::from_iter(chs)
    // }
}
