#[cfg(test)]
mod tests {
    use cli_graph::core::line;
    use std::f32;
    #[test]
    fn equals_x() {
        let max = 20usize;
        let mut data:Vec<f32> = Vec::with_capacity(max);
        for i in 0..max {
            data.push(i as f32);
        }
        //println!("data:\n{:.2?}",data);
        line(0f32,max as f32,&data,max,2usize);

        assert!(false);
    }
    #[test]
    fn minus_x() {
        let max = 20usize;
        let mut data:Vec<f32> = Vec::with_capacity(max);
        for i in 0..max {
            data.push(-(i as f32));
        }
        //println!("data:\n{:.2?}",data);
        line(-(max as f32),0f32,&data,max,2usize);

        assert!(false);
    }
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
        let max = 32usize;
        let pi = f32::consts::PI;
        let eighth = 1f32/16f32;
        let mut data:Vec<f32> = Vec::with_capacity(max);
        for i in 0..max {
            data.push((i as f32 * pi * eighth).sin());
        }
        line(-1f32,1f32,&data,max,2usize);

        assert!(false);
    }
    #[test]
    fn cos() {
        let max = 32usize;
        let pi = f32::consts::PI;
        let eighth = 1f32/16f32;
        let mut data:Vec<f32> = Vec::with_capacity(max);
        
        for i in 0..max {
            data.push((i as f32 * pi * eighth).cos());
        }
        line(-1f32,1f32,&data,max,1usize);

        assert!(false);
    }
    #[test]
    fn tan() {
        let max = 40usize;
        let pi = f32::consts::PI;
        let eighth = 1f32/32f32;
        let mut data:Vec<f32> = Vec::with_capacity(max);
        
        for i in 0..max {
            data.push((i as f32 * pi * eighth).tan());
        }
        line(-1f32,1f32,&data,max,1usize);

        assert!(false);
    }
}
