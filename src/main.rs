

use std::collections::LinkedList;

fn main(){
    let mut list:LinkedList<u32>=LinkedList::new();
    let mut listtrie:LinkedList<u32>=LinkedList::new();
    list.push_back(5);
    list.push_back(15);
    list.push_back(0);
    list.push_back(21);
    list.push_back(8);
    list.push_back(10);
    trier(list,listtrie);
}
fn trier(mut list:LinkedList<u32>,mut listtrie:LinkedList<u32>){
    let mut lepluspetit=list.front().unwrap();
    let mut i=0;
    let mut index=0;
    for element in list.iter() {
       

        if lepluspetit>element{
            lepluspetit=element;
            index=i;
        }
        i+=1;
     
    }
    listtrie.push_back(*lepluspetit);
    let mut split_list = list.split_off(index);
    split_list.pop_front();
    list.append(&mut split_list);
   
    if list.len()>0{
        trier(list,listtrie);
    }else{
        println!("{:?}",listtrie);
    }


}