mod composition {
    pub struct Composition<T>{
        net : T,
        parts : Vec<T>,
        initial_value : T,
        fold_fun : fn(T,T) -> T,
        is_commutative : bool
    }

    pub fn check_fold<T : Clone+std::cmp::PartialEq>(x : &Composition<T>) -> bool {
        return x.net == x.parts.iter().fold(x.initial_value.clone(),|a, b| (x.fold_fun)(a,(*b).clone()));
    }

    pub fn num_parts<T>(x : &Composition<T>) -> usize {
        return x.parts.len();
    }

    pub fn singleton<T : Clone>(net : T, initial_value : T, fold_fun : fn(T,T) -> T,is_commutative : bool) -> Composition<T> {
        let mut z = Vec::new();
        z.push(net.clone());
        return Composition {net : net,parts : z,initial_value : initial_value,fold_fun : fold_fun, is_commutative : is_commutative}
    }

    pub fn construct<T : Clone>(parts : Vec<T>, initial_value : T, fold_fun : fn(T,T) -> T, is_commutative : bool) -> Composition<T> {
        let n = parts.iter().fold(initial_value.clone(),|a, b| fold_fun(a,(*b).clone()));
        return Composition {net : n,parts : parts,initial_value : initial_value,fold_fun : fold_fun, is_commutative : is_commutative};
    }

    pub fn combine<T : std::cmp::PartialEq>(x : Composition<T>, y : Composition<T>, check_valid : bool) -> Composition<T>{
        let my_initial_value = if !check_valid || x.initial_value==y.initial_value {
            x.initial_value
        }
        else {
            panic!("Both initial values must be the same")
        };
        let my_fold_fun = if !check_valid || x.fold_fun==y.fold_fun {
            x.fold_fun
        }
        else {
            panic!("Both folding functions must be the same")
        };
        let my_is_commutative = x.is_commutative && y.is_commutative;
        let mut z = Vec::new();
        let mut xmod = x.parts;
        let mut ymod = y.parts;
        z.append(&mut xmod);
        z.append(&mut ymod);
        return Composition {net:my_fold_fun(x.net,y.net), parts : z, initial_value : my_initial_value, fold_fun : my_fold_fun, is_commutative : my_is_commutative};
    }

    pub fn decompose<T : std::cmp::PartialEq>(x : & mut Composition<T>, y : Composition<T>, idx : usize, check_valid : bool) {
        if check_valid && !(x.initial_value==y.initial_value) {
            panic!("Both initial values must be the same")
        };
        if check_valid && !(x.fold_fun==y.fold_fun) {
            panic!("Both folding functions must be the same")
        };
        if !check_valid || x.parts[idx]==y.net {
            x.parts.splice(idx..idx+1,y.parts)
        }
        else{
            panic!("The net result of the composition to be inserted did not match with what the single part of the composition it was supposed to replace")
        };
        return;
    }

    pub fn split<T : Clone>(x : &Composition<T>,predicate : fn(&T) -> bool) -> (Composition<T>,Composition<T>) {
        if !(*x).is_commutative{
            panic!("The operation must be commutaive for the parts to be split according to a proposition otherwise result changes upon recombining")
        }
        let (satisfiers,unsatisfiers) : (Vec<T>,Vec<T>) = (*x).parts.iter().cloned().partition(|z| predicate(&z));
        let satisfying = construct(satisfiers, (*x).initial_value.clone(), (*x).fold_fun, true);
        let unsatisfying = construct(unsatisfiers, (*x).initial_value.clone(), (*x).fold_fun, true);
        return (satisfying,unsatisfying);
    }

    /* pub fn split_in_place<T>(x : & mut Composition<T>,predicate : fn(&T) -> bool) -> usize {
        if !(*x).is_commutative{
            panic!("The operation must be commutaive for the parts to be reordered according to a proposition")
        }
        let satisfiers = (*x).parts.iter_mut().partition_in_place(predicate);
        return satisfiers;
    } */

    pub fn to_string<T : std::fmt::Debug + std::fmt::Display>(x : &Composition<T>) -> String {
        return format!("{:?} of {} with {} parts",(*x).parts,(*x).net,num_parts(x));
    }
}

fn main() {
    use composition::Composition;
    let add = |a, b| a + b;
    let x_1 : Composition<u8> = composition::singleton(1,0,add,true);
    assert!(composition::check_fold(&x_1));
    let x_1_copy = composition::singleton(1,0,add,true);
    assert!(composition::check_fold(&x_1_copy));
    let x_2 : Composition<u8> = composition::singleton(2,0,add,true);
    assert!(composition::check_fold(&x_2));
    let x_12 = composition::combine(x_1,x_2,true);
    assert!(composition::check_fold(&x_12));
    let mut x_121 = composition::combine(x_12,x_1_copy,true);
    assert!(composition::check_fold(&x_121));
    println!("{:?}",composition::to_string(&x_121));
    let (x_1s,x_2s) = composition::split(&x_121,|z| (*z)==1);
    println!("{:?} splits to {:?} and {:?}",composition::to_string(&x_121),composition::to_string(&x_1s),composition::to_string(&x_2s));
    let x_11 = composition::construct(vec![1,1],0,add,true);
    assert!(composition::check_fold(&x_11));
    composition::decompose(&mut x_121,x_11,1,true);
    assert!(composition::check_fold(&x_121));
    println!("{:?}",composition::to_string(&x_121));
    /*let num_1s = composition::split_in_place(&mut x_121,|z| (*z)==1);
    println!("After moving {} 1's to the beginning, {:?}",num_1s,composition::to_string(&x_121));*/

    let x_11_fake : Composition<f32> = composition::construct(vec![1.0,1.0],0.0,|a,b| a+b,true);
    assert!(composition::check_fold(&x_11_fake));
    
    println!("Operation is now multiplication");
    let z_121 = composition::construct(vec![1,2,1],1,|a,b| a*b,true);
    println!("{:?}",composition::to_string(&z_121));

    println!("Operation is now string concatenation");
    let str_cat = |a:String,b:String| a.to_owned()+"\n"+&b;
    let greeting = composition::singleton("Hello".to_string(),"".to_string(),str_cat,false);
    let banter = composition::singleton("Banter".to_string(),"".to_string(),str_cat,false);
    let goodbye = composition::singleton("Bye".to_string(),"".to_string(),str_cat,false);
    let mut conversation = composition::combine(composition::combine(greeting,banter,false),goodbye,false);
    println!("{:?}",composition::to_string(&conversation));
    let banter_refined = composition::construct(vec!["Banter 1".to_string(),"Banter 2".to_string()],"".to_string(),str_cat,false);
    composition::decompose(&mut conversation,banter_refined,1,false);
    println!("{:?}",composition::to_string(&conversation));
    if !composition::check_fold(&conversation){
        println!("The lack of check valid means that it does not fold to give the exact same result as before");
    }
    /* errors because operation is not commutative so can't partition and rearrange steps freely
    let (_greeting_part,_rest) = composition::split(&conversation,|z| (*z)=="Hello"); */

}