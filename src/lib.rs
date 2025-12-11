use aegis_core::{Value, NativeFn};
use std::collections::HashMap;
use uuid::Uuid;

#[unsafe(no_mangle)]
pub extern "C" fn _aegis_register(map: &mut HashMap<String, NativeFn>) {
    map.insert("uuid_v1".to_string(), uuid_v1);
    map.insert("uuid_v3".to_string(), uuid_v3);
    map.insert("uuid_v4".to_string(), uuid_v4);
    map.insert("uuid_v5".to_string(), uuid_v5);
    map.insert("uuid_v6".to_string(), uuid_v6);
    map.insert("uuid_v7".to_string(), uuid_v7);
}

// v1: Timestamp + MAC Address (Simulé si pas d'accès réseau bas niveau)
fn uuid_v1(_: Vec<Value>) -> Result<Value, String> {
    // Note: uuid v1 nécessite un contexte (node_id + counter).
    // La crate uuid permet de générer ça, mais c'est plus complexe.
    // Pour simplifier, on utilise une méthode qui gère ça ou on simule.
    // Uuid::new_v1 nécessite Context. Pour un binding simple, c'est parfois compliqué.
    // Simplification : On utilise now_v1 si dispo, sinon on skip ou on mock.
    
    // Uuid >= 1.0 gère v1 via Uuid::now_v1(node_id)
    // On génère un node_id aléatoire pour l'exemple
    let node_id = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]; 
    let uuid = Uuid::now_v1(&node_id);
    Ok(Value::String(uuid.to_string()))
}

// v3: MD5 Hash (Namespace + Name)
fn uuid_v3(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 { return Err("uuid_v3(namespace_uuid, name)".into()); }
    
    let ns_str = args[0].as_str()?;
    let name = args[1].as_str()?;
    
    let ns = Uuid::parse_str(&ns_str).map_err(|e| e.to_string())?;
    let uuid = Uuid::new_v3(&ns, name.as_bytes());
    
    Ok(Value::String(uuid.to_string()))
}

// v4: Random (Le standard)
fn uuid_v4(_: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(Uuid::new_v4().to_string()))
}

// v5: SHA-1 Hash (Namespace + Name) - Préféré à v3
fn uuid_v5(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 { return Err("uuid_v5(namespace_uuid, name)".into()); }
    
    let ns_str = args[0].as_str()?;
    let name = args[1].as_str()?;
    
    let ns = Uuid::parse_str(&ns_str).map_err(|e| e.to_string())?;
    let uuid = Uuid::new_v5(&ns, name.as_bytes());
    
    Ok(Value::String(uuid.to_string()))
}

// v6: Reordered Time (Comme v1 mais triable)
fn uuid_v6(_: Vec<Value>) -> Result<Value, String> {
    let node_id = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let uuid = Uuid::now_v6(&node_id);
    Ok(Value::String(uuid.to_string()))
}

// v7: Unix Timestamp (Triable + Random) - Le nouveau standard pour les DB
fn uuid_v7(_: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(Uuid::now_v7().to_string()))
}
