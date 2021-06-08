/*1-honglvdeng*/
pub enum TraitLight{
    Red,
    Yellow,
    Green,
}

impl TraitLight {
    fn time(&self)->u32{
        match &self{
            TraitLight::Red=>10,
            TraitLight::Green=>20,
            TraitLight::Yellow=>30,
        }
    }
}

/*2-sum*/
pub fn add_sum(ulist:&[u32])->Option<u32>{
    let mut sum:u32 = 0;
    for ele in ulist.iter(){
        sum=sum+ele;
    }
    Some(sum)
}

/*3--x*y*/
pub struct HW<T>{
    width:T,
    height:T,
}

pub fn RectAB<T:std::ops::Mul<Output = T> + Copy>(rect:&HW<T>)->T{
    return rect.width*rect.height
}



fn main() {
    let t = crate::TraitLight::time(&TraitLight::Green);
    println!("1--Green Wait Time => {}",t);

    let ul = [10,20,30,40,50];
    let sum = add_sum(&ul);

    println!("2--sum=>{:?}",&sum);

    let r = HW{width:2,height:3};
    let rect:u32 = RectAB(&r);

    println!("3--x*y=>{:?}",rect);
}
