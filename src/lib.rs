
/// Contains core functionality
pub mod core {
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

        println!("       │");

        for i in (1usize..inner_max as usize + 2usize).rev() {
            vertical_scale(data,&inner_data,i);

            for t in 0..inner_data.len() {
                print!("{: <1$}","",h_spacing);
                if inner_data[t] < i as f32 && inner_data[t] >= (i-1) as f32  {
                    print!("*");
                }
                else {
                    print!(" ");
                }
                
            }
            
            println!();
        }
        horizontal_scale(data,&inner_data,h_spacing);
        println!();

        // TODO Adjust this to scale properly
        fn vertical_scale(data:&Vec<f32>,inner_data:&Vec<f32>,i:usize) {
            let mut increment_print = false;
            
            for t in 0..inner_data.len() {
                if inner_data[t] as usize == i - 1 {
                    print!(" {:+.2} ",data[t]);
                    increment_print = true;
                    break;
                }
            }
            if !increment_print {
                print!("       ");
            }
            print!("│");
        }

        fn horizontal_scale(data:&Vec<f32>,inner_data:&Vec<f32>,h_spacing:usize) {
            let v_axis_offset = 7usize;
            print!("{: <1$}","",v_axis_offset);
            println!("└{:─<1$}","",((inner_data.len()+1)*(h_spacing+1)) + 3);

            // Prints x-axis scale
            print!("{: <1$}","",v_axis_offset+1);
            for i in 0..inner_data.len() {
                print!("{: <1$}","",h_spacing);
                print!("{}",i%10);
            }
            print!("\n{: <1$}","",v_axis_offset+1);
            for _ in 0..inner_data.len() / 10 {
                print!("└{:─<1$}┘","",(10*(h_spacing+1))-2);
            }
            let remainder_labels = inner_data.len()%10;
            let remainder_space = remainder_labels*(h_spacing+1);
            print!("└{:─<1$}","",remainder_space);
            print!("\n{: <1$}","",v_axis_offset+h_spacing);
            print!("{: <1$}","",5*(h_spacing+1));
            for i in 0..inner_data.len() / 10 {
                print!("{:0>2}",i);
                print!("{: <1$}","",(10*(h_spacing+1))-2);
            }
        } 
    }
}