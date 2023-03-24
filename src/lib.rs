pub struct TableOptions {
    pub table_color: usize,
    pub header_color: usize,
    pub entry_color: usize
}

#[macro_export]
macro_rules! print_as_table {
    ($data:expr, $opts:expr) => {
        // Indexies                      0    1    2    3    4    5    6    7    8    9   10
        let box_style: Vec<&str> = vec!["═", "║", "╔", "╦", "╗", "╠", "╬", "╣", "╚", "╩", "╝"];

        let rows: usize = $data.len();
        let cols: usize = $data[0].len();

        let mut largest_in: Vec<usize> = vec![];
        for col in 0..cols {
            let mut c: usize = 0;
            for row in 0..rows {
                if $data[row][col].len() > c {c = $data[row][col].len();}
            }
            largest_in.push(c);
        }

        for row in 0..rows {

            // Top Row
            if row == 0 {
                print!("\x1b[38;5;{}m", $opts.table_color);
                for col in 0..cols {
                         if col == 0 {print!("{}",box_style[2]);}
                    else             {print!("{}",box_style[3]);}
                    for _ in 0..largest_in[col] {print!("{}",box_style[0]);}
                }
                println!("{}\x1b[0m",box_style[4]);
            }

            // Entries
            for col in 0..cols {
                print!("\x1b[38;5;{}m{}",$opts.table_color,box_style[1]);
                let col_size: usize = largest_in[col];
                let mut color: usize = $opts.entry_color;
                if row == 0 {color = $opts.header_color;}
                print!("\x1b[38;5;{}m{: <col_size$}", color, $data[row][col]);
            }
            println!("\x1b[38;5;{}m{}\x1b[0m",$opts.table_color,box_style[1]);

            // In-Betweens
            if row != rows-1 {
                print!("\x1b[38;5;{}m", $opts.table_color);
                for col in 0..cols {
                         if col == 0 {print!("{}",box_style[5]);}
                    else             {print!("{}",box_style[6]);}
                    for _ in 0..largest_in[col] {print!("{}",box_style[0]);}
                }
                println!("{}\x1b[0m",box_style[7]);
            }

            // Bottom Row
            if row == rows-1 {
                print!("\x1b[38;5;{}m", $opts.table_color);
                for col in 0..cols {
                         if col == 0 {print!("{}",box_style[8]);}
                    else             {print!("{}",box_style[9]);}
                    for _ in 0..largest_in[col] {print!("{}",box_style[0]);}
                }
                println!("{}\x1b[0m",box_style[10]);
            }

        }
    };
    ($data:expr) => {
        print_as_table!($data,TableOptions {
            table_color: 15,
            header_color:14,
            entry_color: 15
        });
    };
}