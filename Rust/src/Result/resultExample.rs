enum SoundData{
    alert,
    snooze,
}

fn getSound(name:&str)->Result<SoundData,String>{
    if name=="abc"{
        Ok(SoundData::alert)
    }
    else{
        Err("Unable to find".to_string())
    }
}

fn main(){
    let sound=getSound("alert");
    match sound{
        Ok(_)=>println!("sound data loaded"),
        Err(e)=>println!("err{:?}",e),
    }
}