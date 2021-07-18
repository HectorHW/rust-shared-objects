mod object;
use object::*;

fn main() {

    let mut stack:Vec<StackObject> = vec![];
    let map = StackObject::make::<ObjectMap>();
    let map2 = map.ref_clone();

    stack.push(map);
    stack.push(map2);

    {
        //TODO: write nice api to work with stack objects like normal collections
        //reference from stack slot 0
        let stack_obj: &mut StackObject = stack.get_mut(0).unwrap();
        //in real language use type checking instead of unwrapping and throw exception if necessary

        let map_rc = stack_obj.as_map_rc().unwrap();
        map_rc.borrow_mut().insert(StackObject::make_from(1), StackObject::make_from(8));
    }

    {
        //reference from stack slot 1, should point to same object in memory
        let stack_obj: &mut StackObject = stack.get_mut(1).unwrap();
        //in real language use type checking instead of unwrapping and throw exception if necessary
        let map_rc = stack_obj.as_map_rc().unwrap();

        println!("{}", map_rc.borrow().get(&StackObject::make_from(1)).unwrap().as_int().unwrap());
    }

    {
        let stack_0 = stack.get(0).unwrap();
        let stack_1 = stack.get(1).unwrap();

        dbg!(stack_0==stack_1);
    }
}
