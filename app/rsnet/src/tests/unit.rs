use crate::calculations::*;

#[test]
fn normalizing_hosts() {
    let mut hosts: Vec<u32> = vec![2, 5, 7, 120, 253, 254, 1020, 63];
    for index in 0..hosts.len() {
        normalize_number(&mut hosts[index]);
    }
    assert_eq!(4, hosts[0]);
    assert_eq!(8, hosts[1]);
    assert_eq!(16, hosts[2]);
    assert_eq!(128, hosts[3]);
    assert_eq!(256, hosts[4]);
    assert_eq!(256, hosts[5]);
    assert_eq!(1024, hosts[6]);
    assert_eq!(128, hosts[7]);
}

#[test]
fn creating_broadcast() {
    let net_address: Address = Address::new(192, 168, 0, 0);
    let mask: Address = Address::new(255, 255, 255, 0);
    let wanted_broadcast: Address = Address::new(192, 168, 0, 255);
    let created_broadcast: Address = broadcast(net_address, &mask);
    assert_eq!(wanted_broadcast.get(), created_broadcast.get());

    let net_address: Address = Address::new(192, 168, 0, 0);
    let mask: Address = Address::new(255, 255, 255, 252);
    let wanted_broadcast: Address = Address::new(192, 168, 0, 3);
    let created_broadcast: Address = broadcast(net_address, &mask);
    assert_eq!(wanted_broadcast.get(), created_broadcast.get());
}
