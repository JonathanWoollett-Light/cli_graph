
pub mod core {
    pub fn line(min:f32,max:f32,data:&Vec<f32>,height:usize,h_spacing:usize) {
        
        if height < data.len() {
            panic!("`height` must be more than `(max-min).abs()`. Cannot squeeze data.");
        }

        // Sets scaled data
        let mut inner_data = data.clone();
        //println!("inner_data.len():{}",inner_data.len());
        //println!("x_scale:{}",x_scale);
        let y_scale:f32 = height as f32 / (max-min).abs();
        //println!("y_scale:{}",y_scale);
        // If `y_scale != 1f32` 
        if y_scale > 1f32 {
            for i in 0..inner_data.len() {
                inner_data[i] *= y_scale;
            }
        }
        //println!("min:{}, max:{}, height:{}",min,max,height);

        let inner_min = y_scale * min;
        let mut inner_max = y_scale * max;
        //println!("inner_min:{}, inner_max:{}",inner_min,inner_max);
        //println!("inner_data:{:.1?}",inner_data);
        // Prints graph
        println!();

        inner_max -= inner_min;
        for val in &mut inner_data {
            *val = *val - inner_min;
        }

        //println!("inner_min:{}, inner_max:{}",inner_min,inner_max);
        //println!("inner_data:{:.1?}",inner_data);

        println!("       │");

        for i in (1usize..inner_max as usize + 2usize).rev() {
            //println!("i:{}",i);
            vertical_scale(data,&inner_data,i);

            for t in 0..inner_data.len() {
                //println!("inner_data[t]: {} <&>= {}",inner_data[t], i as f32);
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

        fn vertical_scale(data:&Vec<f32>,inner_data:&Vec<f32>,i:usize) {
            let mut increment_print = false;
            
            for t in 0..inner_data.len() {
                //println!("inner_data[t].1 as usize: {}",inner_data[t].1 as usize);
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
        // TODO Adjust this to be like the horizontal scale in my `splitter` library
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

#[cfg(test)]
mod tests {
    use crate::core::line;
    use std::f32;
    #[test]
    fn up_pyramid() {
        let data = vec![
            1f32,2f32,3f32,4f32,5f32,4f32,3f32,2f32,1f32
        ];
        line(1f32,5f32,&data,10usize,0usize);

        assert!(false);
    }
    #[test]
    fn down_pyramid() {
        let mut data = vec![
            1f32,2f32,3f32,4f32,5f32,4f32,3f32,2f32,1f32
        ];
        for val in &mut data {
            *val = -(*val);
        }
        line(-5f32,-1f32,&data,10usize,2usize);

        assert!(false);
    }
    #[test]
    fn sin() {
        let pi = f32::consts::PI;
        let eighth = 1f32/16f32;
        let mut data:Vec<f32> = Vec::with_capacity(33usize);
        for i in 0..33 {
            //println!("({}).sin():{}",i as f32 * pi * eighth,(i as f32 * pi * eighth).sin());
            data.push((i as f32 * pi * eighth).sin());
        }
        //println!("data:\n{:.2?}",data);
        line(-1f32,1f32,&data,33usize,2usize);

        assert!(false);
    }
    #[test]
    fn cos() {
        let pi = f32::consts::PI;
        let eighth = 1f32/16f32;
        let mut data:Vec<f32> = Vec::with_capacity(33usize);
        
        for i in 0..33 {
            //println!("({}).sin():{}",i as f32 * pi * eighth,(i as f32 * pi * eighth).sin());
            data.push((i as f32 * pi * eighth).cos());
        }
        //println!("data:{:1.?}",data);
        //println!("data:\n{:.2?}",data);
        line(-1f32,1f32,&data,33usize,1usize);

        assert!(false);
    }
    #[test]
    fn tan() {
        let pi = f32::consts::PI;
        let eighth = 1f32/32f32;
        let mut data:Vec<f32> = Vec::with_capacity(41usize);
        
        for i in 0..41 {
            data.push((i as f32 * pi * eighth).tan());
        }
        //println!("data:{:1.?}",data);
        //println!("data:\n{:.2?}",data);
        line(-1f32,1f32,&data,41usize,1usize);

        assert!(false);
    }
}
