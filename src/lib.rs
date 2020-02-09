
/// Contains core functionality
pub mod core {
    use std::io::{Write, Stdout,stdout};
    /// Prints a line graph
    pub fn line(min:f32,max:f32,data:&Vec<f32>,height:usize,h_spacing:usize) {
        
        if height < data.len() {
            panic!("`height` must be more than `(max-min).abs()`. Cannot squeeze data.");
        }

        // Sets scaled data
        let mut inner_data = data.clone();
        let y_scale:f32 = height as f32 / (max-min).abs();
        // If `y_scale != 1f32` 
        if y_scale > 1f32 {
            for i in 0..inner_data.len() {
                inner_data[i] *= y_scale;
            }
        }

        let inner_min = y_scale * min;
        let mut inner_max = y_scale * max;
        // Prints graph
        println!();

        inner_max -= inner_min;
        for val in &mut inner_data {
            *val = *val - inner_min;
        }

        let mut stdout = stdout();

        stdout.write(format!("").as_bytes()).unwrap();
        stdout.write(format!("       │").as_bytes()).unwrap();

        for i in (1usize..inner_max as usize + 2usize).rev() {
            vertical_scale(&mut stdout,data,&inner_data,i);

            for t in 0..inner_data.len() {
                stdout.write(format!("{: <1$}","",h_spacing).as_bytes()).unwrap();
                if inner_data[t] < i as f32 && inner_data[t] >= (i-1) as f32  {
                    stdout.write("*".as_bytes()).unwrap();
                }
                else {
                    stdout.write(" ".as_bytes()).unwrap();
                }
                
            }
            // TODO Is `format!` needed here?
            stdout.write(format!("\n").as_bytes()).unwrap();
        }
        horizontal_scale(&mut stdout,data,&inner_data,h_spacing);
        stdout.write(format!("\n").as_bytes()).unwrap();

        // TODO Adjust this to scale properly
        fn vertical_scale(stdout:&mut Stdout,data:&Vec<f32>,inner_data:&Vec<f32>,i:usize) {
            let mut increment_print = false;
            
            for t in 0..inner_data.len() {
                if inner_data[t] as usize == i - 1 {
                    stdout.write(format!(" {:+.2} ",data[t]).as_bytes()).unwrap();
                    increment_print = true;
                    break;
                }
            }
            if !increment_print {
                stdout.write("       ".as_bytes()).unwrap();
            }
            stdout.write("│".as_bytes()).unwrap();
        }

        fn horizontal_scale(stdout:&mut Stdout,data:&Vec<f32>,inner_data:&Vec<f32>,h_spacing:usize) {
            let v_axis_offset = 7usize;
            stdout.write(format!("{: <1$}","",v_axis_offset).as_bytes()).unwrap();
            stdout.write(format!("└{:─<1$}","",((inner_data.len()+1)*(h_spacing+1)) + 3).as_bytes()).unwrap();
            stdout.write(format!("{: <1$}","",v_axis_offset+1).as_bytes()).unwrap();
            // Prints x-axis scale
            for i in 0..inner_data.len() {
                stdout.write(format!("{: <1$}","",h_spacing).as_bytes()).unwrap();
                stdout.write(format!("{}",i%10).as_bytes()).unwrap();
            }
            stdout.write(format!("\n{: <1$}","",v_axis_offset+1).as_bytes()).unwrap();
            for _ in 0..inner_data.len() / 10 {
                stdout.write(format!("└{:─<1$}┘","",(10*(h_spacing+1))-2).as_bytes()).unwrap();
            }
            let remainder_labels = inner_data.len()%10;
            let remainder_space = remainder_labels*(h_spacing+1);
            stdout.write(format!("└{:─<1$}","",remainder_space).as_bytes()).unwrap();
            stdout.write(format!("\n{: <1$}","",v_axis_offset+h_spacing).as_bytes()).unwrap();
            stdout.write(format!("{: <1$}","",5*(h_spacing+1)).as_bytes()).unwrap();

            for i in 0..inner_data.len() / 10 {
                stdout.write(format!("{:0>2}",i).as_bytes()).unwrap();
                stdout.write(format!("{: <1$}","",(10*(h_spacing+1))-2).as_bytes()).unwrap();
            }
        } 
    }
}