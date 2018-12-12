mod excel;
mod database;

fn main() {
    excel::readfile();

    /*    database::execute();

    println!("Hello, world!");
    println!("Servus Anton");
    let mut s:String = "Hallo".to_string(); // String literals are refs, have to be converted?
    let p:&str   = "Foo";
    s.push_str(p);
    println!("{}",s);

    let mut a = [1,17,33];
    fn swap(arr:&mut [i32;3])->[i32;3]{
        let temp = arr[0];
        arr[0]=arr[2];
        arr[2]=temp;
        *arr
    }
    let b = swap(&mut a);
    println!("b={:?}",b);
    println!("a={:?}",a);


    enum GRTyp {
        Moc,
        Base,
        Incremental
    }

    const FIRST:&str="0000";

    fn choose(x:GRTyp){
        match x {
            GRTyp::Moc => println!("MOC"),
            GRTyp::Base => println!("BASE"),
            GRTyp::Incremental => println!("{}",FIRST),
        }
    }

    choose(GRTyp::Moc);
    choose(GRTyp::Base);
    choose(GRTyp::Incremental);
    choose(GRTyp::Moc);
    choose(GRTyp::Base);
    choose(GRTyp::Incremental);

    let mut v:Vec<i32> = Vec::new();
    v.push(-33);
    v.push(-38);
    let mut w = vec![77,78,79];
    v.append(&mut w);
    println!("v={:?}, w={:?}",v,w);

*/
}
